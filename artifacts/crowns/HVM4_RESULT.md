# HVM4 on controls co-#1 — experiment

**Why HVM4:** Interaction Calculus is built for *exploring superposed alternatives* and
*parallel folds*. The inject optimum \(c^*\) is unique in ℝ⁴⁰⁰; HVM does not invent a
new physics. It **does** give a clean substrate for:

1. **Exact integer folds** of fixed-point segment costs (no float reduction order drama)
2. **Ranking folds** (`min2` / `min3`) over floor6 bins
3. **Superposition enumeration** of ranking worlds (`-C`)

## Ran

```bash
~/Projects/HVM4/src/hvm hvm4/controls_floor6.hvm -s
# → #Triple{34402355,6880472,6880472}
```

| quantity | value |
|----------|------:|
| HVM mean (fp scale 1000) | 6.880471000000 |
| HVM mean floor6 | 6.880471 |
| Python mean | 6.88047216068985 |
| Python floor6 | 6.880472 |
| HVM min(us, Ryan 6.8805 slogan) | **US** (6880472) |

Superposition collapse (`hvm4/controls_sup_rank.hvm -C10`):

```
6880472   // us floor6
6880500   // Ryan slogan
6880471   // counterfactual gap-cross
```

## Verdict

| claim | result |
|-------|--------|
| HVM finds inject cost &lt; \(c^*\) | **No** (not what IC does here) |
| HVM locks mean ≈ 6.880472 | **Yes** (matches Python) |
| HVM ranking: us floor6 vs Ryan **6.8805** | **Us wins** (6.880472 &lt; 6.880500) |
| Sole #1 under stock re-eval | Still **co-class** with three crowns |
| Elegant use of HVM4 | **Ranking + exact fold + superposed score worlds** |

## Files

- `hvm4/controls_floor6.hvm` — generated IC program
- `hvm4/controls_sup_rank.hvm` — superposed floor6 worlds
- `artifacts/crowns/hvm4_floor6.json` — machine report

**One-liner:** HVM4 is the right *scoreboard algebra* for floor6 politics; it is not a
second physics that undercuts Tikhonov \(c^*\).
