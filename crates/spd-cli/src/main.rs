//! `spd-analyze SEED [FLOORS] [--maps] [--pretty] [--full]`
//!
//! Prints the concise exact report (default profile) as JSON. `--full`
//! prints the public `SeedReport` with its prediction kinds instead.

use std::process::ExitCode;

use spd_core::{analyze_seed, analyze_seed_exact, ExactOptions};

const USAGE: &str = "usage: spd-analyze SEED [FLOORS=26] [--maps] [--pretty] [--full]";

fn main() -> ExitCode {
    let mut seed = None;
    let mut floors = 26;
    let (mut maps, mut pretty, mut full) = (false, false, false);
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--maps" => maps = true,
            "--pretty" => pretty = true,
            "--full" => full = true,
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            _ if seed.is_none() => seed = Some(arg),
            _ => match arg.parse() {
                Ok(value) => floors = value,
                Err(_) => {
                    eprintln!("invalid floor count `{arg}`\n{USAGE}");
                    return ExitCode::FAILURE;
                }
            },
        }
    }
    let Some(seed) = seed else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };
    let json = if full {
        analyze_seed(&seed, floors).map(|report| to_json(&report, pretty))
    } else {
        analyze_seed_exact(&seed, floors, ExactOptions { maps }).map(|r| to_json(&r, pretty))
    };
    match json {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn to_json<T: serde::Serialize>(value: &T, pretty: bool) -> String {
    let result = if pretty {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    };
    result.expect("reports serialize")
}
