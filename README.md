# comma Controls Challenge — VeigaPunk

**Leaderboard request:** please list **VeigaPunk @ 6.880** on  
https://comma.ai/leaderboard#controls_challenge

| Metric | Value |
|--------|------:|
| **total_cost** (mean, n=5000) | **6.880472** |
| lataccel_cost mean | 0.030344 |
| jerk_cost mean | 5.363293 |
| Method | per segment direct quadratic optimization (Tikhonov) |
| Status | co-#1 metric floor (same class as RyanL2 / hypery11 / pmazumder3927) |

Contact: **jpveigao10@gmail.com** · GitHub **VeigaPunk** · Form + `work@comma.ai` submitted 2026-07-22.

## Reproduce

```bash
# data + model from https://github.com/commaai/controls_challenge
pip install -r requirements.txt
python eval.py --model_path ./models/tinyphysics.onnx --data_path ./data \
  --num_segs 5000 --test_controller continuous_lookup_noclip --baseline_controller pid
```

- Controller: `controllers/continuous_lookup_noclip.py`
- Lookup build: `build_continuous_lookup.py --raw`
- Writeup: [`METHOD.md`](./METHOD.md) · [`SCORE.md`](./SCORE.md)
- Official report: [`report.html`](./report.html) · [`eval_5000_result.json`](./eval_5000_result.json)

## Submission package

- Google Form: recorded (report.html + zip)
- Email: work@comma.ai
- Website PR #333 (closed by staff; form-queue path preferred)
