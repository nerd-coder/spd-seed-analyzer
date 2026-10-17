//! Timed engine workloads: report generation, JSON projection, and search.

use std::time::Instant;

use serde::Serialize;
use spd_core::{analyze_seed, analyze_seed_exact, search_seeds, ExactOptions, SeedSearchRequest};

use crate::alloc;

#[derive(Serialize)]
pub struct Timing {
    pub mean_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
}

impl Timing {
    fn from_samples(mut samples: Vec<f64>) -> Self {
        samples.sort_by(f64::total_cmp);
        let pick = |q: f64| samples[((samples.len() - 1) as f64 * q).round() as usize];
        Self {
            mean_ms: samples.iter().sum::<f64>() / samples.len() as f64,
            p50_ms: pick(0.5),
            p95_ms: pick(0.95),
            min_ms: samples[0],
            max_ms: samples[samples.len() - 1],
        }
    }
}

#[derive(Serialize)]
pub struct Memory {
    pub peak_heap_bytes: usize,
    pub allocations_per_seed: f64,
    pub allocated_bytes_per_seed: f64,
}

impl Memory {
    fn from_snapshot(snapshot: alloc::Snapshot, seeds: usize) -> Self {
        Self {
            peak_heap_bytes: snapshot.peak_bytes,
            allocations_per_seed: snapshot.allocations as f64 / seeds as f64,
            allocated_bytes_per_seed: snapshot.allocated_bytes as f64 / seeds as f64,
        }
    }
}

#[derive(Serialize)]
pub struct Workload {
    pub name: String,
    pub floors: u32,
    pub seeds: usize,
    /// Generation only.
    pub generate: Timing,
    /// `serde_json::to_string` of the generated report (the WASM boundary cost).
    pub serialize: Timing,
    pub json_bytes_per_seed: f64,
    pub memory: Memory,
}

#[derive(Serialize)]
pub struct Search {
    pub name: String,
    pub floors: u32,
    pub candidates: u32,
    pub seeds_per_second: f64,
    pub ms_per_seed: f64,
    pub matches: usize,
    pub memory: Memory,
}

pub fn seeds(count: usize) -> Vec<String> {
    (0..count as u64)
        .map(|i| (i * 7_919_993 + 1).to_string())
        .collect()
}

fn workload<R: Serialize>(
    name: &str,
    floors: u32,
    seeds: &[String],
    generate: impl Fn(&str) -> R,
) -> Workload {
    let baseline = alloc::reset();
    let mut generation = Vec::with_capacity(seeds.len());
    let mut serialization = Vec::with_capacity(seeds.len());
    let mut bytes = 0usize;
    for seed in seeds {
        let start = Instant::now();
        let report = generate(seed);
        generation.push(start.elapsed().as_secs_f64() * 1e3);
        let start = Instant::now();
        let json = serde_json::to_string(&report).expect("report serializes");
        serialization.push(start.elapsed().as_secs_f64() * 1e3);
        bytes += json.len();
    }
    Workload {
        name: name.to_string(),
        floors,
        seeds: seeds.len(),
        generate: Timing::from_samples(generation),
        serialize: Timing::from_samples(serialization),
        json_bytes_per_seed: bytes as f64 / seeds.len() as f64,
        memory: Memory::from_snapshot(alloc::snapshot(baseline), seeds.len()),
    }
}

pub fn workloads(seeds: &[String]) -> Vec<Workload> {
    let mut out = Vec::new();
    for floors in [1, 5, 26] {
        out.push(workload("full report", floors, seeds, |seed| {
            analyze_seed(seed, floors).expect("analyze")
        }));
    }
    for (name, maps) in [("exact report", false), ("exact report + maps", true)] {
        out.push(workload(name, 26, seeds, |seed| {
            analyze_seed_exact(seed, 26, ExactOptions { maps }).expect("analyze exact")
        }));
    }
    out
}

pub fn searches(scale: f64) -> Vec<Search> {
    [(5u32, 2_000f64), (26, 300.0)]
        .into_iter()
        .map(|(floors, candidates)| {
            let candidates = (candidates * scale).max(10.0) as u32;
            let request: SeedSearchRequest = serde_json::from_value(serde_json::json!({
                "startSeed": 1,
                "candidateCount": candidates,
                "floors": floors,
                "matchMode": "all",
                "maxMatches": 100,
                "constraints": [{"itemGroup": "ring", "minLevel": 4, "minDepth": 1, "maxDepth": floors}],
            }))
            .expect("valid search request");
            let baseline = alloc::reset();
            let start = Instant::now();
            let result = search_seeds(&request).expect("search");
            let seconds = start.elapsed().as_secs_f64();
            let scanned = result.candidates_scanned.max(1) as usize;
            Search {
                name: "search: ring +4".into(),
                floors,
                candidates,
                seeds_per_second: scanned as f64 / seconds,
                ms_per_seed: seconds * 1e3 / scanned as f64,
                matches: result.matches.len(),
                memory: Memory::from_snapshot(alloc::snapshot(baseline), scanned),
            }
        })
        .collect()
}
