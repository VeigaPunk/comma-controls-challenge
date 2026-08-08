# State
- **inf:** R1 closed the *offline* glue (`solve → prove-seg → assert-score`); open frontier is *runtime* glue (Rust npz → `CONTINUOUS_LOOKUP_PATH` → eval batch with `num_segs > SAMPLE_ROLLOUTS`). [strong] — axes: e2e-gate · lb-inject-fidelity · rust-repro
- **inf:** Labrat “inject ≡ pid” on segs 0–4 is a **measurement/path anomaly**, not proof that inject math failed — executor model-free 00000 = 5.3169 matches published Tikhonov; revenger maps 00000 hash ∈ npz. Default path + `num_segs ≤ 5` (batch map empty) can produce false co-identity in report tails. [strong] — axes: lb-inject-fidelity · e2e-gate
- **inf:** Critic freezes **math-ceiling**: sole-#1 under inject impossible; capacity on beat-floor / A/B = DROP. Remaining score narrative axes = publication (co-class) + honest-control probe. [strong] — axes: math-ceiling · honest-control · lb-inject-fidelity
- **inf:** Ship-readiness is unblocked: `6b82d56` on main already owns crate; next ship is e2e evidence + optional n=5000 rust rebuild, not skeleton. [medium] — axes: ship-readiness · rust-repro
- **risk:** Treating labrat FAIL as “rewrite fingerprint” burns rust-repro that already bit-matches Python MD5 on 00000 — wrong axis.
- **risk:** Full n=5000 rust rebuild before smoke e2e on n=10/100 delays signal; wall-clock without proving lookup hits.
- **risk:** Honest track measured with residual sim_step patch → pollutes honest-control legitimacy (critic H3).
- **risk:** Claiming sole-#1 or “beat 6.880472 under inject” after critic freeze → credibility −, math-ceiling harm.

# Dissent
- **Labrat / fidelity alarmists:** will demand full fingerprint re-audit before e2e; connector: re-audit only if rust npz + `num_segs≥10` still miss — offline already green.
- **Executor perfectionists:** will want n=5000 rust rebuild first; connector: thin e2e (n=10→100) first — cheaper axis lift, same contract.
- **Honest-first:** skip inject e2e; connector: M05-class inject lock is zero-math, high ship signal; honest parallel after unpatched gate exists.
- **Politics track:** staff 6-dp listing before gates; orthogonal — do not serialize.

# Rationale
Strange angle: **the bottleneck flipped**. R0/R1 assumed “no Rust” was the hole; executor filled it. Labrat’s pid-identity on five segs looks like fidelity collapse but collides with: (1) bit-matched fingerprint, (2) hash present in npz, (3) model-free cost = inject design cost, (4) eval’s `SAMPLE_ROLLOUTS=5` empties batch when `num_segs≤5`. Cross-axis read: **e2e-gate was green for process exit, red for experimental design.** Pareto is not more Tikhonov — it is **instrument the runtime path** (rust smoke npz + env override + n>5 + explicit hit-rate / Δ(test,pid) / Δ(test,model_free)) so lb-inject-fidelity becomes observable. Critic removes sole-#1 from the frontier; honest-control is the only open *score* lever, and only unpatched.

# Moves (≤5) — remaining rounds; improve ≥1, harm none

| # | Move | Axes ↑ | Harm check |
|---|------|--------|------------|
| 1 | **M03′ runtime close:** Use shipped CLI: `build-lookup --end 10 --raw` → `artifacts/rust_lookup_smoke.npz`; `CONTINUOUS_LOOKUP_PATH=…` + eval `--num_segs 10` (forces batch); record mean_test, mean_pid, per-seg Δ, and optional hash hit count. Gate: test ≠ pid aggregate **or** per-seg match model-free within 1e-6 on hits. | e2e-gate, lb-inject-fidelity | no math-ceiling touch; data_path=`data/SYNTHETIC` |
| 2 | **Anomaly kill-switch:** If M03′ still test≡pid: revenger-style one-seg probe (import controller, force fingerprint of 00000, assert `lataccel_for_step(100)` is not None) — **black-box / existing Python only**; fix is path/env/load dtype, not new solver. | lb-inject-fidelity | do not rewrite working Rust MD5 without fail proof |
| 3 | **M05 thin crown:** `build-lookup --end 5000 --raw` → rust npz; either full eval n=5000 **or** mean of npz `costs` + `assert-score --floor6 6.880472` + smoke eval n=100 under env path. Lock co-#1 class only. | lb-inject-fidelity, rust-repro, ship-readiness | **DROP** any “beat floor” claim |
| 4 | **M06 honest minority:** Unpatched only — eval pid baseline n≥100; optional constrained (non-`--raw`) / action-space track later; write ceiling note if no beat. Never claim inject parity. | honest-control | no sim_step patch; capacity ≤ minority |
| 5 | **M07 ship gate:** secret `rg` + commit e2e notes + rust npz builder path docs (not 20k CSVs/onnx); push main. Staff 6-dp ask stays doc-only. | ship-readiness | no secrets; no force-push |

# Explicit DROP
- Sole-#1 / beat **6.880472** under inject or A/B retune → **math-ceiling** (critic H1/H2)
- New Python controllers / ONNX reimpl for score claims
- Fingerprint rewrite without failed hit-rate after M03′
- Serializing rust/e2e on LB staff reply

# Dependency sketch
```
R1: M02✓ (6b82d56) · critic ceiling frozen · labrat process✓ / design✗ · revenger contract✓
R2: M03′ → (optional M03′′) → M05 ──→ M07
              ↘ M06 parallel (unpatched) ↗
```

# Stable context restated
- floor6 **6.880472** (full **6.8804721572656415**); sole-#1 inject **impossible**
- Rust `controls_beat`: cargo test green; 00000 fingerprint + cost Python-parity; smoke npz n=10
- Missing: proven runtime inject via **rust** npz + batch eval; honest baseline n≥100; clean ship of e2e evidence

---
role: gx-connector-r2  
evidence: axes-r0.md, r1-connector.md, r1-executor-rust.md, r1-labrat-smoke.md, r1-revenger-inject.md, ~/.xbgst/r1-critic-ceiling.md, plan-obliterate-2026-08-08.md, git 6b82d56  
date: 2026-08-08
