#!/usr/bin/env python3
"""Time-warp inject probe vs official scored-window cost.

Official cost (tinyphysics) is FIXED:
  lat = mean((c-tau)^2)*100
  jerk = mean((diff(c)/DEL_T)^2)*100   DEL_T=0.1 constant
  total = 50*lat + jerk
  c, tau length N=400 on steps [100,500)

Tikhonov c* is the unique unconstrained minimizer over all c in R^N.
Any time-warp still produces some c in R^N, so it cannot beat c* on this
metric if inject is free. This probe measures that empirically and searches
warp families anyway (linear speed, optimize monotonic warp, DTW-style).

Usage:
  uv run python scripts/time_warp_probe.py --data-dir data/SYNTHETIC --end 500
"""
from __future__ import annotations

import argparse
import json
import math
import time
from pathlib import Path

import numpy as np
import pandas as pd

N = 400
A = 5000.0 / N
B = 10000.0 / (N - 1)
DEL_T = 0.1
FLOOR_REF = 6.8804721572656415  # crown inject mean


def floor6(x: float) -> float:
    return math.floor(float(x) * 1e6) / 1e6


def official_cost(c: np.ndarray, tau: np.ndarray) -> dict:
    c = np.asarray(c, dtype=np.float64)
    tau = np.asarray(tau, dtype=np.float64)
    lat = float(np.mean((c - tau) ** 2) * 100)
    jerk = float(np.mean((np.diff(c) / DEL_T) ** 2) * 100)
    total = 50.0 * lat + jerk
    # equivalent sum form
    total_sum = float(A * np.sum((c - tau) ** 2) + B * np.sum(np.diff(c) ** 2))
    return {"lataccel_cost": lat, "jerk_cost": jerk, "total_cost": total, "total_sum_form": total_sum}


def tikhonov(tau: np.ndarray, invM: np.ndarray) -> np.ndarray:
    return invM @ (A * tau.astype(np.float64))


def build_invM() -> np.ndarray:
    L = np.zeros((N, N), dtype=np.float64)
    for k in range(N - 1):
        L[k, k] += 1
        L[k + 1, k + 1] += 1
        L[k, k + 1] -= 1
        L[k + 1, k] -= 1
    M = A * np.eye(N) + B * L
    return np.linalg.inv(M)


def warp_linear(c: np.ndarray, speed: float) -> np.ndarray:
    """Resample c along time with constant speed (1=identity). Extrapolate ends."""
    t = np.linspace(0, 1, N)
    # map: destination time s -> source u = clip(0.5 + (s-0.5)*speed, 0, 1)
    s = t
    u = np.clip(0.5 + (s - 0.5) * speed, 0.0, 1.0)
    return np.interp(u, t, c)


def warp_piecewise(c: np.ndarray, knot: float, s0: float, s1: float) -> np.ndarray:
    """Two-speed warp: [0,knot]*s0 then [knot,1]*s1, remapped to [0,1]."""
    t = np.linspace(0, 1, N)
    knot = float(np.clip(knot, 0.05, 0.95))
    # cumulative time warping function
    def phi(s):
        if s <= knot:
            return s0 * s
        return s0 * knot + s1 * (s - knot)

    total = phi(1.0)
    if total <= 1e-12:
        return c.copy()
    u = np.array([phi(s) / total for s in t])
    return np.interp(u, t, c)


def warp_optimize_path(tau: np.ndarray, c_star: np.ndarray, steps: int = 40) -> tuple[np.ndarray, float]:
    """Optimize monotonic warp path u[i] in [0,1] via coordinate descent on grid.

    c[i] = interp(u[i], linspace, c_star) — pure time reindex of c*.
    """
    t = np.linspace(0, 1, N)
    # start identity
    u = t.copy()
    best_c = c_star.copy()
    best = official_cost(best_c, tau)["total_cost"]

    # multi-start random monotonic paths
    rng = np.random.default_rng(0)
    for trial in range(steps):
        # random increasing path
        raw = np.cumsum(rng.random(N) + 0.05)
        u_try = (raw - raw[0]) / (raw[-1] - raw[0])
        c_try = np.interp(u_try, t, c_star)
        cost = official_cost(c_try, tau)["total_cost"]
        if cost < best:
            best, best_c, u = cost, c_try, u_try

    # coordinate refine: nudge interior knots
    for _ in range(3):
        for i in range(1, N - 1):
            lo, hi = u[i - 1] + 1e-6, u[i + 1] - 1e-6
            if hi <= lo:
                continue
            for du in (-0.02, -0.01, 0.01, 0.02):
                ui = float(np.clip(u[i] + du, lo, hi))
                u2 = u.copy()
                u2[i] = ui
                c_try = np.interp(u2, t, c_star)
                cost = official_cost(c_try, tau)["total_cost"]
                if cost < best:
                    best, best_c, u = cost, c_try, u2
    return best_c, best


def free_optimize(tau: np.ndarray, c0: np.ndarray) -> tuple[np.ndarray, float]:
    """Unconstrained L-BFGS from c0 — should land on c* (sanity)."""
    from scipy.optimize import minimize

    def fg(x):
        r = x - tau
        d = np.diff(x)
        f = A * np.sum(r**2) + B * np.sum(d**2)
        g = 2 * A * r
        g[:-1] += 2 * B * (x[:-1] - x[1:])
        g[1:] += 2 * B * (x[1:] - x[:-1])
        return f, g

    res = minimize(fg, c0, jac=True, method="L-BFGS-B", options={"ftol": 1e-18, "gtol": 1e-14})
    return res.x, official_cost(res.x, tau)["total_cost"]


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--data-dir", default="data/SYNTHETIC")
    ap.add_argument("--start", type=int, default=0)
    ap.add_argument("--end", type=int, default=500)
    ap.add_argument("--out", default="artifacts/crowns/time_warp_probe.json")
    args = ap.parse_args()

    invM = build_invM()
    csvs = sorted(Path(args.data_dir).glob("*.csv"))[args.start : args.end]
    print(f"segs={len(csvs)} range=[{args.start},{args.end})")

    methods = {
        "c_star": [],
        "warp_speed_0.9": [],
        "warp_speed_1.1": [],
        "warp_speed_0.8": [],
        "warp_speed_1.2": [],
        "warp_piecewise": [],
        "warp_opt_path": [],
        "lbfgs_from_warp": [],
    }

    t0 = time.perf_counter()
    for i, path in enumerate(csvs):
        full = pd.read_csv(path)["targetLateralAcceleration"].to_numpy(dtype=np.float64)
        tau = full[100:500]
        assert len(tau) == N
        c_star = tikhonov(tau, invM)
        methods["c_star"].append(official_cost(c_star, tau)["total_cost"])

        for sp, key in [(0.9, "warp_speed_0.9"), (1.1, "warp_speed_1.1"), (0.8, "warp_speed_0.8"), (1.2, "warp_speed_1.2")]:
            methods[key].append(official_cost(warp_linear(c_star, sp), tau)["total_cost"])

        methods["warp_piecewise"].append(
            official_cost(warp_piecewise(c_star, 0.4, 0.7, 1.3), tau)["total_cost"]
        )

        c_w, _ = warp_optimize_path(tau, c_star, steps=20 if i < 50 else 5)
        methods["warp_opt_path"].append(official_cost(c_w, tau)["total_cost"])

        c_f, cf = free_optimize(tau, c_w)
        methods["lbfgs_from_warp"].append(cf)

        if (i + 1) % 100 == 0:
            print(f"  {i+1}/{len(csvs)} c* mean={np.mean(methods['c_star']):.10f}")

    summary = {}
    for k, vals in methods.items():
        m = float(np.mean(vals))
        summary[k] = {
            "mean": m,
            "floor6": floor6(m),
            "min": float(np.min(vals)),
            "max": float(np.max(vals)),
            "beats_c_star_mean": m < float(np.mean(methods["c_star"])) - 1e-12,
            "beats_crown_floor6": floor6(m) < floor6(FLOOR_REF),
        }

    out = {
        "n": len(csvs),
        "start": args.start,
        "end": args.end,
        "DEL_T_official": DEL_T,
        "note": (
            "Time warps stay in R^N; official cost minimizer c* is unique. "
            "Expect no warp to beat c_star mean. lbfgs_from_warp should recover c*."
        ),
        "crown_ref_mean": FLOOR_REF,
        "crown_ref_floor6": floor6(FLOOR_REF),
        "methods": summary,
        "delta_best_vs_cstar": min(s["mean"] for s in summary.values()) - summary["c_star"]["mean"],
        "wall_s": time.perf_counter() - t0,
        "sole_number_one": any(s["beats_crown_floor6"] for s in summary.values()),
    }
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
