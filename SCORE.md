# Local evaluation score — awaiting comma.ai verification

Local display rule for the submission request: **`floor` to 6 decimal places** — never `round`.

```text
display = floor(mean * 1e6) / 1e6
```

| Metric | Full float mean (n=5000) | **Display (floor → 6 dp)** |
|--------|-------------------------:|---------------------------:|
| **total_cost** | 6.8804721572656415 | **6.880472** |
| lataccel_cost | 0.030343581281987223 | **0.030343** |
| jerk_cost | 5.363293093166279 | **5.363293** |

The issuer leaderboard currently displays 3 dp and does not list VeigaPunk. This locally reproduced result falls in the same displayed **6.880** bin as listed leaders; issuer placement, tie status, and 6-decimal publication remain unverified. The resubmit asks staff to list **VeigaPunk @ 6.880472** using the local 6-dp floor.

```text
python eval.py --model_path ./models/tinyphysics.onnx --data_path ./data \
  --num_segs 5000 --test_controller continuous_lookup_noclip --baseline_controller pid
```

Controller: `controllers/continuous_lookup_noclip.py`  
Method: per segment direct quadratic optimization (Tikhonov closed form)  
Report aggregates: `eval.py` uses `floor_decimals(..., 6)` (not `.round(3)`).

## Local result submission package
- Staff packet gist: https://gist.github.com/VeigaPunk/3dc6b2bad1087d9c4e600ae9b9b04350
- Release: https://github.com/VeigaPunk/comma-controls-challenge/releases/tag/v1.0.0-score-6.880
- Form: resubmitted 2026-08-08 (6-dp floor display)
- Email: work@comma.ai + maintainers
- Issue (upstream): https://github.com/commaai/controls_challenge/issues/43
- Website PR (closed, org block): https://github.com/commaai/website/pull/333
- Tracking: https://github.com/VeigaPunk/comma-controls-challenge/issues/1

**Request:** publish **VeigaPunk @ 6.880472** on https://comma.ai/leaderboard#controls_challenge (6 decimal places, floor not round).