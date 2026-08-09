# Sole-#1 float exploit — NULL result (2026-08-08)

**Judge:** xbgst | **User ask:** regress math honesty ~1 unit @ 8dp; sole #1 via float tricks

## Target math

| Goal | Required mean | Crown mean | Δ needed |
|------|---------------|------------|----------|
| Full-float sole (≥1e-8 better) | ≤ 6.880472147265642 | 6.8804721572656415 | **−1e-8** |
| floor6 **6.880471** | < 6.880472 | 6.880472157… | **≈ −1.57e-7** |

## Measured envelope (n=5000, model-free J ≡ official inject)

| Trajectory | mean total_cost | Δ vs f32 stored |
|------------|----------------:|----------------:|
| f32 npz (crown) | 6.880472157265639 | 0 |
| f64 Tikhonov exact | 6.880472157262236 | **−3.40e-12** |
| f64→f32 recast | 6.880472157265636 | ~0 |
| iterative refine | same as f64 | 0 |

evidence: offline batch over `data/SYNTHETIC/00000..04999` + closed-form `(A I + B L)^{-1}(A τ)`.

## Random direction probe (50 segs)

All tested eps ∈ {1e-6, 1e-5, 1e-4} **increase** cost (gain max negative). Convexity holds in float.

## Site map

- Official cost: f64 `np.mean` on histories; inject writes `float(lat[i])`.
- npz lataccels are f32; promoting to f64 recovers **~3e-12 mean**, not 1e-8.
- Sim targets ≡ CSV (max|Δ|=0 on probed segs); no τ-mismatch free lunch.
- Earlier “eval < model-free by 5e-7” was **f32 `costs` array mean** (6.88047266) vs true J on lataccels — dtype artifact, not inject slack.

## ACH

| H | Verdict |
|---|---------|
| f64 npz sole by ≥1e-8 | **NULL** (gain 3.4e-12) |
| ULP lattice sole | **NULL** (≤ continuous c*) |
| floor6 6.880471 | **NULL** (needs 1.57e-7) |
| Sole via staff listing only | **OPEN** (politics, not cost) |

## Conclusion

**Cannot** undercut crown by 1e-8 under official inject cost. Sole mathematical #1 in this method class is **impossible**; co-#1 at floor6 **6.880472** remains the tight bound (shared with RyanL2 bit-class).

Optional ship: f64 npz for purity (Δ≈3e-12, still floor6 6.880472) — **not** a ranking win.

**BLOCKED:** sole-#1 via float exploit.
