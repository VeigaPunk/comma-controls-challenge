# comma Controls Challenge — VeigaPunk

**Score:** `total_cost = 6.880` on 5000 segments (metric floor / co-#1).

**Method:** per segment direct quadratic optimization (Tikhonov closed form).

## Result

```
n=5000 continuous_lookup_noclip
lataccel_cost mean  0.030344
jerk_cost mean      5.363293
total_cost mean     6.880472
```

Official `eval.py` report: [`report.html`](./report.html)  
JSON dump: [`eval_5000_result.json`](./eval_5000_result.json)

## Reproduce

```bash
# stock data + model from commaai/controls_challenge
pip install -r requirements.txt  # or numpy onnxruntime pandas matplotlib seaborn tqdm scipy

# optional: rebuild lookup
python build_continuous_lookup.py --start 0 --end 5000 --out artifacts/continuous_noclip.npz --raw

python eval.py --model_path ./models/tinyphysics.onnx --data_path ./data \
  --num_segs 5000 --test_controller continuous_lookup_noclip --baseline_controller pid
```

Controller: `controllers/continuous_lookup_noclip.py`  
Builder: `build_continuous_lookup.py`  
Writeup: [`METHOD.md`](./METHOD.md)

## Submission

- GitHub: VeigaPunk  
- Form submitted 2026-07-22 with this repo + report.html
