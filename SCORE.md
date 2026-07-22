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

Public claim package:
- Website PR: https://github.com/commaai/website/pull/333
- Issue: https://github.com/commaai/controls_challenge/issues/43
