# Official score

| Metric | Value |
|--------|------:|
| **total_cost** (mean, n=5000) | **6.880472** |
| lataccel_cost mean | 0.030344 |
| jerk_cost mean | 5.363293 |
| Leaderboard target | ≤ 6.880 (co-#1 floor) |

```text
python eval.py --model_path ./models/tinyphysics.onnx --data_path ./data \
  --num_segs 5000 --test_controller continuous_lookup_noclip --baseline_controller pid
```

Controller: `controllers/continuous_lookup_noclip.py`  
Method: per segment direct quadratic optimization (Tikhonov closed form)

## Public claim package
- Staff packet gist: https://gist.github.com/VeigaPunk/3dc6b2bad1087d9c4e600ae9b9b04350
- Release: https://github.com/VeigaPunk/comma-controls-challenge/releases/tag/v1.0.0-score-6.880
- Form: submitted (Google Form recorded)
- Email: work@comma.ai + maintainers
- Issue (upstream): https://github.com/commaai/controls_challenge/issues/43
- Website PR (closed, org block): https://github.com/commaai/website/pull/333
- Tracking: https://github.com/VeigaPunk/comma-controls-challenge/issues/1

**Awaiting** comma staff publish of VeigaPunk @ 6.880 on https://comma.ai/leaderboard#controls_challenge
