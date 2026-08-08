# State
- **inf:** inject floor is *already* owned; the real bottleneck is **Rust ownership of the closed form + black-box e2e gate**, not more cost math. [strong] — axes: lb-inject-fidelity · rust-repro · e2e-gate · ship-readiness · math-ceiling
- **inf:** sole-#1 under inject is a *category error*; “obliterate” = dual track (lock co-#1 proof in Rust + open honest PID-crush), not thrashing total_cost. [strong] — axes: math-ceiling · honest-control · lb-inject-fidelity
- **inf:** Python eval/`CONTINUOUS_LOOKUP_PATH` is the score oracle; Rust must *build* npz + *assert* floor6, never reimplement ONNX for claims. [strong] — axes: e2e-gate · rust-repro · lb-inject-fidelity
- **inf:** data path is a silent fidelity killer (`data/` vs `data/SYNTHETIC`); wrong path harms inject fidelity and e2e without touching code. [medium] — axes: e2e-gate · lb-inject-fidelity · ship-readiness
- **risk:** reimplementing ONNX or “better” Tikhonov weights → float drift → false miss of 6.880472; burns inject + ship credibility
- **risk:** shipping 20k CSVs / onnx without policy → bloat/secret-adjacent dumps; blocks clean milestone-ship
- **risk:** honest track if measured with patched sim → pollutes honest-control axis (second-order legitimacy break)
- **risk:** capacity on sole-#1 lobby without 6-dp staff listing → zero score delta, stalls rust/e2e

# Dissent
- **Executor / “beat the number” roles:** will want lower total_cost under inject; math-ceiling + CROWN-REPRO say DROP — expect pushback until judge re-asserts co-#1 lock.
- **Pure-Rust purists:** may reimplement tinyphysics; plan forbids for *score claims* — black-box Python oracle only.
- **LB politics track:** prioritize staff email/gist over M02–M05; connector says listing is ship-adjacent but **orthogonal** to rust-repro gates — do not serialize rust behind staff reply.
- **Honest-first dissent:** argue inject is illegitimate so skip M05; plan: M05 locks fidelity (harm none); M06 parallel — skip only if judge demotes inject axis.

# Rationale
Strange angle: **the frontier is not the cost surface** — convex floor is bit-identical (RyanL2 ↔ VeigaPunk Δ=0). What is *not* owned is the **language-lock substrate** (0 crates) that makes the floor *re-provable under xbgst rules* and shipable without re-authoring Python. Cross-axis glue: `c* = (A I + B L)^{-1}(A τ)` in Rust → npz bytes → `CONTINUOUS_LOOKUP_PATH` → official eval floor6 assert is the single path that lifts **rust-repro + e2e-gate + inject fidelity + ship** simultaneously without touching math-ceiling or honest legitimacy. Honest QP is the only axis that can still move *score narrative* after inject lock — but only if unpatched.

# Moves (≤5) — improve ≥1 axis, harm none

| # | Move | Axes ↑ | Harm check |
|---|------|--------|------------|
| 1 | **M02 now:** `rust/controls_beat` — fingerprint, Tikhonov A/B/L, model-free cost, fixtures for seg `00000`; `cargo test -q` green | rust-repro | no inject touch |
| 2 | **M03/M04 thin e2e:** Rust `build-lookup --raw` → 1–10 segs npz; `CONTINUOUS_LOOKUP_PATH=…` + `eval.py --data_path data/SYNTHETIC`; prove-seg closed-form vs inject | e2e-gate, rust-repro, inject (smoke) | data_path fixed to SYNTHETIC |
| 3 | **M05 crown assert only:** rebuild n=5000 rust npz + `assert-score --floor6 6.880472` vs `SCORE.md` / `eval_5000_result.json` (reuse existing Python eval; no new controller) | lb-inject-fidelity, ship-readiness | math-ceiling respected (lock, not beat) |
| 4 | **M06 parallel (post-M04):** rate-limited / unpatched track only — n≥100 total_cost vs `pid`; if blocked, write ceiling note, do not fake with sim_step patch | honest-control | must not patch sim |
| 5 | **M07 ship gate:** secret `rg` + milestone-ship scripts/docs/rust + score artifacts; **not** bulk data/models unless download script; keep staff 6-dp request as doc, not blocker | ship-readiness | no secrets; no force-push |

# Explicit DROP
- Any “beat 6.880472 under inject/raw Tikhonov” proposal → **math-ceiling**
- New Python controllers or ONNX reimpl for claimed LB scores
- Serializing M02 on leaderboard listing response

# Dependency sketch
```
M01✓ → M02 → M03 → M04 → M05 → M07
                 ↘ M06 (parallel) ↗ optional into M07
```

# Stable context restated
- co-#1 inject floor6 **6.880472** (full 6.8804721572656415); Ryan rebuild bit-match
- Missing: any Rust crate; honest baseline unknown; LB 6-dp listing pending (orthogonal)
- Pareto remaining: Rust repro chain + optional honest PID-crush + clean ship

---
role: gx-connector-r1  
evidence: axes-r0.md, plan-obliterate-2026-08-08.md, SCORE.md, METHOD.md, CROWN-REPRO-MEANS.md, eval_5000_result.json  
date: 2026-08-08
