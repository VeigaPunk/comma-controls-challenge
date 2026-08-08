# r1 labrat smoke — comma-controls-challenge

agent: gx-labrat-math-smoke  
date: 2026-08-08  
axes: [e2e-gate, lb-inject-fidelity]  
scope: black-box only (no Python written)

## Environment

| check | result |
|-------|--------|
| `.venv` | OK (`VENV_OK`, `.venv/bin/python` present) |
| `data/SYNTHETIC` | OK — 20000 CSV segs (`00000.csv`…) |
| `models/tinyphysics.onnx` | used successfully by eval |
| `controllers/` | `continuous_lookup_noclip.py`, `pid.py`, `__init__.py` |
| `artifacts/*.npz` | present |
| `rust/` | **NO_RUST_DIR** (absent at repo root) |

### NPZ artifacts

```
artifacts/continuous_noclip.npz              8681010  (2026-08-08 19:48)
artifacts/continuous_noclip_rebuild_raw.npz  8681010  (2026-08-08 20:03)
```

`continuous_noclip.npz` keys (via venv numpy):

- `hashes` (5000,)
- `lataccels` (5000, 400)
- `init_costs` (5000,)
- `costs` (5000,)

Controller default path: `artifacts/continuous_noclip.npz` (from module docstring/path).

## Eval smoke (`eval.py`)

### Command (n=3)

```bash
.venv/bin/python eval.py \
  --model_path ./models/tinyphysics.onnx \
  --data_path ./data/SYNTHETIC \
  --num_segs 3 \
  --test_controller continuous_lookup_noclip \
  --baseline_controller pid
```

**exit:** 0  
**output (tail):**

```
Running rollouts for visualizations...
  ... 3/5 (SAMPLE_ROLLOUTS=5 caps viz loop total display; files[:3] effectively)
Running batch rollouts => baseline controller: pid
0it [00:00, ?it/s]   # expected: files[SAMPLE_ROLLOUTS:] empty when num_segs <= 5
Running batch rollouts => test controller: continuous_lookup_noclip
0it [00:00, ?it/s]
Report saved to: './report.html'
```

### Command (n=5)

Same; viz 5/5 completed; batch still `0it` (by design: `SAMPLE_ROLLOUTS = 5` in `eval.py`; batch = `files[5:]`).  
**exit:** 0  
**report.html:** written (~465690 bytes).

### e2e-gate: **PASS**

Data path + controller registration + onnx + report generation all green.

## Per-seg costs (direct `run_rollout`, segs 00000–00004)

| seg | continuous_lookup_noclip total | pid total |
|-----|--------------------------------:|----------:|
| 00000 | 5.316900534469632 | 5.316900534469632 |
| 00001 | 0.6037025036112206 | 0.6037025036112206 |
| 00002 | 3.058771507573824 | 3.058771507573824 |
| 00003 | 9.513545356918293 | 9.513545356918293 |
| 00004 | 0.9180975213763019 | 0.9180975213763019 |

**lb-inject-fidelity: FAIL / suspicious** — test controller costs **bit-identical** to `pid` on first 5 SYNTHETIC segs.  
Likely lookup miss / fingerprint mismatch / fallback path (npz covers 5000 hashed segs; SYNTHETIC may not match table keys). Injected unconstrained lataccel path not empirically proven on this data.

## Unknowns / notes

- No `rust/` tree — Rust port not present.
- Batch map empty for `num_segs ≤ 5` is **not** a failure (eval design).
- Fidelity needs either segs present in `hashes` or a known-hit segment list; not verified here beyond cost equality with pid.
- Did not write Python; black-box probes only.

## Verdict

```
# State
- obs: Hypothesis e2e-gate **pass** [certain] — evidence: eval exit 0, report.html, 5/5 viz rollouts, npz+controller load
- obs: Hypothesis lb-inject-fidelity **fail** [strong] — evidence: identical total/lataccel/jerk costs vs pid on segs 0–4

# Unknowns
- rust/: absent — affects future rust port axis only
- hash hit-rate on SYNTHETIC vs npz hashes: unknown — affects inject fidelity
```
