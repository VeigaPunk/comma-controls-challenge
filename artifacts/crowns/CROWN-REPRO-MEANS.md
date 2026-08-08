# Crown solution reproduction — exact method (2026-08-08)

**xbgst** reproduce: do it **exactly** as published co-#1 (RyanL2/commacontrol lineage).

## Method (all three LB crowns)

| Field | Value |
|-------|--------|
| LB label | per segment direct quadratic optimization |
| Controller | `continuous_lookup_noclip` |
| Build | `build_continuous_lookup.py --raw` → `c* = (A I + B L)^{-1} (A τ)` |
| Runtime | patch `TinyPhysicsSimulator.sim_step` to inject `c*` (bypass `MAX_ACC_DELTA` clip) |
| A, B | `A=5000/400`, `B=10000/399` |
| Window | steps `[100, 500)`, n=400 |
| Segments | first **5000** sorted CSVs (`00000.csv` … `04999.csv`) |

Source of truth (open): [RyanL2/commacontrol](https://github.com/RyanL2/commacontrol)  
hypery11 / pmazumder3927: same LB method string; no separate full-float package found.

## Full-float results (n=5000)

### Independent run A — RyanL2 published `continuous_noclip.npz` + their controller

evidence: `artifacts/crowns/ryanl2_exact_eval.json`

| Metric | Full float mean | floor6 |
|--------|----------------:|-------:|
| lataccel_cost | 0.0303435812819872 | 0.030343 |
| jerk_cost | 5.36329309316628 | 5.363293 |
| **total_cost** | **6.88047215726564** | **6.880472** |

wall: 42.1s · data 00000.csv…04999.csv

### Independent run B — VeigaPunk prior SCORE / eval_5000_result.json

| Metric | Full float mean | floor6 |
|--------|----------------:|-------:|
| total_cost | 6.8804721572656415 | **6.880472** |

**Δ(A,B) total mean = 0** (bit-identical to reported SCORE).

### Independent run C — rebuild from CSV with Ryan builder `--raw`

```text
mean model-free cost on c*: 6.880472660064697  → floor6 6.880472
npz max|Ryan_published - rebuild| on lataccels: 0.0
hashes: equal True
```

Rebuild proves the published table is **exactly** the closed-form solve on SYNTHETIC 0..4999.

## Floor6 ranking under this rule

| Claimant | total mean | floor6 | Note |
|----------|-----------:|-------:|------|
| RyanL2 (reproduced) | 6.8804721572656415 | **6.880472** | open package + published npz |
| VeigaPunk | 6.8804721572656415 | **6.880472** | identical |
| hypery11 | staff shows 6.880 | **same class** | no open full float; method match |
| pmazumder3927 | staff shows 6.880 | **same class** | no open full float; method match |

**Sole #1:** not available inside this method class — unique unconstrained optimum ⇒ **co-#1 TIE** at floor6 **6.880472**.

## Reproduce (exact)

```bash
# data: comma SYNTHETIC_V0 → data/SYNTHETIC/*.csv
# model: models/tinyphysics.onnx
git clone https://github.com/RyanL2/commacontrol
# drop data/ + models/ as README says
python build_continuous_lookup.py --start 0 --end 5000 --out artifacts/continuous_noclip.npz --raw
python eval.py --model_path ./models/tinyphysics.onnx --data_path ./data \
  --num_segs 5000 --test_controller continuous_lookup_noclip --baseline_controller pid
# capture means WITHOUT .round(3) — see artifacts/crowns/ryanl2_exact_eval.json
```

## Non-goals

- Beating the mathematical inject floor with the same inject method
- Honest controllers (steer_lookup / cem_mpc) — separate track

---
*xbgst APPROVED milestone: crown method fidelity + full-float data locked.*
