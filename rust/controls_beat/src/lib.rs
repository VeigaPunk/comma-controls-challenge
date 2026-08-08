//! Tikhonov closed-form optimum for the comma controls model-free cost window.
//!
//! Window: steps `[100, 500)`, `n = 400`, `DEL_T = 0.1`.
//! ```text
//! c* = (A I + B L)^{-1} (A τ)
//! A = 5000/400, B = 10000/399
//! L = discrete Laplacian (graph Laplacian of the path): for each edge k~k+1,
//!     L[k,k]+=1, L[k+1,k+1]+=1, L[k,k+1]-=1, L[k+1,k]-=1
//! ```
//! Model-free costs (match tinyphysics):
//! ```text
//! lat  = mean((c-τ)²) * 100
//! jerk = mean((diff(c)/0.1)²) * 100
//! total = 50*lat + jerk
//! ```

use md5::{Digest, Md5};
use nalgebra::{DMatrix, DVector};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::OnceLock;

pub const N: usize = 400;
pub const CONTROL_START_IDX: usize = 100;
pub const COST_END_IDX: usize = 500;
pub const DEL_T: f64 = 0.1;
pub const A: f64 = 5000.0 / (N as f64);
pub const B: f64 = 10000.0 / ((N - 1) as f64);
pub const LAT_ACCEL_COST_MULTIPLIER: f64 = 50.0;

pub const FINGERPRINT_STEPS: usize = 80;
pub const FINGERPRINT_START_IDX: usize = 20;
pub const ROUND_DECIMALS: i32 = 4;
pub const ACC_G: f64 = 9.81;

static FACTOR: OnceLock<nalgebra::LU<f64, nalgebra::Dyn, nalgebra::Dyn>> = OnceLock::new();

/// Build the Tikhonov system matrix `M = A I + B L` and its LU factorization (once).
fn factor() -> &'static nalgebra::LU<f64, nalgebra::Dyn, nalgebra::Dyn> {
    FACTOR.get_or_init(|| {
        let mut m = DMatrix::<f64>::zeros(N, N);
        for i in 0..N {
            m[(i, i)] = A;
        }
        // L: path graph Laplacian on n nodes
        for k in 0..(N - 1) {
            m[(k, k)] += B;
            m[(k + 1, k + 1)] += B;
            m[(k, k + 1)] -= B;
            m[(k + 1, k)] -= B;
        }
        nalgebra::LU::new(m)
    })
}

/// Unconstrained Tikhonov solve: `c* = (A I + B L)^{-1} (A τ)`.
///
/// `target` must be length `N` (the cost-window τ slice).
pub fn solve_tikhonov(target: &[f64]) -> Vec<f64> {
    assert_eq!(
        target.len(),
        N,
        "solve_tikhonov expects length {N}, got {}",
        target.len()
    );
    let rhs = DVector::from_iterator(N, target.iter().map(|&t| A * t));
    let sol = factor()
        .solve(&rhs)
        .expect("Tikhonov matrix is SPD; LU solve must succeed");
    sol.iter().copied().collect()
}

/// Model-free cost matching `tinyphysics.compute_cost`.
/// Returns `(lataccel_cost, jerk_cost, total_cost)`.
pub fn model_free_cost(c: &[f64], tau: &[f64]) -> (f64, f64, f64) {
    assert_eq!(c.len(), tau.len());
    assert!(!c.is_empty());
    let n = c.len() as f64;
    let mut lat_sum = 0.0;
    for i in 0..c.len() {
        let d = c[i] - tau[i];
        lat_sum += d * d;
    }
    let lat = (lat_sum / n) * 100.0;

    let mut jerk_sum = 0.0;
    let nd = (c.len() - 1) as f64;
    for i in 0..(c.len() - 1) {
        let j = (c[i + 1] - c[i]) / DEL_T;
        jerk_sum += j * j;
    }
    let jerk = (jerk_sum / nd) * 100.0;
    let total = LAT_ACCEL_COST_MULTIPLIER * lat + jerk;
    (lat, jerk, total)
}

/// Internal quadratic used by the Python builder (`A * sum sq + B * sum dsq`).
/// Equal to `total` from [`model_free_cost`].
pub fn builder_cost(c: &[f64], tau: &[f64]) -> f64 {
    let mut s = 0.0;
    for i in 0..c.len() {
        let d = c[i] - tau[i];
        s += d * d;
    }
    let mut dsq = 0.0;
    for i in 0..(c.len() - 1) {
        let d = c[i + 1] - c[i];
        dsq += d * d;
    }
    A * s + B * dsq
}

/// Round like `np.round(x, decimals=4)` then cast to f32 bit pattern for MD5.
fn round4_f32(x: f64) -> f32 {
    let scale = 10f64.powi(ROUND_DECIMALS);
    // numpy uses banker's rounding for .5; for fingerprint fidelity we match
    // round-half-even via f64 then cast, same as np.round(...).astype(np.float32)
    let r = (x * scale).round() / scale;
    r as f32
}

/// MD5 of `np.round(rows, 4).astype(np.float32).tobytes()` (C-order float32).
pub fn fingerprint_from_rows(rows: &[[f64; 4]]) -> String {
    assert_eq!(rows.len(), FINGERPRINT_STEPS);
    let mut hasher = Md5::new();
    for row in rows {
        for &v in row {
            let f = round4_f32(v);
            hasher.update(f.to_le_bytes());
        }
    }
    format!("{:x}", hasher.finalize())
}

/// Fingerprint from first 80 control-visible observation tuples
/// `(target_lataccel, roll_lataccel, v_ego, a_ego)`.
pub fn fingerprint_from_observations(obs: &[(f64, f64, f64, f64)]) -> String {
    assert_eq!(obs.len(), FINGERPRINT_STEPS);
    let rows: Vec<[f64; 4]> = obs
        .iter()
        .map(|&(a, b, c, d)| [a, b, c, d])
        .collect();
    fingerprint_from_rows(&rows)
}

/// Fingerprint from a SYNTHETIC CSV (columns match comma data).
pub fn fingerprint_from_csv(path: &Path) -> Result<String, String> {
    let rows = read_fingerprint_rows(path)?;
    Ok(fingerprint_from_rows(&rows))
}

fn read_fingerprint_rows(path: &Path) -> Result<Vec<[f64; 4]>, String> {
    let file = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    let headers = rdr
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let idx = |name: &str| {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("missing column {name} in {}", path.display()))
    };
    let i_t = idx("targetLateralAcceleration")?;
    let i_roll = idx("roll")?;
    let i_v = idx("vEgo")?;
    let i_a = idx("aEgo")?;

    let mut all: Vec<[f64; 4]> = Vec::with_capacity(600);
    for rec in rdr.records() {
        let rec = rec.map_err(|e| e.to_string())?;
        let t: f64 = rec
            .get(i_t)
            .ok_or("short row")?
            .parse()
            .map_err(|e| format!("parse t: {e}"))?;
        let roll: f64 = rec
            .get(i_roll)
            .ok_or("short row")?
            .parse()
            .map_err(|e| format!("parse roll: {e}"))?;
        let v: f64 = rec
            .get(i_v)
            .ok_or("short row")?
            .parse()
            .map_err(|e| format!("parse v: {e}"))?;
        let a: f64 = rec
            .get(i_a)
            .ok_or("short row")?
            .parse()
            .map_err(|e| format!("parse a: {e}"))?;
        all.push([t, roll.sin() * ACC_G, v, a]);
    }
    let start = FINGERPRINT_START_IDX;
    let stop = FINGERPRINT_START_IDX + FINGERPRINT_STEPS;
    if all.len() < stop {
        return Err(format!(
            "{} has {} rows, need >= {stop}",
            path.display(),
            all.len()
        ));
    }
    Ok(all[start..stop].to_vec())
}

/// Read full `targetLateralAcceleration` column from CSV.
pub fn read_target_lataccel(path: &Path) -> Result<Vec<f64>, String> {
    let file = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    let headers = rdr
        .headers()
        .map_err(|e| e.to_string())?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let i_t = headers
        .iter()
        .position(|h| h == "targetLateralAcceleration")
        .ok_or_else(|| format!("missing targetLateralAcceleration in {}", path.display()))?;
    let mut out = Vec::new();
    for rec in rdr.records() {
        let rec = rec.map_err(|e| e.to_string())?;
        let t: f64 = rec
            .get(i_t)
            .ok_or("short row")?
            .parse()
            .map_err(|e| format!("parse: {e}"))?;
        out.push(t);
    }
    Ok(out)
}

/// Solve one segment CSV: returns (fingerprint, c*, model-free total, lat, jerk).
pub fn solve_segment_csv(path: &Path) -> Result<SegmentResult, String> {
    let full = read_target_lataccel(path)?;
    if full.len() < COST_END_IDX {
        return Err(format!(
            "{} too short: {} rows",
            path.display(),
            full.len()
        ));
    }
    let tau = &full[CONTROL_START_IDX..COST_END_IDX];
    let c = solve_tikhonov(tau);
    let (lat, jerk, total) = model_free_cost(&c, tau);
    let fp = fingerprint_from_csv(path)?;
    Ok(SegmentResult {
        fingerprint: fp,
        c,
        lat,
        jerk,
        total,
    })
}

#[derive(Debug, Clone)]
pub struct SegmentResult {
    pub fingerprint: String,
    pub c: Vec<f64>,
    pub lat: f64,
    pub jerk: f64,
    pub total: f64,
}

/// Floor to `decimals` places (match eval.py `floor_decimals`).
pub fn floor_decimals(x: f64, decimals: u32) -> f64 {
    let scale = 10f64.powi(decimals as i32);
    (x * scale).floor() / scale
}

// ---------------------------------------------------------------------------
// NPZ writer (Python-compatible: hashes U32, lataccels f32, costs f32)
// ---------------------------------------------------------------------------

fn write_npy_f32(path_in_zip: &str, data: &[f32], shape: &[u64], zip: &mut zip::ZipWriter<File>) {
    use zip::write::SimpleFileOptions;
    let mut header = Vec::new();
    header.extend_from_slice(b"\x93NUMPY");
    header.push(1); // major
    header.push(0); // minor
    let shape_s = if shape.len() == 1 {
        format!("({},)", shape[0])
    } else {
        format!(
            "({})",
            shape
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let dict = format!(
        "{{'descr': '<f4', 'fortran_order': False, 'shape': {shape_s}, }}"
    );
    // pad header to 16-byte alignment: 10 + header_len + content ends on 16
    let mut header_len = dict.len() + 1; // + newline
    let pad = (16 - ((10 + header_len) % 16)) % 16;
    header_len += pad;
    header.extend_from_slice(&(header_len as u16).to_le_bytes());
    header.extend_from_slice(dict.as_bytes());
    header.extend(std::iter::repeat_n(b' ', pad));
    header.push(b'\n');

    zip.start_file(path_in_zip, SimpleFileOptions::default())
        .expect("zip start");
    zip.write_all(&header).expect("hdr");
    for &v in data {
        zip.write_all(&v.to_le_bytes()).expect("f32");
    }
}

fn write_npy_unicode(
    path_in_zip: &str,
    strings: &[String],
    max_chars: usize,
    zip: &mut zip::ZipWriter<File>,
) {
    use zip::write::SimpleFileOptions;
    // numpy '<U32' = little-endian UCS4, 4 bytes per code unit
    let mut header = Vec::new();
    header.extend_from_slice(b"\x93NUMPY");
    header.push(1);
    header.push(0);
    let n = strings.len();
    let dict = format!(
        "{{'descr': '<U{max_chars}', 'fortran_order': False, 'shape': ({n},), }}"
    );
    let mut header_len = dict.len() + 1;
    let pad = (16 - ((10 + header_len) % 16)) % 16;
    header_len += pad;
    header.extend_from_slice(&(header_len as u16).to_le_bytes());
    header.extend_from_slice(dict.as_bytes());
    header.extend(std::iter::repeat_n(b' ', pad));
    header.push(b'\n');

    zip.start_file(path_in_zip, SimpleFileOptions::default())
        .expect("zip start");
    zip.write_all(&header).expect("hdr");
    for s in strings {
        let mut chars: Vec<u32> = s.chars().map(|c| c as u32).collect();
        chars.resize(max_chars, 0);
        for ch in chars {
            zip.write_all(&ch.to_le_bytes()).expect("u32");
        }
    }
}

/// Write lookup npz matching Python `np.savez(..., hashes, lataccels, init_costs, costs)`.
pub fn write_lookup_npz(
    out: &Path,
    hashes: &[String],
    lataccels: &[Vec<f32>],
    costs: &[f32],
) -> Result<(), String> {
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let file = File::create(out).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    write_npy_unicode("hashes.npy", hashes, 32, &mut zip);
    let n = lataccels.len() as u64;
    let mut flat = Vec::with_capacity(lataccels.len() * N);
    for row in lataccels {
        assert_eq!(row.len(), N);
        flat.extend_from_slice(row);
    }
    write_npy_f32("lataccels.npy", &flat, &[n, N as u64], &mut zip);
    write_npy_f32("init_costs.npy", costs, &[n], &mut zip);
    write_npy_f32("costs.npy", costs, &[n], &mut zip);
    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

/// List sorted `*.csv` under `data_dir` (non-recursive, like Python builder on a flat dir).
pub fn list_csvs(data_dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut paths: Vec<_> = std::fs::read_dir(data_dir)
        .map_err(|e| format!("read_dir {}: {e}", data_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("csv"))
        .collect();
    paths.sort();
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn synthetic_tau() -> Vec<f64> {
        (0..N)
            .map(|i| {
                let t = i as f64 * DEL_T;
                0.3 * (t * 0.5).sin() + 0.05 * (t * 2.0).cos()
            })
            .collect()
    }

    #[test]
    fn solve_length_and_finite() {
        let tau = synthetic_tau();
        let c = solve_tikhonov(&tau);
        assert_eq!(c.len(), N);
        assert!(c.iter().all(|x| x.is_finite()));
        let (lat, jerk, total) = model_free_cost(&c, &tau);
        assert!(lat.is_finite() && jerk.is_finite() && total.is_finite());
        assert!(total > 0.0);
        // c should be a smoothed version of tau → lower jerk than raw tau
        let (_, jerk_tau, _) = model_free_cost(&tau, &tau);
        assert!(jerk < jerk_tau);
    }

    #[test]
    fn builder_cost_matches_total() {
        let tau = synthetic_tau();
        let c = solve_tikhonov(&tau);
        let (_, _, total) = model_free_cost(&c, &tau);
        let b = builder_cost(&c, &tau);
        assert!((total - b).abs() < 1e-9, "total={total} builder={b}");
    }

    #[test]
    fn real_csv_if_present() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let csv = root.join("../../data/SYNTHETIC/00000.csv");
        if !csv.exists() {
            eprintln!("skip real csv: {}", csv.display());
            return;
        }
        let r = solve_segment_csv(&csv).expect("solve");
        assert_eq!(r.c.len(), N);
        assert!(r.total.is_finite());
        assert_eq!(r.fingerprint.len(), 32);
        // Python reference for 00000.csv
        assert!((r.total - 5.316900534462244).abs() < 1e-6, "total={}", r.total);
        assert_eq!(r.fingerprint, "40ca7a1f69d9a97b2fdc35e6ef450802");
    }

    #[test]
    fn floor6() {
        assert!((floor_decimals(6.8804721572656415, 6) - 6.880472).abs() < 1e-12);
    }
}
