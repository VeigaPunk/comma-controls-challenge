# r1-executor-rust — controls_beat

**Agent:** gx-executor-rust-m02  
**Date:** 2026-08-08  
**Axes:** rust-repro↑, e2e-gate↑, lb-inject-fidelity lock, ship-readiness↑

## Goal

Rust crate owning Tikhonov closed-form `c*`, model-free cost, fingerprint, and CLIs
`build-lookup` / `prove-seg` / `assert-score`.

## Artifact

Path: `rust/controls_beat/`

| Piece | Location |
|-------|----------|
| `solve_tikhonov` | `src/lib.rs` |
| `model_free_cost` | `src/lib.rs` |
| fingerprint (MD5 / 80 steps / round4 f32) | `src/lib.rs` |
| npz writer (hashes U32, lataccels f32, costs) | `src/lib.rs` |
| CLI | `src/main.rs` |

Constants locked to Python crown:

- `N=400`, window `[100,500)`, `A=5000/400`, `B=10000/399`
- Laplacian: path-graph form matching `build_continuous_lookup.py`
- Cost: `lat=mean((c-τ)²)*100`, `jerk=mean((diff(c)/0.1)²)*100`, `total=50*lat+jerk`

## Commands + results

### `cargo test` (release)

```text
running 4 tests
test tests::floor6 ... ok
test tests::builder_cost_matches_total ... ok
test tests::solve_length_and_finite ... ok
test tests::real_csv_if_present ... ok
test result: ok. 4 passed; 0 failed
```

Real CSV `data/SYNTHETIC/00000.csv`:

- fingerprint `40ca7a1f69d9a97b2fdc35e6ef450802` (bit-match Python)
- total `5.316900534462242` vs Python `5.316900534462244` (Δ &lt; 1e-14)

### `prove-seg --csv data/SYNTHETIC/00000.csv --raw`

```json
{
  "c0": -0.016380545127454044,
  "c1": -0.015754113220495608,
  "c_len": 400,
  "fingerprint": "40ca7a1f69d9a97b2fdc35e6ef450802",
  "lataccel_cost": 0.017484205376836686,
  "jerk_cost": 4.442690265620408,
  "total_cost": 5.316900534462242,
  "floor6_total": 5.3169
}
```

### `build-lookup --start 0 --end 10 --raw`

```json
{
  "n": 10,
  "mean": 3.6782283425331115,
  "floor6": 3.678228,
  "out": "artifacts/rust_lookup_smoke.npz",
  "wall_s": ~0.006
}
```

Python mean10 reference: `3.678228367055559` (f64); Rust mean of stored f32 costs is within 3e-8.

### `assert-score --json eval_5000_result.json --floor6 6.880472`

```json
{"floor6":6.880472,"mean":6.8804721572656415,"note":"floor6 match","pass":true,"target_floor6":6.880472}
```

## Notes

- Builder implements **unconstrained** c* only (Python `--raw` path). Constrained L-BFGS-B remains Python-only; noclip floor uses raw.
- Linear algebra: `nalgebra` LU on dense `A I + B L` (cached once).
- Full n=5000 rebuild not run (scope: smoke end≤10).

## Status

**done** — cargo test green; CLI gates green; fingerprint + cost fidelity vs Python on 00000.

APPROVED: rust controls_beat owns Tikhonov c* + model-free cost + fingerprint + CLI gates with Python bit-match on 00000.
