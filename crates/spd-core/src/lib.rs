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
    AnalyzeError, FloorReport, GuaranteedAppearance, GuaranteedAppearanceKind, ModeledOutcome,
    SeedInfo, SeedReport,
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

/// Analyze a seed and replay every currently modeled run-condition branch.
///
/// The primary `floors` projection contains only facts safe without selecting
/// a player-controlled run state. `modeled_outcomes` supplies the separate
/// first-generation replays for supported conditions.
pub fn analyze_seed(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    let seed = parse_seed(input)?;
    let first_effective_depth =
        TrinketSelectionReport::for_run(&init_run(seed.numeric)).first_effective_depth;
    let mut modeled_outcomes = Vec::new();

    for (condition, notes, profile) in modeled_profiles(first_effective_depth) {
        let outcome = analyze_seed_internal(input, floors, Some(&profile))?;
        modeled_outcomes.push(ModeledOutcome {
            condition,
            notes,
            floors: outcome.floors,
        });
    }

    // Run the conservative projection last. A few parity probes retain the
    // most-recent replay trace, which must remain the profile-free path for
    // callers that inspect it after `analyze_seed` returns.
    let mut report = analyze_seed_internal(input, floors, None)?;
    report.modeled_outcomes = modeled_outcomes;
    report.analysis_notes = vec![
        "The seed-only result never assumes a challenge, trinket, or artifact history."
            .to_string(),
        format!(
            "{} modeled first-generation combinations are replayed below: Forbidden Runes on or off, with no held trinket or Mossy Clump, Trap Mechanism, and Mimic Tooth at +0 through +3 from floor {first_effective_depth}.",
            report.modeled_outcomes.len()
        ),
        "Acquire/upgrade/transmute timing, Rat Skull, Cracked Spyglass, Barren Land, Badder Bosses, external artifacts, and other runtime paths are not fully replayed yet. Their effects remain conditional while analysis status is partial."
            .to_string(),
    ];
    Ok(report)
}

/// Seed-only projection used by the bounded finder. It deliberately avoids
/// replaying the UI-facing modeled branch matrix for every candidate seed.
pub(crate) fn analyze_seed_seed_only(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    analyze_seed_internal(input, floors, None)
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
    let mut layout_dungeon = dungeon.clone();
    let identities = dungeon.identities.clone();
    let mut floor_reports = level::analyze_floors_with_profile(&mut dungeon, floors, profile);
    if let Some(profile) = profile {
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
        modeled_outcomes: Vec::new(),
        analysis_notes: Vec::new(),
        status: "partial".to_string(),
        message: Some(
            "Analysis accuracy is partial; results may differ from the pinned game.".to_string(),
        ),
    })
}

fn modeled_profiles(first_effective_depth: u32) -> Vec<(String, Vec<String>, MapProfile)> {
    let mut profiles = Vec::new();
    for forbidden_runes in [false, true] {
        let challenge_label = forbidden_runes.then_some("Forbidden Runes");
        let base_profile = MapProfile {
            challenges: forbidden_runes
                .then_some(Challenge::ForbiddenRunes)
                .into_iter()
                .collect(),
            ..MapProfile::default()
        };
        profiles.push((
            challenge_label
                .map(|challenge| format!("{challenge}; no held trinket"))
                .unwrap_or_else(|| "No challenges; no held trinket".to_string()),
            vec![
                "This is one modeled first-generation condition, not a claim about player choices."
                    .to_string(),
            ],
            base_profile,
        ));

        for (trinket, label) in [
            (TrinketKind::MossyClump, "Mossy Clump"),
            (TrinketKind::TrapMechanism, "Trap Mechanism"),
            (TrinketKind::MimicTooth, "Mimic Tooth"),
        ] {
            for level in 0..=3 {
                let mut trinket_events = vec![TrinketEvent {
                    before_depth: first_effective_depth,
                    action: TrinketEventAction::Acquired { trinket },
                }];
                trinket_events.extend((0..level).map(|_| TrinketEvent {
                    before_depth: first_effective_depth,
                    action: TrinketEventAction::Upgraded,
                }));
                let profile = MapProfile {
                    challenges: forbidden_runes
                        .then_some(Challenge::ForbiddenRunes)
                        .into_iter()
                        .collect(),
                    trinket_events,
                    ..MapProfile::default()
                };
                let challenge_prefix = challenge_label
                    .map(|challenge| format!("{challenge}; "))
                    .unwrap_or_default();
                profiles.push((
                    format!("{challenge_prefix}{label} +{level} held from floor {first_effective_depth}"),
                    vec![
                        "This is a modeled possible held-trinket condition. Its actual acquisition, upgrade, or transmutation route is player-dependent."
                            .to_string(),
                    ],
                    profile,
                ));
            }
        }
    }
    profiles
}

#[cfg(test)]
#[path = "analyze_smoke.rs"]
mod analyze_smoke;
