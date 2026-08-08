# gx-simplifier-r3 — rust/controls_beat

## Axes
- rust-repro
- ship-readiness

## State
- obs: unused dep `byteorder` — anti-overfit: pass — savings: 1 Cargo.toml line + lock edge [certain]
- obs: `_use_read` + unused `Read` import — anti-overfit: pass — savings: ~4 lines [certain]
- obs: empty `src/bin/` — anti-overfit: pass — savings: empty dir [certain]
- obs: `fingerprint_from_observations` (public, no in-crate callers) — anti-overfit: **fail** (library surface // Python parity) — **kept**
- obs: other deps (nalgebra, csv, clap, md-5, serde, serde_json, zip) — all used — keep

## Artifact: deletion
1. Removed `byteorder = "1.5"` from `Cargo.toml` (never imported; LE via `to_le_bytes`).
2. Removed `use std::io::Read` and dead `fn _use_read` from `src/lib.rs`.
3. Removed empty `src/bin/`.

evidence: `cargo test` — **4 passed**, 0 failed (including `real_csv_if_present` crown totals/fingerprint).
`cargo build --release` — green.

Formulas/crown untouched (solve_tikhonov, model_free_cost, fingerprint path, npz writer).

Mirror: `rust/controls_beat/.xbgst/r3-simplifier.md`
