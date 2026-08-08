# AXES — obliterate controls_challenge
**Date:** 2026-08-08 | **Judge:** xbgst

| # | Axis | Direction | Observable | Baseline |
|---|------|-----------|------------|----------|
| 1 | lb-inject-fidelity | lock floor6 **6.880472** | official eval n=5000 floor6 | 6.880472 claimed |
| 2 | rust-repro | ↑ | `cargo test` + rust lookup builder green | 0 Rust crates |
| 3 | e2e-gate | ↑ | CONTINUOUS_LOOKUP_PATH → python eval smoke | Python-only |
| 4 | honest-control | ↓ total_cost unpatched | eval n≥100 vs pid, no sim_step patch | unknown |
| 5 | ship-readiness | ↑ | secrets clean + commit + push main | docs present, no rust ship |
| 6 | math-ceiling | lock | sole-#1 under inject = impossible (convex floor) | co-#1 tie documented |

Pareto rule: improve ≥1, harm none. Inject sole-#1 moves that claim beat-floor without math → DROP.
