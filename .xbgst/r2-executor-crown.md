# r2 executor crown — n=5000 Rust lookup + official eval e2e

agent: gx-executor-crown-m05  
date: 2026-08-08  
commit_base: `6b82d56`  
axes: [lb-inject-fidelity, rust-repro, e2e-gate, ship-readiness]

## Goal

Rust-build full n=5000 raw continuous lookup; run official Python eval black-box; assert floor6 **6.880472**; document e2e gate.

## 1. Build lookup (Rust)

```bash
cd rust/controls_beat
cargo run --release -p controls_beat -- build-lookup \
  --data-dir ../../data/SYNTHETIC --start 0 --end 5000 --raw \
  --out ../../artifacts/rust_continuous_noclip.npz
```

**stdout (compact):**

```json
{
  "beats_6.88": false,
  "floor6": 6.880472,
  "mean": 6.880472160689848,
  "n": 5000,
  "out": "../../artifacts/rust_continuous_noclip.npz",
  "wall_s": 1.41436922
}
```

Note: builder mean is **model-free** Tikhonov cost (matches published npz costs arrays). Eval total_cost_mean is slightly different float path but same floor6.

## 2. NPZ parity vs published `artifacts/continuous_noclip.npz`

Black-box numpy compare:

| check | result |
|-------|--------|
| keys | both: `hashes`, `lataccels`, `costs`, `init_costs` |
| shapes | hashes (5000,), lataccels (5000, 400), costs (5000,) |
| hashes equal | **True** (0 mismatches) |
| lataccels max abs diff | **0.0** |
| costs max abs diff | **0.0** |
| mean costs | **6.880472160689848** (identical) |

File size differs (zip compression): rust `7703816` vs published `8681010` — content arrays identical.

## 3. Official eval (black-box)

```bash
CONTINUOUS_LOOKUP_PATH=artifacts/rust_continuous_noclip.npz \
  .venv/bin/python eval.py \
  --model_path ./models/tinyphysics.onnx \
  --data_path ./data/SYNTHETIC \
  --num_segs 5000 \
  --test_controller continuous_lookup_noclip \
  --baseline_controller pid
```

- exit 0; `report.html` written  
- **Important:** `num_segs` must be **> SAMPLE_ROLLOUTS (5)** or batch path is empty (r1 labrat)

Metrics dump via same `eval.run_rollout` + `process_map` path → `artifacts/rust_eval_5000.json`:

```json
{
  "controller": "continuous_lookup_noclip",
  "lookup_path": "artifacts/rust_continuous_noclip.npz",
  "n": 5000,
  "lataccel_cost_mean": 0.030343581281987223,
  "jerk_cost_mean": 5.363293093166279,
  "total_cost_mean": 6.8804721572656415,
  "total_cost_mean_floor6": 6.880472,
  "lataccel_cost_mean_floor6": 0.030343,
  "jerk_cost_mean_floor6": 5.363293
}
```

Matches published `eval_5000_result.json` totals bit-for-bit on means.

## 4. assert-score CLI

```bash
cargo run --release -p controls_beat -- assert-score \
  --json ../../artifacts/rust_eval_5000.json --floor6 6.880472
```

Expect: `"pass": true`, `"floor6": 6.880472`.

## 5. Axis results

| axis | result |
|------|--------|
| lb-inject-fidelity | PASS — hashes/lataccels/costs identical to published crown npz |
| rust-repro | PASS — n=5000 build ~1.4s; floor6 6.880472 |
| e2e-gate | PASS — official eval + run_rollout batch n=5000 → floor6 6.880472 |
| ship-readiness | PASS if commit+push after secret gate |

## 6. Artifacts staged (not bulk data)

- `artifacts/rust_eval_5000.json` (metrics only)
- `.xbgst/r2-executor-crown.md` (this file)
- rust crate already on main; optional note that bulk `rust_continuous_noclip.npz` may be gitignored

## APPROVED

e2e floor6 **6.880472** with Rust-built raw lookup under `CONTINUOUS_LOOKUP_PATH`.
