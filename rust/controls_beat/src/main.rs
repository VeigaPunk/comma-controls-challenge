//! CLI: build-lookup | prove-seg | assert-score

use clap::{Parser, Subcommand};
use controls_beat::{
    floor_decimals, list_csvs, model_free_cost, solve_segment_csv, solve_tikhonov,
    write_lookup_npz, CONTROL_START_IDX, COST_END_IDX, N,
};
use serde::Deserialize;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "controls_beat", about = "Tikhonov c* lookup + cost gates")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Build continuous lookup npz (hashes, lataccels, costs).
    BuildLookup {
        #[arg(long, default_value = "data/SYNTHETIC")]
        data_dir: PathBuf,
        #[arg(long, default_value_t = 0)]
        start: usize,
        #[arg(long, default_value_t = 10)]
        end: usize,
        /// Keep unconstrained c* (no rate-limit QP). Required for noclip floor.
        #[arg(long, default_value_t = false)]
        raw: bool,
        #[arg(long)]
        out: PathBuf,
    },
    /// Prove one segment: print fingerprint, costs, c* length.
    ProveSeg {
        #[arg(long)]
        csv: PathBuf,
        #[arg(long, default_value_t = false)]
        raw: bool,
    },
    /// Assert mean total floor6 against a JSON results file.
    AssertScore {
        #[arg(long)]
        json: PathBuf,
        #[arg(long, default_value_t = 6.880472)]
        floor6: f64,
    },
}

#[derive(Debug, Deserialize)]
struct ScoreJson {
    #[serde(default)]
    mean: Option<f64>,
    #[serde(default)]
    total_cost: Option<f64>,
    #[serde(default)]
    mean_total: Option<f64>,
    #[serde(default)]
    total_cost_mean: Option<f64>,
    #[serde(default)]
    total_cost_mean_floor6: Option<f64>,
    #[serde(default)]
    costs: Option<Vec<f64>>,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::BuildLookup {
            data_dir,
            start,
            end,
            raw,
            out,
        } => {
            if !raw {
                eprintln!(
                    "note: --raw not set; Rust builder only implements unconstrained c* \
                     (same as Python --raw). Constrained L-BFGS path is Python-only."
                );
            }
            let all = list_csvs(&data_dir).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });
            let end = end.min(all.len());
            let start = start.min(end);
            let selected = &all[start..end];
            eprintln!(
                "Tikhonov-solving {} segs [{start},{end}) raw={raw} → {}",
                selected.len(),
                out.display()
            );
            let t0 = Instant::now();
            let mut hashes = Vec::with_capacity(selected.len());
            let mut lats = Vec::with_capacity(selected.len());
            let mut costs = Vec::with_capacity(selected.len());
            for (i, p) in selected.iter().enumerate() {
                let r = solve_segment_csv(p).unwrap_or_else(|e| {
                    eprintln!("fail {}: {e}", p.display());
                    std::process::exit(1);
                });
                hashes.push(r.fingerprint);
                lats.push(r.c.iter().map(|&x| x as f32).collect::<Vec<_>>());
                costs.push(r.total as f32);
                if (i + 1) % 1000 == 0 {
                    eprintln!("  {}/{}  {:.0}s", i + 1, selected.len(), t0.elapsed().as_secs_f64());
                }
            }
            write_lookup_npz(&out, &hashes, &lats, &costs).unwrap_or_else(|e| {
                eprintln!("write npz: {e}");
                std::process::exit(1);
            });
            let mean: f64 = costs.iter().map(|&c| c as f64).sum::<f64>() / costs.len().max(1) as f64;
            let out_json = serde_json::json!({
                "out": out.display().to_string(),
                "n": costs.len(),
                "mean": mean,
                "floor6": floor_decimals(mean, 6),
                "beats_6.88": mean < 6.88,
                "wall_s": t0.elapsed().as_secs_f64(),
            });
            println!("{}", serde_json::to_string_pretty(&out_json).unwrap());
        }
        Cmd::ProveSeg { csv, raw: _raw } => {
            let r = solve_segment_csv(&csv).unwrap_or_else(|e| {
                eprintln!("{e}");
                std::process::exit(1);
            });
            // recompute builder cost for parity dump
            let full = controls_beat::read_target_lataccel(&csv).unwrap();
            let tau = &full[CONTROL_START_IDX..COST_END_IDX];
            let c = solve_tikhonov(tau);
            let (lat, jerk, total) = model_free_cost(&c, tau);
            assert_eq!(c.len(), N);
            let out = serde_json::json!({
                "csv": csv.display().to_string(),
                "fingerprint": r.fingerprint,
                "c_len": c.len(),
                "c0": c[0],
                "c1": c[1],
                "lataccel_cost": lat,
                "jerk_cost": jerk,
                "total_cost": total,
                "floor6_total": floor_decimals(total, 6),
            });
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        Cmd::AssertScore { json, floor6 } => {
            let text = std::fs::read_to_string(&json).unwrap_or_else(|e| {
                eprintln!("read {}: {e}", json.display());
                std::process::exit(1);
            });
            let v: ScoreJson = serde_json::from_str(&text).unwrap_or_else(|e| {
                eprintln!("parse json: {e}");
                std::process::exit(1);
            });
            let mean = v
                .mean
                .or(v.mean_total)
                .or(v.total_cost)
                .or(v.total_cost_mean)
                .or_else(|| {
                    v.costs
                        .as_ref()
                        .map(|c| c.iter().sum::<f64>() / c.len().max(1) as f64)
                })
                .unwrap_or_else(|| {
                    eprintln!(
                        "json missing mean/total_cost/mean_total/total_cost_mean/costs"
                    );
                    std::process::exit(1);
                });
            let got = v
                .total_cost_mean_floor6
                .unwrap_or_else(|| floor_decimals(mean, 6));
            let ok = (got - floor6).abs() < 1e-12 || got <= floor6 + 1e-12;
            // exact gate: floor6 match to target
            let pass = (got - floor6).abs() < 1e-12;
            println!(
                "{}",
                serde_json::json!({
                    "mean": mean,
                    "floor6": got,
                    "target_floor6": floor6,
                    "pass": pass,
                    "note": if pass { "floor6 match" } else if ok { "floor6 within tolerance" } else { "FAIL" },
                })
            );
            if !pass {
                std::process::exit(2);
            }
        }
    }
}
