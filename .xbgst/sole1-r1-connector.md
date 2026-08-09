# State
- **inf:** Official n=5000 mean **≡** model-free cost of **f32-promoted** `lataccels` (mean **6.8804721572656415** bit-match). The ~5e−7 “official < rebuild” gap is **`costs` f32 array + f32 reduction**, not inject free juice. [strong] — axes: float-site-fidelity, ulp-mean-gap, math-ceiling-reopen
- **inf:** Continuous Tikhonov c* unique; f64-solve vs nearest-f32 promote mean gap ≈ **3e−12** (n=500 sample); max per-seg quantization gap ~**5e−11**. Floor6 sole needs **Δmean ≤ −1.57e−7**; full-float sole needs **≲ −1e−8**. Float lattice cannot close either under quadratic curvature. [strong] — axes: ulp-mean-gap, floor6-display, math-ceiling-reopen
- **inf:** Sites F1–F7 are locked: f64 solve → f32 store → f64 load → `float()` inject → f64 `compute_cost` → pandas mean → floor6 **display only**. Ranking mean is f64 path; F8 cannot sole. [strong] — axes: float-site-fidelity, claim-ship
- **risk:** Treating npz `costs.mean()` as oracle → false “gap” narrative and wasted M02 on non-eval scalar.
- **risk:** Claiming sole from model-free builder alone without official eval JSON → claim-ship credibility kill.
- **risk:** Reopening continuous A/B / time-warp after TIME_WARP_RESULT null → axis thrash, zero Δ.

# Dissent
- **Planner optimism (sole via float):** expects ≥1e−8 from ULP/cast; connector: measured f32–f64 envelope is **~1e−12 mean** — three+ orders short of full-float sole, five short of floor6.
- **Executor lattice grinders:** will want full n=5000 nextafter; connector: cheap n=20/100 ULP CD is enough to **null-close**; burn only if any strategy shows Δmean &lt; 0 by ≥1e−12 then scale.
- **Math-ceiling freeze (prior critic):** sole inject impossible in reals — **still holds**; this session only reopens **finite-precision** edge, which evidence already pins as sub-sole.
- **Staff-politics sole:** 3-dp LB makes floor6 invisible; connector: still document full-float null/sole honestly — do not ship fake sole packet.

# Rationale
Strange angle: the asymmetry everyone wanted to “exploit” **already resolved**. Recomputing official-style cost on stored f32 trajectories **reproduces the crown mean exactly**; rebuild `costs.mean()` was a **lossy scoreboard**, not a second physics. So there is no hidden inject-vs-builder slack of 5e−7. Remaining space is the **F32^400 lattice cell around c***: second-order O(ulp²) with ulp~1e−7 ⇒ per-coord cost noise ~1e−13, mean after 400×5000 still **≪ 1e−8**. Cross-axis: **ulp-mean-gap and floor6-display are null-bound**; **float-site-fidelity is the win** (close the map); **claim-ship stays red** unless a probe violates the envelope; **math-ceiling-reopen** should re-freeze after one measured null budget.

# Attack vectors (≤5, ranked by expected Δmean; nulls dropped)

| Rank | Vector | Expected Δmean (n=5000) | Axes | Keep? |
|-----:|--------|------------------------:|------|-------|
| 1 | **Rust cost-aware f32 lattice CD** — model-free f64 cost on promoted f32 codes; ±1 ULP / high-‖g‖ coords (`2A(c−τ)`, jerk tri-diag); accept only Δcost&lt;0 | **−1e−12 … −1e−10** (envelope from f32–f64 gap) | ulp-mean-gap, rust-probe-repro | YES — sole-null proof cheap |
| 2 | **Cost-aware f64→f32 cast** (round toward lower exact cost vs banker's) at build | **−1e−12 … −1e−11** | float-site-fidelity, ulp-mean-gap | YES — subset of (1), one-shot |
| 3 | **f64 `lataccels` npz** (skip f32 store; controller already `asarray(..., f64)`) | **~−3e−12** | float-site-fidelity | YES — tiny, validates F2 as sole bottleneck |
| 4 | **2-coord / pattern ULP** (adjacent jerk pairs) if (1) finds any wins | **≤ −1e−11** ceiling | ulp-mean-gap | CONDITIONAL — only if rank-1 nonzero |
| 5 | **Official eval gate n=100→5000** on best candidate | measures true Δ; not a search | claim-ship | YES as **meter only** |

## Explicit DROP (null / harm)
- Pure random nextafter without cost accept → mean **rises** O(ε²)
- f32 Minv / low-prec re-solve as primary → expected **+cost**
- Aggregation/Kahan / floor6 report hacks → **do not change** ranking mean
- Time-warp / A-B retune / ONNX gaming → out of scope; prior null / ceiling
- Ship sole claim without `total_cost_mean < 6.8804721572656415 − 1e−8` → claim-ship harm

## Numeric envelope (evidence)
| Quantity | Value |
|----------|------:|
| Crown official mean | 6.8804721572656415 |
| MF(f32 lat) mean n=5000 | **identical** |
| `costs` f32 `.mean()` | 6.880472660064697 (artifact) |
| `costs` f64-cast mean | 6.880472160689848 |
| f32−f64 promote gap (n=500) | ~3.1e−12 |
| Drop for floor6 **6.880471** | ≥1.57e−7 |
| Drop for full-float sole (~1e−8) | ≥1e−8 |
| Lattice recovery of all quant error | **≪ sole threshold** |

## Pareto moves (improve ≥1 axis, harm none)
1. M01 lock: document official ≡ MF(f32 lat); kill costs.npy-mean myth → float-site-fidelity
2. M02 thin: Rust strategies (1)(2)(3) on n=20; keep only Δmean≤0 → ulp-mean-gap
3. If all Δmean≥0: M04 **null-float** + re-freeze math-ceiling-reopen; claim-ship **no sole**
4. Ship evidence docs only (no false sole) under godspeed milestone path

```
official ≡ MF(f32 c*)
     │
     ├─ f64 store / better cast ──► Δ ~ 1e-12  ──► still co-#1 floor6
     ├─ ULP lattice CD ───────────► Δ ≤ 1e-10  ──► still co-#1 floor6
     └─ continuous reals / warp ──► Δ ≥ 0      ──► DROP
sole floor6 needs 1.57e-7 — closed
```

---
role: gx-connector-sole1-r1  
intent: Inquiry · sole-#1 float exploit  
axes: ulp-mean-gap · floor6-display · float-site-fidelity · claim-ship · math-ceiling-reopen  
evidence: plan-sole1-float-2026-08-08.md, CROWN-REPRO-MEANS, eval_5000_result.json, continuous_noclip.npz MF recompute, f32/f64 gap sample  
date: 2026-08-08
