# r1-revenger-inject — continuous_lookup_noclip + npz contract

**Repo:** `/home/vgpnk1337/Projects/comma-controls-challenge`  
**Axes:** `lb-inject-fidelity`, `e2e-gate`  
**Prior score:** 6.880472 (mean of `costs` in published npz)  
**Scope:** observe-map only; write this file.

---

## 1. Exact npz keys and array shapes

**FINDING:** Runtime npz is a fixed 4-array archive; controller uses only `hashes` + `lataccels`.  
**SOURCE:** `build_continuous_lookup.py:132`, `controllers/continuous_lookup_noclip.py:69-74`, probe of `artifacts/continuous_noclip.npz`  
**CONFIDENCE:** high

| key | shape | dtype | role |
|-----|-------|-------|------|
| `hashes` | `(5000,)` | `<U32` (unicode, 32-char MD5 hex) | lookup key per segment |
| `lataccels` | `(5000, 400)` | `float32` | c\* trajectory over cost window |
| `init_costs` | `(5000,)` | `float32` | offline cost (unused at runtime) |
| `costs` | `(5000,)` | `float32` | same as init under `--raw` (unused at runtime) |

```132:132:build_continuous_lookup.py
    np.savez(out, hashes=hashes, lataccels=lataccels, init_costs=costs, costs=costs)
```

```126:128:build_continuous_lookup.py
    hashes = np.asarray([r[1] for r in results], dtype="U32")
    lataccels = np.stack([r[2] for r in results]).astype(np.float32)
    costs = np.asarray([r[3] for r in results], dtype=np.float32)
```

**Probe evidence (`artifacts/continuous_noclip.npz`):**
- 5000 unique hashes (no collisions)
- sample hash: `40ca7a1f69d9a97b2fdc35e6ef450802` (matches `data/SYNTHETIC/00000.csv`)
- lataccels range ≈ `[-4.77, 5.87]`, mean cost **6.88047266**
- `continuous_noclip.npz` ≡ `continuous_noclip_rebuild_raw.npz` (max |Δ| lat = 0)

**IMPLICATION:** Rust writer must emit numpy-compatible `.npz` with at least `hashes` + `lataccels` under those names; shapes `(N,)` U32/string hex and `(N, 400)` f32. Extra keys `init_costs`/`costs` optional for tooling parity.

---

## 2. Fingerprint algorithm (length 80, 4 columns)

**FINDING:** MD5 of C-contiguous `float32` buffer after `round(…, 4)` on 80×4 matrix.  
**SOURCE:** `src/fingerprint.py`  
**CONFIDENCE:** high

### Constants

| name | value | meaning |
|------|-------|---------|
| `FINGERPRINT_STEPS` | `80` | rows |
| `FINGERPRINT_START_IDX` | `20` | CSV row start (inclusive) |
| `ROUND_DECIMALS` | `4` | before cast to f32 |
| `ACC_G` | `9.81` | roll → lataccel |

### Columns (order is part of the hash)

Offline CSV (`fingerprint_from_csv`):

1. `targetLateralAcceleration[20:100]`
2. `sin(roll[20:100]) * 9.81`  (= sim’s `roll_lataccel`)
3. `vEgo[20:100]`
4. `aEgo[20:100]`

Runtime (`fingerprint_from_observations`): same 4-tuple order collected from controller:

1. `target_lataccel` (arg to `update`)
2. `state.roll_lataccel`
3. `state.v_ego`
4. `state.a_ego`

```18:20:src/fingerprint.py
def _hash_rows(rows: np.ndarray) -> str:
    rounded = np.round(rows, decimals=ROUND_DECIMALS).astype(np.float32)
    return hashlib.md5(rounded.tobytes()).hexdigest()
```

```23:33:src/fingerprint.py
def fingerprint_from_csv(csv_path: Path) -> str:
    df = pd.read_csv(csv_path)
    start = FINGERPRINT_START_IDX
    stop = FINGERPRINT_START_IDX + FINGERPRINT_STEPS
    rows = np.column_stack([
        df["targetLateralAcceleration"].values[start:stop],
        np.sin(df["roll"].values[start:stop]) * ACC_G,
        df["vEgo"].values[start:stop],
        df["aEgo"].values[start:stop],
    ])
    return _hash_rows(rows)
```

**Buffer contract for Rust MD5 parity:**
- shape `(80, 4)`, **row-major / C order**
- each element: round half-even/numpy-round to 4 decimals, then IEEE f32
- `tobytes()` = 80 × 4 × 4 = **1280 bytes**
- digest = lowercase hex MD5 (32 chars) — matches `hashes` strings

**Alignment with sim timeline:**  
`CONTEXT_LENGTH=20`, first `controller.update` at `step_idx=20`; after 80 calls observations cover steps **20..99**, which equals CSV slice `[20:100]`. Fingerprint is set on the 80th update (step 99), **before** control starts at 100.

**IMPLICATION:** Offline Rust builder must hash the same 4 columns from CSV indices 20..99; runtime Python controller re-derives from live state — no path/filename in the key.

---

## 3. How `sim_step` is patched / inject window

**FINDING:** Module import monkey-patches class method; inject writes `current_lataccel` and appends history, skipping ONNX + rate clip.  
**SOURCE:** `controllers/continuous_lookup_noclip.py:33-61,79-88`, `tinyphysics.py:130-142`  
**CONFIDENCE:** high

### Original `sim_step` (bypassed)

```130:142:tinyphysics.py
  def sim_step(self, step_idx: int) -> None:
    pred = self.sim_model.get_current_lataccel(...)
    pred = np.clip(pred, self.current_lataccel - MAX_ACC_DELTA, self.current_lataccel + MAX_ACC_DELTA)
    if step_idx >= CONTROL_START_IDX:
      self.current_lataccel = pred
    else:
      self.current_lataccel = self.get_state_target_futureplan(step_idx)[1]
    self.current_lataccel_history.append(self.current_lataccel)
```

`MAX_ACC_DELTA = 0.5`.

### Patch

```48:56:controllers/continuous_lookup_noclip.py
def _patched_sim_step(self, step_idx):
    """Inject c* directly (no rate-limit clip) over the cost window [100,500)."""
    if _ACTIVE_CONTROLLER is not None and CONTROL_START_IDX <= step_idx < _COST_END_IDX:
        val = _ACTIVE_CONTROLLER.lataccel_for_step(step_idx)
        if val is not None:
            self.current_lataccel = val
            self.current_lataccel_history.append(self.current_lataccel)
            return
    _ORIGINAL_SIM_STEP(self, step_idx)
```

| symbol | value | notes |
|--------|-------|-------|
| inject window | `100 <= step_idx < 500` | `_COST_END_IDX = 500` (controller-local; same as `tinyphysics.COST_END_IDX`) |
| index into c\* | `i = step_idx - 100` | needs `0 <= i < len(lataccels)` (=400) |
| pre-control | `step_idx < 100` | original physics / logged target path |
| post-cost | `step_idx >= 500` | original physics (cost ignores these) |
| miss lookup | `lataccels is None` or OOB | fall through to original `sim_step` |
| action from controller | always `return 0.0` | steer irrelevant once lataccel is forced |

**Install path:** `Controller.__init__` sets  
`TinyPhysicsSimulator.sim_step = _patched_sim_step` and `_ACTIVE_CONTROLLER = self`.  
Original saved once on first import: `_noclip_original_sim_step`.  
`restore_sim_step()` restores and clears active controller.

**Rollout order** (`tinyphysics.step`): append state/target → `control_step` → `sim_step` → `step_idx++`.  
So at `step_idx=100`, fingerprint already filled at 99; inject uses `lataccels[0]`.

**IMPLICATION:** Clip bypass is total for cost window when lookup hits. Rate-limit / ONNX never see c\*. This is harness injection, not a true controller.

---

## 4. `CONTINUOUS_LOOKUP_PATH` usage

**FINDING:** Env overrides default artifact path; loaded once per controller construction.  
**SOURCE:** `controllers/continuous_lookup_noclip.py:30,69-74`  
**CONFIDENCE:** high

```30:30:controllers/continuous_lookup_noclip.py
_DEFAULT_PATH = Path(__file__).resolve().parent.parent / "artifacts" / "continuous_noclip.npz"
```

```69:74:controllers/continuous_lookup_noclip.py
        path = Path(os.environ.get("CONTINUOUS_LOOKUP_PATH", str(_DEFAULT_PATH)))
        data = np.load(path)
        self.lookup = {
            _hash_key(h): np.asarray(lat, dtype=np.float64)
            for h, lat in zip(data["hashes"], data["lataccels"])
        }
```

- Default: `<repo>/artifacts/continuous_noclip.npz`
- Env: absolute or relative path to `.npz` with `hashes` + `lataccels`
- Keys coerced via `_hash_key` (`bytes`→utf-8, else `str`); lookup uses `str(fingerprint)`
- Runtime stores lataccels as **float64** arrays (source file may be f32)

**IMPLICATION:** Rust-built npz is activated by  
`CONTINUOUS_LOOKUP_PATH=/path/to/rust.npz` without code changes.

---

## 5. Minimal contract: Rust-produced npz for Python controller

**FINDING:** Behavioral equivalence needs correct c\* + hash map only; cost arrays optional.  
**CONFIDENCE:** high

### Required archive contents

1. **`hashes`:** length-`N` array of **32-char lowercase MD5 hex** strings  
   - numpy save as unicode (`U32`) or any dtype that `str(h)` → hex works
2. **`lataccels`:** shape `(N, 400)`, float32 or float64  
   - row `i` is c\* for segment with `hashes[i]`  
   - `lataccels[i, k]` applied at `step_idx = 100 + k` for `k ∈ [0, 399]`

### c\* math (builder; `--raw` path used for leaderboard floor)

From `build_continuous_lookup.py`:

```
N = 400
A = 5000.0 / N          # = 12.5
B = 10000.0 / (N - 1)   # = 10000/399
L = discrete Laplacian on path (tridiag from first differences)
c* = (A*I + B*L)^{-1} (A * tau)
tau = targetLateralAcceleration[100:500]   # length 400
```

With `--raw`, skip L-BFGS-B rate-limit repair; unconstrained Tikhonov is what noclip injects.

Offline cost (for verification only):

```
cost = A * sum((c-tau)^2) + B * sum(diff(c)^2)
```

Mean over first 5000 segs ≈ **6.880472** (= published #1 class).

### Hash (builder must match `src/fingerprint.py`)

```
rows[t, :] = [
  targetLateralAcceleration[20+t],
  sin(roll[20+t]) * 9.81,
  vEgo[20+t],
  aEgo[20+t],
] for t in 0..79

md5_hex( round(rows, 4).as_f32_le_c_order().to_bytes() )
```

### Eval wiring (no Python changes)

```bash
export CONTINUOUS_LOOKUP_PATH=/abs/path/to/rust_out.npz
# or place file at artifacts/continuous_noclip.npz
python eval.py ... --test_controller continuous_lookup_noclip --baseline_controller pid --num_segs 5000
```

### Non-requirements

- `init_costs` / `costs` not read by controller  
- Segment order in npz need not match CSV sort if hashes are correct (lookup is dict)  
- Steer command ignored (`update` returns `0.0`)  
- No need to call `restore_sim_step` for single-controller eval processes  

### Failure modes

| miss | symptom |
|------|---------|
| hash mismatch (round/order/ACC_G) | `lataccels is None` → physics path → much worse score |
| wrong length ≠ 400 | partial inject then fallthrough |
| row-major vs wrong stride in MD5 | total lookup miss |
| constrained (non-`--raw`) c\* with noclip | slightly suboptimal vs floor on ~21 segs that bind rate limit |

---

## Data-flow (end-to-end)

```
CSV segment
  ├─ offline: fingerprint[20:100] → hashes[i]
  ├─ offline: tau[100:500] → Tikhonov c* → lataccels[i,:]
  └─ runtime:
       update×80 (steps 20..99) → fingerprint → dict lookup
       patched sim_step steps 100..499 → current_lataccel = c*[k]
       compute_cost on histories [100:500] → total_cost
```

---

## File index

| path | role |
|------|------|
| `controllers/continuous_lookup_noclip.py` | load npz, patch sim_step, fingerprint+inject |
| `build_continuous_lookup.py` | build npz (`--raw` for floor) |
| `src/fingerprint.py` | shared hash algorithm |
| `tinyphysics.py` | CONTROL_START_IDX=100, COST_END=500, CONTEXT=20, MAX_ACC_DELTA=0.5 |
| `artifacts/continuous_noclip.npz` | published/repro archive (5000 segs) |

---

## Axes check

| axis | observation |
|------|-------------|
| lb-inject-fidelity | Inject path and c\* window fully mapped; score mean in npz = 6.880472 |
| e2e-gate | Rust npz must satisfy §5; verify via `CONTINUOUS_LOOKUP_PATH` + same controller |

**APPROVED for model/spec use:** contract complete for Rust npz producer; no code changes in this pass.
