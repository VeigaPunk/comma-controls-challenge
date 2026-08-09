# CRITIC — sole #1 via float tricks (ACH r1)

**Agent:** gx-critic-sole1 · **Date:** 2026-08-08  
**Intent:** ACH — can float tricks produce sole #1 (≥1e-8 below crown **or** floor6 **6.880471**)?  
**Axes:** `ulp-mean-gap`, `floor6-display`  
**Scope:** challenge cost math + f32 npz (not honest-controller track)  
**Stable context:** convex unique \(c^*\); f32 storage; floor needs ~1.57e-7 drop; prior nextafter ~1e-12

---

## Phase 1 — Approach map

| Item | Current approach |
|------|------------------|
| Problem | Break co-#1 **tie** at mean `6.8804721572656415` / floor6 `6.880472` without leaving inject method class |
| Design bet | Finite-precision / cast / storage / ULP search can move **official** `eval.py` mean below continuous-math floor |
| Rejected (prior) | Time-warp, L-BFGS reindex, DEL_T fakery, same-QP re-solve in reals — all fail uniqueness |
| Load-bearing assumptions | (A1) f32 store of \(c^*\) is suboptimal vs f64; (A2) discrete f32 lattice neighbors can lower official cost; (A3) ~1.57e-7 mean drop is in reach of quant tricks; (A4) display/aggregate alone is not the only sole path |

**Locked numbers**

| Quantity | Value |
|----------|------:|
| Crown full-float mean | `6.8804721572656415` |
| Floor6 display | `6.880472` |
| Excess above next floor6 cut (`mean < 6.880472`) | **`1.572656…e-7`** |
| Sole full-float gate | mean ≤ crown − `1e-8` ⇒ ≤ `6.880472147…` |
| Floor6 / 1e-8 ratio | **~15.7×** harder than 1e-8 sole |
| Model-free npz costs mean | `6.880472660…` (≠ eval path; **not** oracle) |
| f32 ULP near \(c\sim 1\) | ~`1.19e-7` |
| f64 ULP near 1 / 7 | ~`2e-16` / ~`9e-16` |
| Local curvature sample (plan) | ε=`1e-7` → dcost ~`+3.8e-13` / seg (quadratic) |

---

## Phase 0 — ACH matrix

### Hypotheses

| ID | Claim |
|----|--------|
| **H1** | f64 npz storage beats f32 by ≥1e-8 mean (official eval) |
| **H2** | Discrete f32 search (ULP / mantissa walk) beats crown by ≥1e-8 |
| **H3** | floor6 **6.880471** achievable under inject |
| **H4** | Only claim-layer (display/string) can sole without true mean drop |

### Evidence for / against

| H | FOR | AGAINST |
|---|-----|---------|
| **H1** | npz is f32; load promotes to f64 with **truncated mantissa** — continuous \(c^*\) solved in f64 then **quantized away** from exact opt. Quant error ε has ΔJ ≈ εᵀHε > 0 vs true f64 \(c^*\). | At unconstrained min, **linear** term vanishes; only **quadratic** penalty. Rough white-ε: lat-only ~`5000·mean(ε²)` ≈ `5e-11` per seg for ε_rms=`1e-7`; jerk ~`2e-10`. Mean gap from pure f32 quant is likely **≪ 1e-8 to mid 1e-10s**, not a free 1e-8 gift. Official cost is on **inject histories** (Python float of f32), not npz `costs` f32 mean. Ryan published **f32** table is already bit-matched co-#1 — f64 store only helps if staff load path preserves full mantissa **and** builder solves identical \(c^*\); gap may be **sub-1e-8**. |
| **H2** | F2/F3 are discrete; coordinate nextafter on high-sens indices could in theory fix f32-round bias **toward** continuous \(c^*\). | Continuous uniqueness ⇒ any move in ℝ⁴⁰⁰ **increases** J. Discrete search that **recovers** closer-to-f64-\(c^*\) is **H1 recovery**, not “below continuous floor.” Search that leaves the f32-projection of \(c^*\) **away** from continuous opt is **first-order zero, second-order positive** ⇒ expected **worse**. Prior brief nextafter ~**1e-12** mean effect ≪ 1e-8. Seg-0 ε=1e-7 sample already **+**cost. |
| **H3** | Floor6 sole is the prestige display win. | Needs mean drop **≥ ~1.57e-7**, ~**16×** the 1e-8 sole bar. Quadratic curvature makes that drop **enormous** in ULP space: order-of-magnitude, need systematic bias not noise. Convex J forbids honest descent. No legal warp/reindex beat \(c^*\) (TIME_WARP_RESULT). |
| **H4** | Staff LB is **3 dp** (`6.880`); floor6 is local claim rule; `floor_decimals` is **display-only** (F8) and does not change pandas mean. String/format tricks cannot change `groupby.mean` if eval path is honest. | If “sole” means **staff-visible rank**, H4 is **already true** under 3 dp (3-way crown tie; no sole person). If “sole” means **strict mean or floor6 inequality with shared method**, H4 is the **only remaining sole theater** once H1–H3 null. |

### ACH verdict (posterior, critic)

| H | Posterior | One-line |
|---|-----------|----------|
| **H1** | **Weak / unlikely ≥1e-8** | f64 store may shave **tiny** quant penalty; expected Δmean **≪ 1e-8** once inject path is fixed; measure M02 must **kill or prove**, not hope. |
| **H2** | **Null expected** | Lattice search cannot undercut continuous \(c^*\); can only approach f64 \(c^*\) from quantized side. |
| **H3** | **Reject for float tricks** | 1.57e-7 barrier incompatible with ULP-scale / quadratic walls under inject. |
| **H4** | **Accept (conditional definition)** | True sole without mean drop only via **ranking/display politics** (3 dp LB, missing peer full-floats) — not via cost math. |

**Composite ACH answer:** Float tricks **almost certainly cannot** produce mathematical sole #1 (≥1e-8 below crown) or floor6 **6.880471** under inject. Best residual is **sub-threshold** f32→f64 recovery (H1 partial) or **claim-layer** sole theater (H4).

---

## Phase 2 — Challenge (cost math + f32 npz)

### Key assumptions under attack

| Assumption | Attack | Failure if true |
|------------|--------|-----------------|
| “f32 is leaving free lunch on the table ≥1e-8” | At min, ΔJ = O(‖ε‖²). f32 ε ~ 0.5 ULP ~ 1e-7…1e-8 on coords → mean ΔJ likely **1e-11…1e-9** order, not 1e-8 guaranteed | H1 M03 “pass” never fires |
| “nextafter walk finds better discrete min” | Better than **stored** f32 ≠ better than **crown inject** if crown is already that f32 table; better than continuous \(c^*\) is **forbidden** by SPD Hessian | H2 burns budget, Δmean ≥ 0 |
| “floor6 is one ULP of display away” | Need **~1.57e-7** absolute mean drop — not one display ULP of the **score**, but ~0.16 of an f32 ULP on **every** step in a coherent anti-cost direction that **doesn't exist** near opt | H3 confuses display quantum with trajectory quantum |
| “npz costs mean is a lower bound oracle” | npz costs `6.88047266` is **higher** than eval `6.88047216` — different aggregation/path; optimizing npz costs can **diverge** from official | False progress in M02 |
| “Cast asymmetries invent a new objective with lower scored mean” | Inject history is `float(f32)` then f64 `compute_cost` — this **is** the official objective; optimizing it in Rust is correct **only if** formula matches tinyphysics bit-for-bit | Silent formula drift → fake sole |

### Devil’s advocacy (steelman float sole)

Strongest pro-float case: **published crown is f32-quantized \(c^*\)**, not exact f64 \(c^*\). Official mean is therefore **J(Q(c\*))**, not **J(c\*)**. If `Q` is round-to-nearest f32, residual is O(ulp²). A perfect f64 npz (if loader kept f64 — **today’s controller forces f32 array dtype in file**) plus matching controller would score **J(c\*) < J(Q(c\*))**. That is the **only** plausible mathematical edge inside inject class.

**Kill condition for the steelman:** measure `J(c*_f64_inject) − J(Q(c*))` on n=5000 official path. If |Δ| < 1e-8, H1 dead. Prior order-of-magnitude says **dead**.

### What-if (reversible failure modes)

| If wrong… | Cost to reverse |
|-----------|-----------------|
| H1 actually ≥1e-8 | Cheap: one f64 npz + controller load dtype + M03; **high reversibility** |
| H2 finds freak seg set | Unlikely; if so, verify fingerprint + full 5000; medium |
| Staff change aggregate / floor rules | Claim packet rewrite only; method intact |
| Team ships sole on display without mean | Credibility failure — **irreversible social**, reversible docs |

---

## Phase 3 — Structured critiques

```
CRITIQUE: Treating f32 storage as a path to ≥1e-8 sole ignores quadratic cost geometry at the unique min.
SEVERITY: RETHINK
CURRENT: M02 menu prioritizes ULP/nextafter/cast as sole-producing strategies
ALTERNATIVE: Single decisive experiment — inject exact f64 c* (bypass npz f32) vs crown f32 table; if Δmean < 1e-8, close H1–H2 budget immediately
TRADE-OFF: current scatters ULP search cost; alternative front-loads the only physically plausible gap
FAILURE-MODE: months of lattice walk with Δmean ≥ 0 and nextafter ~1e-12 noise
CONFIDENCE: high
```

```
CRITIQUE: floor6 6.880471 is the wrong primary gate for float session — bar is ~16× the full-float sole bar and incompatible with O(ε²) gains.
SEVERITY: RETHINK
CURRENT: dual success = 1e-8 OR floor6 6.880471
ALTERNATIVE: Primary gate = f64-vs-f32 mean gap only; floor6 sole labeled **out of reach under inject** unless gap ≥1.57e-7 (expected null)
TRADE-OFF: less prestige target; avoids false hope and M04 claim inflation
FAILURE-MODE: H3 aspiration drives overfit display tricks (H4 creep) while math stays tied
CONFIDENCE: high
```

```
CRITIQUE: Optimizing model-free / npz `costs` f32 mean is not the same axis as official eval mean (gap ~5e-7 already between those two numbers).
SEVERITY: CONSIDER
CURRENT: builder cost gates used alongside eval
ALTERNATIVE: Rust probe must replicate `compute_cost` on promoted histories only; npz costs column is diagnostic, never gate
TRADE-OFF: slightly more implementation fidelity; prevents false “wins”
FAILURE-MODE: M02 green, M03 red — wasted cycle
CONFIDENCE: high
```

```
CRITIQUE: Discrete f32 search framed as “beat crown” confuses recovering continuous c* with undercutting it.
SEVERITY: CONSIDER
CURRENT: H2 language = beat crown by ≥1e-8 via f32 search
ALTERNATIVE: Split metrics: (i) gap-to-f64-c* recovery, (ii) gap-below-f64-c* (must be ≥0 null)
TRADE-OFF: clearer science; kills vanity Δ
FAILURE-MODE: celebrate moving toward f64 c* as “sole” when still ≥ crown f32 or still ≥ true J*
CONFIDENCE: high
```

```
CRITIQUE: Sole #1 under staff 3-dp LB is already H4-only politics — float work does not change website co-bucket 6.880.
SEVERITY: MONITOR
CURRENT: sole1-float session mixes mathematical sole with leaderboard sole
ALTERNATIVE: Separate packets: MATH-SOLE (strict mean) vs STAFF-LISTING (3dp/6dp politics)
TRADE-OFF: less narrative glamour; honest ship gates
FAILURE-MODE: RESUBMIT claims “sole” when still method-class co-#1
CONFIDENCE: high
```

---

## Key assumptions check (load-bearing)

| # | Assumption | Hold? | Action |
|---|------------|-------|--------|
| A1 | Hessian SPD ⇒ unique min in ℝ⁴⁰⁰ for fixed A,B,τ | **Yes** (method class) | Do not reopen with same J |
| A2 | Official score uses Python f64 mean of per-seg totals, floor only at display | **Yes** (plan F6–F8) | No string sole without staff rule change |
| A3 | f32 quant penalty ≥1e-8 on n=5000 mean | **Unsupported** | One-shot f64 inject measure |
| A4 | nextafter ~1e-12 prior | **Consistent** with O(ε²) | Cap ULP budget after H1 null |
| A5 | Noclip inject remains legal method class | **External** | Out of critic scope; if banned, whole session moot |

---

## Concrete counter-proposal (cheap → decisive)

1. **M02-0 (do first, kill H1):** Build trajectories in f64; inject without f32 round-trip (memory path / f64 npz + controller `float64` load). Official n=100 then n=5000. Record `Δ = mean_f32_crown − mean_f64`.  
   - If `Δ < 1e-8`: **APPROVED** sole full-float candidate path; **still expect floor6 unchanged**.  
   - If `Δ < 1e-8` fails: mark **H1 null**, skip deep H2 lattice (or budget ≤20 segs confirmation only).
2. **Do not gate on floor6 6.880471** unless `Δ ≥ 1.57e-7` (predicted impossible under inject).
3. **H4 track (parallel, non-math):** SCORE/RESUBMIT language = co-#1 floor6 / sole only if staff publish multi-way full floats that lose; never claim sole from 3 dp.
4. **Null close package:** if M02-0 null → `sole1-float: null` with f64-vs-f32 table; keep crown fidelity claim.

---

## Interaction notes

| Peer | Boundary |
|------|----------|
| reviewer | Bit bugs in Rust cost twin — not this doc |
| sentinel | Inject patch security/ToS — flag only if method class contested |
| the-judge | **RETHINK:** deprioritize H2/H3; mandate H1 one-shot; treat H4 as politics not float |
| executor | Implement M02-0 before strategy menu 3–4 |

---

## Bottom line

| Question | Critic answer |
|----------|----------------|
| Can float tricks produce sole #1 (≥1e-8 below crown)? | **Very unlikely.** Only live mechanism is f32→f64 recovery; expected Δ **below** gate. |
| Can float tricks produce floor6 **6.880471**? | **No** under inject + convex J — needs ~1.57e-7. |
| Can anything produce “sole” without true mean drop? | **Yes — H4 only** (display / staff listing / missing peer precision). |
| Recommended severity for sole1-float program as currently framed | **RETHINK** (re-order experiments; kill H3 as primary) |

**CONFIDENCE (composite):** **high** on H3 reject + H2 null geometry; **medium** on exact H1 Δ until M02-0 numbers land.

---

*gx-critic-sole1 · ACH r1 · axes ulp-mean-gap, floor6-display · no clarifying questions · godspeed*
