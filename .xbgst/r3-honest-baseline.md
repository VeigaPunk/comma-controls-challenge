# r3 honest baseline — pid n=100 vs inject (isolated)

agent: gx-labrat-honest-m06  
date: 2026-08-08  
axes: [honest-control, math-ceiling]  
scope: `/home/vgpnk1337/Projects/comma-controls-challenge`

## Controllers inventory

| name | honest? | notes |
|------|---------|-------|
| **pid** | **yes** | stock `controllers/pid.py` — only non-patch controller present |
| **continuous_lookup_noclip** | **no** | patches `TinyPhysicsSimulator.sim_step`; injects c* |
| continuous_lookup (no noclip) | **absent** | not in `controllers/` — do not invent this round |

Language lock (rust-only): no new Python controllers written. Math ceiling remains inject/offline τ solve.

## Measurement protocol (isolation)

Noclip pollutes **class-level** `sim_step` + `_ACTIVE_CONTROLLER` (see `.xbgst/r2-labrat-inject.md`).  
**Never** run inject then pid in one process without restore.

This round: **two separate Python process trees**, each:

- `process_map` over first **n=100** `data/SYNTHETIC` segs  
- **one** controller type only  
- `max_workers=8`, model `./models/tinyphysics.onnx`  
- means = arithmetic mean of per-seg costs (full float; floor-6 display noted)

Evidence artifacts:

- `artifacts/honest_pid_n100.npz`
- `artifacts/inject_noclip_n100.npz`

## Results n=100 (isolated)

| controller | mean lataccel | mean jerk | **mean total** | floor→6dp total |
|------------|--------------:|----------:|---------------:|----------------:|
| **pid** (honest) | 1.233484496 | 20.365205547 | **82.039430353** | **82.039430** |
| **continuous_lookup_noclip** | 0.021382218 | 3.740935686 | **4.810046582** | **4.810046** |

| ratio / gap | value |
|-------------|------:|
| inject / pid (total) | **0.058631** |
| pid − inject (total) | **+77.229384** |
| pid min / max total | 5.347082 / 368.406760 |
| inject min / max total | 0.008304 / 36.965917 |

### Spot-check first segs (match R2 single-seg isolation)

| seg | pid total | inject total |
|-----|----------:|-------------:|
| 00000 | 80.881924 | 5.316901 |
| 00001 | 30.922612 | 0.603703 |
| 00002 | 58.676444 | 3.058772 |

### Prior context (not re-run here)

| set | pid mean total | inject mean total |
|-----|---------------:|------------------:|
| n=20 isolated (R2) | ~72.71 | ~3.76 |
| n=5000 SCORE.md inject | — | **6.880472** (floor 6.880472) |
| Critic next LB band | — | ~7.083 (honest LB; inject crowns ~6.880) |

n=100 inject (~4.81) is **easier** than full 5000 (~6.88); n=100 pid (~82) is harder than n=20 pid (~72.7). Easy-seg bias on short windows remains.

## Claim discipline

- **Do not claim beat inject.** Inject is harness patch, not a fair controller.
- Honest control = pid only in-tree.
- Sole-#1 on public LB via honest physics-respecting controller: **Status: blocked** this round (language lock + only pid available; gap to inject floor ~17× on n=100 totals; full-5000 honest path not measured beyond pid).

## Status: blocked (sole-#1 honest)

**Blocked** for sole-#1 / honest co-#1 without inject:

1. No `continuous_lookup` **without** noclip in repo.  
2. Only honest controller is **pid** (~82 mean total @ n=100 vs inject ~4.81; full-5000 inject floor **6.880472**).  
3. Language lock: **no new Python controllers** this round; pure-Rust offline score on τ not re-derived here (prior crown is inject path).

### Next steps (honest improvement, no new Python this round)

1. **Rust offline τ score only** (optional): recompute unconstrained quadratic mean on first 100/5000 segs offline — documents math ceiling without claiming live honest control.  
2. **Future honest controller (when unlock):** physics-respecting update() only — rate-limited lataccel; target band ~7.083 LB, not 6.880 inject.  
3. **Eval hygiene permanent:** always isolate processes when any noclip controller is on PATH; never trust dual-controller `eval.py` sample loop for baseline means.  
4. Keep inject results labeled **exploit / math floor**, not honest baseline.

## Axes

| axis | move | outcome |
|------|------|---------|
| honest-control | isolated pid n=100 | mean total **82.039430** (evidence npz) |
| math-ceiling | isolated inject n=100 | mean total **4.810047**; ratio 0.059× pid; full-5000 inject still **6.880472** |

## Verdict

Honest baseline measured. Inject ratio documented. **No continuous_lookup without noclip.**  
**Status: blocked** for sole-#1 via honest controller under current language lock.  
**Do not claim beat inject.**
