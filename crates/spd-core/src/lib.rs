//! Pure Rust port of Shattered Pixel Dungeon generation logic needed for seed analysis.
//!
//! Target game source: local clone of 00-Evan/shattered-pixel-dungeon (see README).

pub mod builders;
mod conditional;
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
    AmbitiousImpQuestBaseline, AmbitiousImpQuestContract, AnalyzeError, BlacksmithObjective,
    FloorReport, GhostTarget, GuaranteedAppearance, GuaranteedAppearanceKind, ImpTarget,
    ImpTargetRule, ItemCondition, ItemDependencyCondition, ItemEnchantment, ItemSpawnCondition,
    OldWandmakerQuestBaseline, OldWandmakerQuestContract, QuestDepthRange, QuestReport,
    QuestRewardSelection, SadGhostQuestBaseline, SadGhostQuestContract, SeedInfo, SeedReport,
    TrollBlacksmithQuestBaseline, TrollBlacksmithQuestContract, WandmakerObjective,
};
pub use run::{dungeon_from_run, init_run, RunState};
pub use search::{
    search_seeds, ItemConstraint, ItemMatchEvidence, MatchMode, SearchError, SeedMatch,
    SeedSearchRequest, SeedSearchResult, MAX_SEARCH_CANDIDATES, MAX_SEARCH_CONSTRAINTS,
    MAX_SEARCH_MATCHES,
};
pub use trinkets::{
    ArtifactEvent, ArtifactEventAction, ArtifactKind, Challenge, ClaimState, MapProfile,
    ProfileError, TrinketEvent, TrinketEventAction, TrinketKind, TrinketSelectionReport,
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

/// Analyze a seed and merge every currently supported item-generating profile
/// into each floor's item list with structured spawn conditions.
pub fn analyze_seed(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    let baseline_profile = MapProfile::default();
    let mut report = analyze_seed_internal(input, floors, Some(&baseline_profile))?;
    let routes = conditional::discover_routes(
        &report.trinket_selection,
        &report.floors,
        report.floors_requested,
    );
    let mut alternatives = Vec::new();
    for route in routes {
        let alternative = analyze_seed_internal(input, route.max_depth, Some(&route.profile))?;
        alternatives.push((route, alternative.floors));
    }
    conditional::merge_possible_rooms(
        &mut report.floors,
        conditional::baseline_condition(),
        &alternatives,
    );
    conditional::merge_possible_items(
        &mut report.floors,
        conditional::baseline_condition(),
        &alternatives,
    );
    // Parity probes retain the latest replay trace for oracle inspection. Keep
    // that diagnostic state on the conservative finder projection rather than
    // whichever conditional route happened to be discovered last.
    let _ = analyze_seed_internal(input, floors, None)?;
    Ok(report)
}

/// Seed-only projection used by the bounded finder. It deliberately avoids
/// replaying the UI-facing modeled branch matrix for every candidate seed.
pub(crate) fn analyze_seed_seed_only(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    // A seed-only search still has a concrete baseline: no challenges, held
    // trinket, or prior player history. Replaying that profile prevents the
    // default-feeling callback from tainting all later layout facts merely
    // because this internal fast path omitted an explicit profile.
    analyze_seed_internal(input, floors, Some(&MapProfile::default()))
}

/// Internal profile replay used by parity tests.
/// It is not exposed through the WASM API: users cannot configure a run state.
#[cfg(test)]
pub(crate) fn analyze_seed_with_profile(
    input: &str,
    floors: u32,
    profile: &MapProfile,
) -> Result<SeedReport, AnalyzeError> {
    analyze_seed_internal(input, floors, Some(profile))
}

fn analyze_seed_internal(
    input: &str,
    floors: u32,
    profile: Option<&MapProfile>,
) -> Result<SeedReport, AnalyzeError> {
    let info = parse_seed(input)?;
    let floors = floors.clamp(1, 26);
    let run = init_run(info.numeric);
    let trinket_selection = TrinketSelectionReport::for_run(&run);
    if let Some(profile) = profile {
        profile.validate(trinket_selection.first_effective_depth)?;
    }
    let mut dungeon = dungeon_from_run(run);
    dungeon.baseline_projection = profile.is_some_and(|profile| profile == &MapProfile::default());
    let identities = dungeon.identities.clone();
    let mut floor_reports = level::analyze_floors_with_profile(&mut dungeon, floors, profile);
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
    })
}

#[cfg(test)]
#[path = "analyze_smoke.rs"]
mod analyze_smoke;
