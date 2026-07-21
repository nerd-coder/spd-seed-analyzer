//! Pure Rust port of Shattered Pixel Dungeon generation logic needed for seed analysis.
//!
//! Target game source: local clone of 00-Evan/shattered-pixel-dungeon (see README).

pub mod builders;
pub mod dungeon;
pub mod dungeon_seed;
pub mod generator;
pub mod geom;
pub mod items;
pub mod java_random;
pub mod level;
pub mod random;
pub mod report;
pub mod rooms;
pub mod run;

pub use dungeon_seed::{DungeonSeed, SeedError, TOTAL_SEEDS};
pub use items::IdentityMaps;
pub use java_random::JavaRandom;
pub use random::Random;
pub use report::{AnalyzeError, FloorReport, SeedInfo, SeedReport};
pub use run::{RunState, dungeon_from_run, init_run};

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
/// Returns identity maps plus **partial** per-floor data (forced drops / feelings).
/// Full room loot is not yet ported.
pub fn analyze_seed(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    let info = parse_seed(input)?;
    let floors = floors.clamp(1, 26);
    let run = init_run(info.numeric);
    let mut dungeon = dungeon_from_run(run);
    let identities = dungeon.identities.clone();
    let floor_reports = level::analyze_floors(&mut dungeon, floors);

    Ok(SeedReport {
        seed: info,
        spd_version: SPD_VERSION.to_string(),
        spd_commit: SPD_COMMIT.to_string(),
        floors_requested: floors,
        identities,
        floors: floor_reports,
        status: "partial".to_string(),
        message: Some(
            "Partial analysis: room layout builder + main createItems drops. Special-room paint loot, quests, and full painter traps/water still incomplete — results may not match the game yet."
                .to_string(),
        ),
    })
}

#[cfg(test)]
mod analyze_smoke {
    use super::*;

    #[test]
    fn analyze_seed_smoke() {
        let r = analyze_seed("GFX-PZH-DCH", 4).expect("analyze");
        eprintln!("status={} floors={}", r.status, r.floors.len());
        for f in &r.floors {
            eprintln!("  floor {} rooms={} items={} map={:?}", f.depth, f.rooms.len(), f.items.len(), f.map.as_ref().map(|m| (m.width, m.height, m.tileset.as_str())));
        }
    }

    #[test]
    fn analyze_several_seeds() {
        for s in ["AAA-AAA-AAA", "JLY-ZYR-HET", "hello", "42"] {
            let r = analyze_seed(s, 6);
            assert!(r.is_ok(), "seed {s}: {:?}", r.err());
        }
    }
}
