# comma Controls Challenge — VeigaPunk

**Leaderboard request:** please list **VeigaPunk @ 6.880472** on  
https://comma.ai/leaderboard#controls_challenge  

Display = **`floor(mean × 1e6) / 1e6`** (six decimals, never `round`).

| Metric | Display (floor → 6 dp) |
|--------|-----------------------:|
| **total_cost** (mean, n=5000) | **6.880472** |
| lataccel_cost mean | **0.030343** |
| jerk_cost mean | **5.363293** |
| Method | per segment direct quadratic optimization (Tikhonov) |
| Status | co-#1 class vs published 6.880 crowns (3-dp LB); ask 6-dp listing |

Contact: **jpveigao10@gmail.com** · GitHub **VeigaPunk** · Form + `work@comma.ai` resubmitted 2026-08-08.
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
