# M07 — Obliterate session ship (Rust crown lock)

**Status:** COMPLETE | **Date:** 2026-08-08 | **Session:** obliterate-r0→r2  
**Role:** gx-scribe-r3 | **Axes:** ship-readiness (primary)

## Does

Locks ship-readiness for the obliterate session: Rust `controls_beat` on main rebuilds the Tikhonov raw continuous lookup bit-identical to the published crown NPZ; official Python eval n=5000 under `CONTINUOUS_LOOKUP_PATH` yields floor6 **6.880472**; session evidence lives under `.xbgst/` without bulk data/models/npz.

## Gate

```bash
test -f artifacts/rust_eval_5000.json && \
  rg -n 'total_cost_mean_floor6|6\.880472' artifacts/rust_eval_5000.json .xbgst/r2-executor-crown.md && \
  test -d rust/controls_beat/src && \
  git log -2 --oneline
```

Expected: floor6 **6.880472** present in metrics + r2 crown note; crate on tree; commits `6b82d56` / `796031e` ancestry.

Actual (executor evidence, transcribed):

- `artifacts/rust_eval_5000.json`: `"total_cost_mean_floor6": 6.880472`, full float `6.8804721572656415`
- Rust build-lookup n=5000: `floor6` 6.880472, mean `6.880472160689848`, wall ~1.41s (model-free path)
- NPZ parity vs `artifacts/continuous_noclip.npz`: hashes equal; lataccels/costs max abs diff **0.0**
- `assert-score --floor6 6.880472` → pass (documented in r2-executor-crown)
- Prior main: `6b82d56` crate; `796031e` rust_eval + r2 crown doc

## Touches (this scribe ship)

- `.xbgst/MILESTONE-OBLITERATE-2026-08-08.md` — this report (1:1:1)
- `.xbgst/axes-r0.md` — named axes freeze
- `.xbgst/plan-obliterate-2026-08-08.md` — Phase-0 plan (read-only archive of session plan)
- `.xbgst/r1-connector.md`, `.xbgst/r1-labrat-smoke.md`, `.xbgst/r1-revenger-inject.md`
- `.xbgst/r2-connector.md`, `.xbgst/r2-labrat-inject.md`
- Already on main (not re-committed unless dirty): `rust/controls_beat/**`, `.xbgst/r1-executor-rust.md`, `.xbgst/r2-executor-crown.md`, `artifacts/rust_eval_5000.json`, `SCORE.md`, `METHOD.md`

## Out-of-scope

- Bulk NPZ (`artifacts/rust_continuous_noclip.npz`, `continuous_noclip_rebuild_raw.npz`, lookup smoke npz) — local only / gitignored
- `data/`, `models/`, `__pycache__/`, `src/comma_controls_beat/` empty py package, dirty `report.html` regen
- M06 honest-control (unpatched sim vs PID) — deferred; math-ceiling sole-#1 under inject remains impossible
- Force-push, LFS of 20k CSVs, new Python controllers

## Findings

- **lb-inject-fidelity PASS:** floor6 6.880472; rust NPZ arrays bit-identical to published crown (r2-executor-crown §2–3; `artifacts/rust_eval_5000.json` lines total_cost_mean_floor6).
- **rust-repro PASS:** crate at `rust/controls_beat` (`6b82d56`); build-lookup / prove-seg / assert-score CLIs (r1-executor-rust).
- **e2e-gate PASS:** official eval with `CONTINUOUS_LOOKUP_PATH=artifacts/rust_continuous_noclip.npz`, `num_segs=5000` > SAMPLE_ROLLOUTS (labrat trap documented r1-labrat / r2-connector).
- **math-ceiling LOCK:** sole-#1 under inject DROP; co-#1 class only (axes-r0 #6; plan escalation).
- **ship-readiness:** secret gate on staged `.xbgst` + known evidence paths; no bulk binaries; push origin main after this commit.
- **honest-control:** not green this session (out of scope M06).

## Axes deltas (session)

| Axis | Baseline | After R2 crown | Δ |
|------|----------|----------------|---|
| lb-inject-fidelity | claimed 6.880472 Python | Rust rebuild + eval floor6 6.880472 | lock |
| rust-repro | 0 crates | `controls_beat` + cargo gates | ↑ |
| e2e-gate | Python-only | CONTINUOUS_LOOKUP_PATH rust npz n=5000 | ↑ |
| honest-control | unknown | deferred | — |
| ship-readiness | docs, no rust ship | crate + metrics + .xbgst evidence on main | ↑ |
| math-ceiling | co-#1 documented | sole-#1 under inject still impossible | lock |

## Evidence paths (local; do not bulk-commit)

| Path | Role |
|------|------|
| `artifacts/rust_eval_5000.json` | **committed** metrics-only n=5000 |
| `.xbgst/r2-executor-crown.md` | **committed** e2e procedure + parity table |
| `.xbgst/r1-executor-rust.md` | **committed** crate design + unit gates |
| `rust/controls_beat/` | **committed** source |
| `artifacts/rust_continuous_noclip.npz` | local rebuild (~7.7MB); content ≡ published crown arrays |
| `artifacts/continuous_noclip.npz` | published crown reference (may already be tracked) |
| `SCORE.md` / `METHOD.md` | display floor6 + Tikhonov method |

## Links

- Plan: `.xbgst/plan-obliterate-2026-08-08.md`
- Prior commits: `6b82d56` (crate), `796031e` (n=5000 doc + json)
- Origin: `git@github.com:VeigaPunk/comma-controls-challenge.git`
- Next: M06 honest-control probe **or** staff 6-dp listing chase (orthogonal)

## APPROVED

Ship session `.xbgst` evidence + milestone report; rust/metrics already on main; no secrets; no bulk data.
