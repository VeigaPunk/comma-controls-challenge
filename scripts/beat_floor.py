#!/usr/bin/env python3
"""Push for sole #1: re-solve unconstrained Tikhonov at max precision.

The shared co-#1 score (~6.880) is the unconstrained convex floor of the
scored-window cost. This script:
  1) re-derives A,B from official multipliers
  2) solves with float64 np.linalg.solve (no inv cache error)
  3) optional L-BFGS polish
  4) reports mean cost + floor6 display

Usage (uv):
  uv run python scripts/beat_floor.py --data-dir data --end 5000 --workers 16
"""
from __future__ import annotations

import argparse
import json
import math
import time
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

import numpy as np
import pandas as pd

N = 400  # CONTROL_START=100 .. COST_END=500
# Official total_cost pieces (tinyphysics.py):
# lat_accel_cost = mean((c-tau)^2)*100
# jerk_cost      = mean((diff(c)/0.1)^2)*100
# total          = 50*lat + jerk
# => total = (5000/N)*sum((c-tau)^2) + (10000/(N-1))*sum(diff(c)^2)
A = 5000.0 / N
B = 10000.0 / (N - 1)


def floor6(x: float) -> float:
    return math.floor(float(x) * 1_000_000) / 1_000_000


def build_system() -> tuple[np.ndarray, np.ndarray]:
    """Return (factor, L) for solve: (A I + B L) c = A tau."""
    L = np.zeros((N, N), dtype=np.float64)
    for k in range(N - 1):
        L[k, k] += 1.0
        L[k + 1, k + 1] += 1.0
        L[k, k + 1] -= 1.0
        L[k + 1, k] -= 1.0
    M = A * np.eye(N, dtype=np.float64) + B * L
    # Cholesky for repeated RHS (SPD)
    chol = np.linalg.cholesky(M)
    return chol, L


_CHOL: np.ndarray | None = None


def _init(chol: np.ndarray) -> None:
    global _CHOL
    _CHOL = chol


def cost(c: np.ndarray, tau: np.ndarray) -> float:
    return float(A * np.sum((c - tau) ** 2) + B * np.sum(np.diff(c) ** 2))


def solve_tau(tau: np.ndarray) -> np.ndarray:
    """c* = M^{-1} (A tau) via Cholesky."""
    assert _CHOL is not None
    rhs = A * tau.astype(np.float64)
    # solve L y = rhs, L^T c = y
    y = np.linalg.solve(_CHOL, rhs)
    c = np.linalg.solve(_CHOL.T, y)
    return c


def polish(c: np.ndarray, tau: np.ndarray) -> np.ndarray:
    """Unconstrained L-BFGS polish (should stay at closed form)."""
    from scipy.optimize import minimize

    def fg(x: np.ndarray):
        r = x - tau
        d = np.diff(x)
        f = A * np.sum(r**2) + B * np.sum(d**2)
        g = 2 * A * r
        # gradient of sum (x[i+1]-x[i])^2
        g[:-1] += 2 * B * (x[:-1] - x[1:])
        g[1:] += 2 * B * (x[1:] - x[:-1])
        return f, g

    res = minimize(fg, c, jac=True, method="L-BFGS-B", options={"ftol": 1e-18, "gtol": 1e-14, "maxiter": 500})
    return res.x


def _one(job: tuple[int, str, bool]) -> tuple[int, float, float]:
    idx, path, do_polish = job
    full = pd.read_csv(path)["targetLateralAcceleration"].to_numpy(dtype=np.float64)
    tau = full[100:500]
    assert len(tau) == N
    c = solve_tau(tau)
    base = cost(c, tau)
    if do_polish:
        c2 = polish(c, tau)
        pol = cost(c2, tau)
        return idx, base, pol
    return idx, base, base


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--data-dir", default="data")
    ap.add_argument("--start", type=int, default=0)
    ap.add_argument("--end", type=int, default=5000)
    ap.add_argument("--workers", type=int, default=16)
    ap.add_argument("--polish", action="store_true")
    ap.add_argument("--out", default="artifacts/beat_floor_report.json")
    args = ap.parse_args()

    root = Path(args.data_dir)
    csvs = sorted(root.rglob("*.csv"))
    if not csvs:
        # flat
        csvs = sorted(p for p in root.iterdir() if p.suffix == ".csv") if root.is_dir() else []
    if not csvs:
        raise SystemExit(f"no csv in {root} — download SYNTHETIC_V0 first")

    selected = csvs[args.start : args.end]
    print(f"segments={len(selected)} polish={args.polish} workers={args.workers}")
    print(f"A={A:.16g} B={B:.16g}")

    chol, _ = build_system()
    jobs = [(i, str(p), args.polish) for i, p in enumerate(selected)]

    t0 = time.perf_counter()
    costs_base: list[float] = []
    costs_pol: list[float] = []
    with ProcessPoolExecutor(max_workers=args.workers, initializer=_init, initargs=(chol,)) as ex:
        for idx, base, pol in ex.map(_one, jobs, chunksize=8):
            costs_base.append(base)
            costs_pol.append(pol)
            if (idx + 1) % 500 == 0:
                print(f"  {idx+1}/{len(jobs)} mean_base={np.mean(costs_base):.12f}")

    mean_b = float(np.mean(costs_base))
    mean_p = float(np.mean(costs_pol))
    report = {
        "n": len(costs_base),
        "A": A,
        "B": B,
        "mean_closed_form": mean_b,
        "mean_polish": mean_p,
        "floor6_closed": floor6(mean_b),
        "floor6_polish": floor6(mean_p),
        "min_seg": float(np.min(costs_base)),
        "max_seg": float(np.max(costs_base)),
        "beats_6880472": floor6(mean_p) < 6.880472,
        "beats_published_3dp_crown": floor6(mean_p) < 6.880000,
        "wall_s": time.perf_counter() - t0,
        "note": "Unconstrained Tikhonov is the unique convex minimizer; sole #1 needs mean < co-#1 float.",
    }
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
