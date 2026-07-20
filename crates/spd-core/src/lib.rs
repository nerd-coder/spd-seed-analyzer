//! Pure Rust port of Shattered Pixel Dungeon generation logic needed for seed analysis.
//!
//! Target game source: local clone of 00-Evan/shattered-pixel-dungeon (see README).

pub mod dungeon_seed;
pub mod java_random;
pub mod random;
pub mod report;

pub use dungeon_seed::{DungeonSeed, SeedError, TOTAL_SEEDS};
pub use java_random::JavaRandom;
pub use random::Random;
pub use report::{AnalyzeError, FloorReport, SeedInfo, SeedReport};

/// Pinned SPD version this port targets (from local clone at scaffold time).
pub const SPD_VERSION: &str = "v3.3.8";
pub const SPD_COMMIT: &str = "7b8b845a7";

/// Parse a user seed string into display info (no levelgen).
pub fn parse_seed(input: &str) -> Result<SeedInfo, SeedError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SeedError::Empty);
    }
    let numeric = DungeonSeed::convert_from_text(trimmed)?;
    let code = if numeric < TOTAL_SEEDS {
        DungeonSeed::convert_to_code(numeric).ok()
    } else {
        // Daily seeds sit outside TOTAL_SEEDS; no ABC-DEF-GHI form.
        None
    };
    Ok(SeedInfo {
        input: trimmed.to_string(),
        numeric,
        code,
        formatted: DungeonSeed::format_text(trimmed),
    })
}

/// Analyze a seed for the given number of floors.
///
/// Currently returns scaffold data (seed info only). Levelgen lands in later phases.
pub fn analyze_seed(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    let info = parse_seed(input)?;
    let floors = floors.clamp(1, 26);
    Ok(SeedReport {
        seed: info,
        spd_version: SPD_VERSION.to_string(),
        spd_commit: SPD_COMMIT.to_string(),
        floors_requested: floors,
        floors: Vec::new(),
        status: "scaffold".to_string(),
        message: Some(format!(
            "Seed parsed. Per-floor item listing not yet implemented (requested {floors} floors)."
        )),
    })
}
