//! Bounded, resumable constraint search over numeric dungeon seeds.

use serde::{Deserialize, Serialize};

use crate::{
    analyze_seed_seed_only,
    report::{ItemEntry, ItemPredictionKind},
    AnalyzeError, IdentityMaps, SeedInfo, TOTAL_SEEDS,
};

/// Maximum candidate seeds evaluated by one search call.
pub const MAX_SEARCH_CANDIDATES: u32 = 10_000;
/// Maximum item constraints accepted by one search call.
pub const MAX_SEARCH_CONSTRAINTS: usize = 32;
/// Maximum matching seeds returned by one search call.
pub const MAX_SEARCH_MATCHES: u32 = 100;

const PARTIAL_SEARCH_MESSAGE: &str = "Search results use the partial analyzer; generated loot is incomplete and may not match the pinned game.";

const fn baseline_search_default() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SeedSearchRequest {
    pub start_seed: i64,
    pub candidate_count: u32,
    pub floors: u32,
    pub constraints: Vec<ItemConstraint>,
    pub match_mode: MatchMode,
    /// Allow fresh/no-history baseline predictions to satisfy constraints.
    /// These matches are planning evidence, not seed-wide guarantees.
    #[serde(default = "baseline_search_default")]
    pub include_baseline: bool,
    pub max_matches: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemConstraint {
    /// Exact Java simple class name, such as `PotionOfHealing`.
    pub class_name: String,
    /// Optional minimum requested upgrade level (`None` accepts any level).
    #[serde(default)]
    pub min_level: Option<i32>,
    pub min_depth: u32,
    pub max_depth: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatchMode {
    Any,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedSearchResult {
    pub start_seed: i64,
    pub requested_candidates: u32,
    pub candidates_scanned: u32,
    /// First unscanned seed, or `None` after reaching `TOTAL_SEEDS`.
    pub next_seed: Option<i64>,
    pub exhausted: bool,
    /// True when scanning stopped because `maxMatches` was reached.
    pub match_limit_reached: bool,
    pub match_mode: MatchMode,
    pub matches: Vec<SeedMatch>,
    /// Always `"partial"` until the underlying analyzer reaches full parity.
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedMatch {
    pub seed: SeedInfo,
    pub identities: IdentityMaps,
    /// One distinct matching item occurrence per satisfied constraint.
    pub evidence: Vec<ItemMatchEvidence>,
    /// Fresh/no-history planning highlights. They contribute search evidence
    /// only when `includeBaseline` is enabled and remain explicitly labeled.
    pub baseline_items: Vec<BaselineItemEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BaselineItemEvidence {
    pub depth: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(flatten)]
    pub item: ItemEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ItemMatchEvidence {
    pub constraint_index: u32,
    pub class_name: String,
    pub depth: u32,
    pub name: String,
    pub level: i32,
    pub prediction: ItemPredictionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("startSeed must be within [0, TOTAL_SEEDS)")]
    StartSeedOutOfRange,
    #[error("candidateCount must be between 1 and {MAX_SEARCH_CANDIDATES}")]
    CandidateCountOutOfRange,
    #[error("floors must be between 1 and 26")]
    FloorsOutOfRange,
    #[error("constraints must contain at least one item constraint")]
    EmptyConstraints,
    #[error("constraints must contain at most {MAX_SEARCH_CONSTRAINTS} entries")]
    TooManyConstraints,
    #[error("constraints[{index}].className must not be empty or whitespace")]
    EmptyClassName { index: usize },
    #[error("constraints[{index}].className must be at most 128 characters")]
    ClassNameTooLong { index: usize },
    #[error("constraints[{index}].minLevel must be between 1 and 4 when present")]
    InvalidLevel { index: usize },
    #[error("constraints[{index}] depths must satisfy 1 <= minDepth <= maxDepth <= floors")]
    InvalidDepthRange { index: usize },
    #[error("maxMatches must be between 1 and {MAX_SEARCH_MATCHES}")]
    MaxMatchesOutOfRange,
    #[error("failed to analyze seed {seed}: {source}")]
    Analyze {
        seed: i64,
        #[source]
        source: AnalyzeError,
    },
}

impl SeedSearchRequest {
    pub fn validate(&self) -> Result<(), SearchError> {
        if !(0..TOTAL_SEEDS).contains(&self.start_seed) {
            return Err(SearchError::StartSeedOutOfRange);
        }
        if !(1..=MAX_SEARCH_CANDIDATES).contains(&self.candidate_count) {
            return Err(SearchError::CandidateCountOutOfRange);
        }
        if !(1..=26).contains(&self.floors) {
            return Err(SearchError::FloorsOutOfRange);
        }
        if self.constraints.is_empty() {
            return Err(SearchError::EmptyConstraints);
        }
        if self.constraints.len() > MAX_SEARCH_CONSTRAINTS {
            return Err(SearchError::TooManyConstraints);
        }
        for (index, constraint) in self.constraints.iter().enumerate() {
            if constraint.class_name.trim().is_empty() {
                return Err(SearchError::EmptyClassName { index });
            }
            if constraint.class_name.chars().count() > 128 {
                return Err(SearchError::ClassNameTooLong { index });
            }
            if constraint
                .min_level
                .is_some_and(|level| !(1..=4).contains(&level))
            {
                return Err(SearchError::InvalidLevel { index });
            }
            if constraint.min_depth == 0
                || constraint.min_depth > constraint.max_depth
                || constraint.max_depth > self.floors
            {
                return Err(SearchError::InvalidDepthRange { index });
            }
        }
        if !(1..=MAX_SEARCH_MATCHES).contains(&self.max_matches) {
            return Err(SearchError::MaxMatchesOutOfRange);
        }
        Ok(())
    }
}

/// Search an ascending, non-wrapping chunk of numeric seeds.
pub fn search_seeds(request: &SeedSearchRequest) -> Result<SeedSearchResult, SearchError> {
    request.validate()?;

    let mut candidates_scanned = 0;
    let mut matches = Vec::new();

    while candidates_scanned < request.candidate_count {
        let seed = request.start_seed + i64::from(candidates_scanned);
        if seed >= TOTAL_SEEDS {
            break;
        }

        let report = analyze_seed_seed_only(&seed.to_string(), request.floors)
            .map_err(|source| SearchError::Analyze { seed, source })?;
        candidates_scanned += 1;

        let evidence = matching_evidence(
            &report.floors,
            &request.constraints,
            request.include_baseline,
        );
        let is_match = match request.match_mode {
            MatchMode::Any => !evidence.is_empty(),
            MatchMode::All => evidence.len() == request.constraints.len(),
        };
        if is_match {
            let baseline_items = report
                .floors
                .iter()
                .flat_map(|floor| {
                    floor
                        .items
                        .iter()
                        .flat_map(|group| &group.variants)
                        .filter(|item| item.prediction == ItemPredictionKind::Baseline)
                        .cloned()
                        .map(|item| BaselineItemEvidence {
                            depth: floor.depth,
                            source: item.source.clone(),
                            item,
                        })
                })
                .collect();
            matches.push(SeedMatch {
                seed: report.seed,
                identities: report.identities,
                evidence,
                baseline_items,
            });
            if matches.len() == request.max_matches as usize {
                break;
            }
        }
    }

    let next = request.start_seed + i64::from(candidates_scanned);
    let exhausted = next >= TOTAL_SEEDS;
    let match_limit_reached = matches.len() == request.max_matches as usize
        && candidates_scanned < request.candidate_count
        && !exhausted;

    Ok(SeedSearchResult {
        start_seed: request.start_seed,
        requested_candidates: request.candidate_count,
        candidates_scanned,
        next_seed: (!exhausted).then_some(next),
        exhausted,
        match_limit_reached,
        match_mode: request.match_mode,
        matches,
        status: "partial".to_string(),
        message: PARTIAL_SEARCH_MESSAGE.to_string(),
    })
}

fn matching_evidence(
    floors: &[crate::FloorReport],
    constraints: &[ItemConstraint],
    include_baseline: bool,
) -> Vec<ItemMatchEvidence> {
    let occurrences = floors
        .iter()
        .flat_map(|floor| {
            floor.items.iter().flat_map(move |group| {
                let quantity = group
                    .variants
                    .iter()
                    .map(|item| item.quantity.max(0))
                    .max()
                    .unwrap_or(0)
                    .min(constraints.len() as i32);
                (0..quantity).map(move |copy_index| ItemOccurrence {
                    depth: floor.depth,
                    group,
                    copy_index,
                })
            })
        })
        .collect::<Vec<_>>();
    let mut assigned_constraints = vec![None; occurrences.len()];

    for constraint_index in 0..constraints.len() {
        let mut visited_occurrences = vec![false; occurrences.len()];
        assign_constraint(
            constraint_index,
            constraints,
            &occurrences,
            include_baseline,
            &mut assigned_constraints,
            &mut visited_occurrences,
        );
    }

    let mut evidence = assigned_constraints
        .iter()
        .enumerate()
        .filter_map(|(occurrence_index, constraint_index)| {
            let constraint_index = (*constraint_index)?;
            evidence_for(
                constraint_index,
                &constraints[constraint_index],
                occurrences[occurrence_index],
                include_baseline,
            )
        })
        .collect::<Vec<_>>();
    evidence.sort_unstable_by_key(|item| item.constraint_index);
    evidence
}

#[derive(Clone, Copy)]
struct ItemOccurrence<'a> {
    depth: u32,
    group: &'a crate::report::ItemGroup,
    copy_index: i32,
}

fn assign_constraint(
    constraint_index: usize,
    constraints: &[ItemConstraint],
    occurrences: &[ItemOccurrence<'_>],
    include_baseline: bool,
    assigned_constraints: &mut [Option<usize>],
    visited_occurrences: &mut [bool],
) -> bool {
    for (occurrence_index, occurrence) in occurrences.iter().copied().enumerate() {
        if visited_occurrences[occurrence_index]
            || matching_item(&constraints[constraint_index], occurrence, include_baseline).is_none()
        {
            continue;
        }
        visited_occurrences[occurrence_index] = true;

        let can_assign = assigned_constraints[occurrence_index].is_none_or(|assigned| {
            assign_constraint(
                assigned,
                constraints,
                occurrences,
                include_baseline,
                assigned_constraints,
                visited_occurrences,
            )
        });
        if can_assign {
            assigned_constraints[occurrence_index] = Some(constraint_index);
            return true;
        }
    }
    false
}

fn evidence_for(
    constraint_index: usize,
    constraint: &ItemConstraint,
    occurrence: ItemOccurrence<'_>,
    include_baseline: bool,
) -> Option<ItemMatchEvidence> {
    let item = matching_item(constraint, occurrence, include_baseline)?;
    Some(ItemMatchEvidence {
        constraint_index: constraint_index as u32,
        class_name: constraint.class_name.clone(),
        depth: occurrence.depth,
        name: item.name.clone(),
        level: item
            .level
            .expect("searchable predictions expose a concrete level"),
        prediction: item.prediction,
        source: item.source.clone(),
    })
}

fn matching_item<'a>(
    constraint: &ItemConstraint,
    occurrence: ItemOccurrence<'a>,
    include_baseline: bool,
) -> Option<&'a ItemEntry> {
    if !(constraint.min_depth..=constraint.max_depth).contains(&occurrence.depth) {
        return None;
    }

    occurrence
        .group
        .variants
        .iter()
        .filter(|item| item.quantity > occurrence.copy_index)
        .find(|item| {
            let concrete_class_matches =
                item.class_name.as_deref() == Some(constraint.class_name.as_str());
            let candidate_class_matches = item
                .candidate_classes
                .iter()
                .any(|candidate| candidate == &constraint.class_name);
            let class_matches = match item.prediction {
                ItemPredictionKind::Exact => concrete_class_matches || candidate_class_matches,
                ItemPredictionKind::Baseline => {
                    include_baseline && (concrete_class_matches || candidate_class_matches)
                }
                ItemPredictionKind::Constrained => candidate_class_matches,
            };

            class_matches
                && constraint
                    .min_level
                    .is_none_or(|minimum| item.level.is_some_and(|level| level >= minimum))
        })
}

#[cfg(test)]
mod tests;
