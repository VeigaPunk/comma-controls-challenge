# Time-warp probe — can we bend time to sole #1?

**Date:** 2026-08-08  
**Official metric:** fixed `DEL_T=0.1`, window 400 steps, inject free ⇒ unique Tikhonov minimizer `c*`.

## What we tried

| Warp / bend | Idea | Result vs `c*` (official cost) |
|-------------|------|--------------------------------|
| Linear speed 0.7–1.3 | compress/stretch trajectory in time | **Much worse** (mean cost hundreds on 1000 segs) |
| Piecewise 2-speed | different early/late tempo | **Worse** |
| Optimized monotonic path warp | DTW-style reindex of `c*` | **Does not beat** `c*`; L-BFGS recovers `c*` |
| ±1 step time shift | delay/advance inject | **Worse** (~+3.3 mean on 1000) |
| Track τ raw | zero tracking error | **Worse** (jerk explosion) |
| Fake `DEL_T=0.09/0.11` | rescore with wrong clock | **Not legal** — staff use 0.1 |

## Full 5000 (model-free official cost)

See `time_warp_5000_light.json` — only `c*` wins among legal warps.

## Verdict

| Question | Answer |
|----------|--------|
| Did time warping get sole #1? | **No** |
| Can any reindex of `c*` beat `c*` under official cost? | **No** (unique unconstrained min over ℝ⁴⁰⁰) |
| Does “bending DEL_T” help on LB? | **No** — evaluator hardcodes 0.1 |
| Sole #1 path | Different **legal** method class below inject floor, or staff rule change — not time warp |

**Honest claim remains:** floor6 **6.880472** co-#1 class with RyanL2 inject method.
