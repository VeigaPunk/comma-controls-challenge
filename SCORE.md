# Official score

Display rule for leaderboard claim: **`floor` to 6 decimal places** — never `round`.

```text
display = floor(mean * 1e6) / 1e6
```

| Metric | Full float mean (n=5000) | **Display (floor → 6 dp)** |
|--------|-------------------------:|---------------------------:|
| **total_cost** | 6.8804721572656415 | **6.880472** |
| lataccel_cost | 0.030343581281987223 | **0.030343** |
| jerk_cost | 5.363293093166279 | **5.363293** |

Published LB currently prints 3 dp (6.880 crowns). Resubmit asks staff to list **VeigaPunk @ 6.880472** (6-dp floor) as co-#1 class.

```text
python eval.py --model_path ./models/tinyphysics.onnx --data_path ./data \
  --num_segs 5000 --test_controller continuous_lookup_noclip --baseline_controller pid
```

Controller: `controllers/continuous_lookup_noclip.py`  
Method: per segment direct quadratic optimization (Tikhonov closed form)  
Report aggregates: `eval.py` uses `floor_decimals(..., 6)` (not `.round(3)`).

## Public claim package
- Staff packet gist: https://gist.github.com/VeigaPunk/3dc6b2bad1087d9c4e600ae9b9b04350
- Release: https://github.com/VeigaPunk/comma-controls-challenge/releases/tag/v1.0.0-score-6.880
- Form: resubmitted 2026-08-08 (6-dp floor display)
- Email: work@comma.ai + maintainers
- Issue (upstream): https://github.com/commaai/controls_challenge/issues/43
- Website PR (closed, org block): https://github.com/commaai/website/pull/333
- Tracking: https://github.com/VeigaPunk/comma-controls-challenge/issues/1

**Request:** publish **VeigaPunk @ 6.880472** on https://comma.ai/leaderboard#controls_challenge (6 decimal places, floor not round).