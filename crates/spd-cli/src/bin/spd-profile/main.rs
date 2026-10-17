//! `spd-profile [--seeds N] [--dump-report PATH] [--loop SECONDS] | --demangle`
//!
//! Benchmarks `spd-core` and prints the measurements as JSON. `--loop` runs
//! only the 26-floor full report for a sampling profiler; `--demangle` turns
//! sampler symbols on stdin into readable paths. `tools/profile` drives this
//! binary and renders the HTML report.

mod alloc;
mod bench;

use std::process::ExitCode;
use std::time::{Duration, Instant};

use serde::Serialize;

#[global_allocator]
static ALLOCATOR: alloc::Counting = alloc::Counting;

const USAGE: &str =
    "usage: spd-profile [--seeds N] [--dump-report PATH] [--loop SECONDS] | --demangle";

#[derive(Serialize)]
struct Output {
    seeds: usize,
    workloads: Vec<bench::Workload>,
    searches: Vec<bench::Search>,
}

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some("--demangle") {
        return demangle_stdin();
    }
    let mut seeds = 100usize;
    let mut dump = None;
    let mut loop_seconds = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args.next();
        let parsed = match (arg.as_str(), value) {
            ("--seeds", Some(v)) => v.parse().map(|n| seeds = n).is_ok(),
            ("--dump-report", Some(v)) => {
                dump = Some(v);
                true
            }
            ("--loop", Some(v)) => v.parse().map(|s: f64| loop_seconds = Some(s)).is_ok(),
            _ => false,
        };
        if !parsed || seeds == 0 {
            eprintln!("{USAGE}");
            return ExitCode::FAILURE;
        }
    }
    let seed_list = bench::seeds(seeds);

    if let Some(seconds) = loop_seconds {
        let deadline = Instant::now() + Duration::from_secs_f64(seconds);
        for seed in seed_list.iter().cycle() {
            if Instant::now() >= deadline {
                break;
            }
            std::hint::black_box(spd_core::analyze_seed(seed, 26).expect("analyze"));
        }
        return ExitCode::SUCCESS;
    }

    if let Some(path) = dump {
        let report = spd_core::analyze_seed(&seed_list[0], 26).expect("analyze");
        let json = serde_json::to_string(&report).expect("report serializes");
        if let Err(error) = std::fs::write(&path, json) {
            eprintln!("write {path}: {error}");
            return ExitCode::FAILURE;
        }
    }

    let output = Output {
        seeds,
        workloads: bench::workloads(&seed_list),
        searches: bench::searches(seeds as f64 / 100.0),
    };
    println!(
        "{}",
        serde_json::to_string(&output).expect("output serializes")
    );
    ExitCode::SUCCESS
}

/// Samplers print raw v0 symbols; demangle one per line without hashes.
fn demangle_stdin() -> ExitCode {
    for line in std::io::stdin().lines().map_while(Result::ok) {
        let symbol = line.trim().trim_start_matches('_');
        match rustc_demangle::try_demangle(&format!("_{symbol}")) {
            Ok(name) => println!("{name:#}"),
            Err(_) => println!("{}", line.trim()),
        }
    }
    ExitCode::SUCCESS
}
