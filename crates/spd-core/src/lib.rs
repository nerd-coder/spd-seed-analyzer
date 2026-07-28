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
pub mod quests;
pub mod random;
pub mod report;
pub mod rooms;
pub mod run;
pub mod search;
pub mod trinkets;

pub use dungeon_seed::{DungeonSeed, SeedError, TOTAL_SEEDS};
pub use items::IdentityMaps;
pub use java_random::JavaRandom;
pub use random::Random;
pub use report::{
    AnalyzeError, FloorReport, GuaranteedAppearance, GuaranteedAppearanceKind, SeedInfo, SeedReport,
};
pub use run::{dungeon_from_run, init_run, RunState};
pub use search::{
    search_seeds, ItemConstraint, ItemMatchEvidence, MatchMode, SearchError, SeedMatch,
    SeedSearchRequest, SeedSearchResult, MAX_SEARCH_CANDIDATES, MAX_SEARCH_CONSTRAINTS,
    MAX_SEARCH_MATCHES,
};
pub use trinkets::{
    HeldTrinketProfile, MapMetaProfile, MapProfile, MapTrinketProfile, ProfileError,
    TrinketSelectionReport,
};

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
    analyze_seed_with_profile(input, floors, None)
}

/// Analyze with explicit run assumptions for generation and map projection.
pub fn analyze_seed_with_profile(
    input: &str,
    floors: u32,
    profile: Option<MapProfile>,
) -> Result<SeedReport, AnalyzeError> {
    let info = parse_seed(input)?;
    let floors = floors.clamp(1, 26);
    let run = init_run(info.numeric);
    let trinket_selection = TrinketSelectionReport::for_run(&run);
    if let Some(profile) = profile.as_ref() {
        profile.validate(trinket_selection.first_effective_depth)?;
    }
    let mut dungeon = dungeon_from_run(run);
    let mut layout_dungeon = dungeon.clone();
    let identities = dungeon.identities.clone();
    let mut floor_reports =
        level::analyze_floors_with_profile(&mut dungeon, floors, profile.as_ref());
    if let Some(profile) = profile.as_ref() {
        let layouts = level::analyze_layouts_with_profile(&mut layout_dungeon, floors, profile);
        for (report, layout) in floor_reports.iter_mut().zip(layouts) {
            report.feeling = layout.feeling;
            report.builder = layout.builder;
            report.rooms = layout.rooms;
            report.guaranteed_appearances = layout.guaranteed_appearances;
            report.map = layout.map;
            report.assumed_map = layout.assumed_map;
        }
    }
    if let Some(floor) = floor_reports
        .iter_mut()
        .find(|floor| floor.depth == trinket_selection.first_alchemy_pot_depth)
    {
        let source = if trinket_selection.first_alchemy_pot_is_secret {
            "SecretLaboratoryRoom"
        } else {
            "LaboratoryRoom"
        };
        if !floor
            .guaranteed_appearances
            .iter()
            .any(|appearance| appearance.source.as_deref() == Some(source))
        {
            floor.guaranteed_appearances.push(GuaranteedAppearance {
                name: "Alchemy pot".into(),
                kind: GuaranteedAppearanceKind::AlchemyPot,
                source: Some(source.into()),
            });
        }
    }

    Ok(SeedReport {
        seed: info,
        spd_version: SPD_VERSION.to_string(),
        spd_commit: SPD_COMMIT.to_string(),
        floors_requested: floors,
        identities,
        trinket_selection,
        floors: floor_reports,
        status: "partial".to_string(),
        message: Some(
            "Analysis accuracy is partial; results may differ from the pinned game.".to_string(),
        ),
        map_profile: profile,
    })
}

#[cfg(test)]
#[path = "analyze_smoke.rs"]
mod analyze_smoke;
