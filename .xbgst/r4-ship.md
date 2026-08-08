# r4-ship — gx-executor-r4-ship

date: 2026-08-08  
agent: gx-executor-r4-ship  
axes: [ship-readiness, rust-repro, honest-control]

## Gates

| Gate | Result |
|------|--------|
| `cargo test -q` rust/controls_beat | **green** — 4 passed |
| Secret rg (staged paths) | **clean** |
| Stage scope | rust + `.xbgst/r3-*` only; no data/ models/ npz |
| Push | `origin/main` SSH VeigaPunk |

## SHA

- **full:** `6378aac3724809a70c2a2d90f7c027fe04beb16d`
- **short:** `6378aac`
- **parent:** `df5b533`
- **msg:** Ship R3 simplifier byteorder drop, honest n=100 baseline docs, and cargo-green controls_beat.

## Shipped paths

- `rust/controls_beat/Cargo.toml` — drop `byteorder`
- `rust/controls_beat/Cargo.lock`
- `rust/controls_beat/src/lib.rs`
- `.xbgst/r3-honest-baseline.md` — pid n=100 vs inject
- `.xbgst/r3-simplifier.md`
- `rust/controls_beat/.xbgst/r3-simplifier.md`

## Not shipped

- `r3-connector` — absent from project `.xbgst`
- `report.html`, `artifacts/*.npz`, `data/`, `models/`, pycache

## APPROVED

APPROVED: R3 simplifier + honest baseline on main @ 6378aac; cargo test green; secrets clean.
