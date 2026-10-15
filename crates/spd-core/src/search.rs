//! Bounded, resumable constraint search over numeric dungeon seeds.

use serde::{Deserialize, Serialize};
use spd_objects::ItemId;

use crate::{
    report::{ItemEntry, ItemPredictionKind},
    search_seed_floors, AnalyzeError, FloorReport, IdentityMaps, SeedInfo, TOTAL_SEEDS,
};

/// Maximum candidate seeds evaluated by one search call.
pub const MAX_SEARCH_CANDIDATES: u32 = 10_000;
/// Maximum item constraints accepted by one search call.
pub const MAX_SEARCH_CONSTRAINTS: usize = 32;
/// Maximum matching seeds returned by one search call.
pub const MAX_SEARCH_MATCHES: u32 = 100;
/// Maximum depth generated or accepted on a constraint window.
pub const MAX_SEARCH_FLOORS: u32 = 26;

const PARTIAL_SEARCH_MESSAGE: &str = "Search results use the partial analyzer; generated loot is incomplete and may not match the pinned game.";

const fn variable_search_default() -> bool {
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
    /// Allow gameplay-variable items to satisfy constraints. These matches
    /// hold when gameplay has not shifted generation, not in every run.
    #[serde(default = "variable_search_default")]
    pub include_variable: bool,
    /// Widen quest-reward matching to every class a deck-index shift can reach,
    /// not just the seed draw. Off by default: it returns more seeds, and a hit
    /// means "reachable in some run", not "present when gameplay is unchanged".
    #[serde(default)]
    pub deep_search: bool,
    pub max_matches: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemConstraint {
    pub item_group: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemId>,
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
    /// Gameplay-variable highlights. They contribute search evidence only when
    /// `includeVariable` is enabled and remain explicitly labeled.
    pub variable_items: Vec<VariableItemEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VariableItemEvidence {
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
    pub item_group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemId>,
    pub depth: u32,
    /// Present only when the matched entry has no single identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub level: i32,
    pub prediction: ItemPredictionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl ItemMatchEvidence {
    pub fn class_name(&self) -> Option<&'static str> {
        self.item.map(ItemId::class_name)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("startSeed must be within [0, TOTAL_SEEDS)")]
    StartSeedOutOfRange,
    #[error("candidateCount must be between 1 and {MAX_SEARCH_CANDIDATES}")]
    CandidateCountOutOfRange,
    #[error("floors must be between 1 and {MAX_SEARCH_FLOORS}")]
    FloorsOutOfRange,
    #[error("constraints must contain at least one item constraint")]
    EmptyConstraints,
    #[error("constraints must contain at most {MAX_SEARCH_CONSTRAINTS} entries")]
    TooManyConstraints,
    #[error("constraints[{index}].itemGroup must not be empty or whitespace")]
    EmptyItemGroup { index: usize },
    #[error("constraints[{index}].item must be a known item id")]
    UnknownItem { index: usize },
    #[error("constraints[{index}].minLevel must be between 1 and 4 when present")]
    InvalidLevel { index: usize },
    #[error(
        "constraints[{index}] depths must satisfy 1 <= minDepth <= maxDepth <= {MAX_SEARCH_FLOORS}"
    )]
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
        if !(1..=MAX_SEARCH_FLOORS).contains(&self.floors) {
            return Err(SearchError::FloorsOutOfRange);
        }
        if self.constraints.is_empty() {
            return Err(SearchError::EmptyConstraints);
        }
        if self.constraints.len() > MAX_SEARCH_CONSTRAINTS {
            return Err(SearchError::TooManyConstraints);
        }
        for (index, constraint) in self.constraints.iter().enumerate() {
            if constraint.item_group.trim().is_empty() {
                return Err(SearchError::EmptyItemGroup { index });
            }
            if constraint.item == Some(ItemId::Unknown) {
                return Err(SearchError::UnknownItem { index });
            }
            if constraint
                .min_level
                .is_some_and(|level| !(1..=4).contains(&level))
            {
                return Err(SearchError::InvalidLevel { index });
            }
            if !(1..=MAX_SEARCH_FLOORS).contains(&constraint.min_depth)
                || !(1..=MAX_SEARCH_FLOORS).contains(&constraint.max_depth)
                || constraint.min_depth > constraint.max_depth
            {
                return Err(SearchError::InvalidDepthRange { index });
            }
        }
        if !(1..=MAX_SEARCH_MATCHES).contains(&self.max_matches) {
            return Err(SearchError::MaxMatchesOutOfRange);
        }
        Ok(())
    }

    /// Floors generated for each candidate: through the deepest constraint
    /// window. Floors below it cannot change a match.
    pub fn generation_floors(&self) -> u32 {
        self.constraints
            .iter()
            .map(|constraint| constraint.max_depth)
            .max()
            .unwrap_or(self.floors)
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

        let (seed_info, identities, floors) =
            search_seed_floors(&seed.to_string(), request.generation_floors(), |so_far| {
                if request.deep_search {
                    let newest = so_far.len() - 1;
                    widen_quest_candidates(&mut so_far[newest..]);
                }
                request.match_mode == MatchMode::Any || closed_windows_can_match(request, so_far)
            })
            .map_err(|source| SearchError::Analyze { seed, source })?;
        candidates_scanned += 1;

        let evidence = matching_evidence(&floors, &request.constraints, request.include_variable);
        let is_match = match request.match_mode {
            MatchMode::Any => !evidence.is_empty(),
            MatchMode::All => evidence.len() == request.constraints.len(),
        };
        if is_match {
            let variable_items = floors
                .iter()
                .flat_map(|floor| {
                    floor
                        .items
                        .iter()
                        .flat_map(|group| &group.variants)
                        .filter(|item| item.prediction == ItemPredictionKind::Variable)
                        .cloned()
                        .map(|item| VariableItemEvidence {
                            depth: floor.depth,
                            source: item.source.clone(),
                            item,
                        })
                })
                .collect();
            matches.push(SeedMatch {
                seed: seed_info,
                identities,
                evidence,
                variable_items,
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

/// All mode needs every constraint matched. Once the newest floor closes a
/// depth window, the constraints whose windows have all closed must already
/// be jointly satisfiable: no deeper floor can supply them.
fn closed_windows_can_match(request: &SeedSearchRequest, floors: &[FloorReport]) -> bool {
    let Some(depth) = floors.last().map(|floor| floor.depth) else {
        return true;
    };
    if !request
        .constraints
        .iter()
        .any(|constraint| constraint.max_depth == depth)
    {
        return true;
    }
    let closed = request
        .constraints
        .iter()
        .filter(|constraint| constraint.max_depth <= depth)
        .cloned()
        .collect::<Vec<_>>();
    matching_evidence(floors, &closed, request.include_variable).len() == closed.len()
}

fn matching_evidence(
    floors: &[FloorReport],
    constraints: &[ItemConstraint],
    include_variable: bool,
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
            include_variable,
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
                include_variable,
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
    include_variable: bool,
    assigned_constraints: &mut [Option<usize>],
    visited_occurrences: &mut [bool],
) -> bool {
    for (occurrence_index, occurrence) in occurrences.iter().copied().enumerate() {
        if visited_occurrences[occurrence_index]
            || matching_item(&constraints[constraint_index], occurrence, include_variable).is_none()
        {
            continue;
        }
        visited_occurrences[occurrence_index] = true;

        let can_assign = assigned_constraints[occurrence_index].is_none_or(|assigned| {
            assign_constraint(
                assigned,
                constraints,
                occurrences,
                include_variable,
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
    include_variable: bool,
) -> Option<ItemMatchEvidence> {
    let item = matching_item(constraint, occurrence, include_variable)?;
    Some(ItemMatchEvidence {
        constraint_index: constraint_index as u32,
        item_group: constraint.item_group.clone(),
        item: item
            .item
            .or_else(|| item.candidates.first().copied())
            .or(constraint.item),
        depth: occurrence.depth,
        label: item.label.clone(),
        level: item.level.unwrap_or(0),
        prediction: item.prediction,
        source: item.source.clone(),
    })
}

fn matching_item<'a>(
    constraint: &ItemConstraint,
    occurrence: ItemOccurrence<'a>,
    include_variable: bool,
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
            if item.category != constraint.item_group {
                return false;
            }

            let class_matches = if let Some(wanted) = constraint.item {
                let concrete_class_matches = item.item == Some(wanted);
                let candidate_class_matches = item.candidates.contains(&wanted);
                match item.prediction {
                    ItemPredictionKind::Exact => concrete_class_matches || candidate_class_matches,
                    ItemPredictionKind::Variable => {
                        include_variable && (concrete_class_matches || candidate_class_matches)
                    }
                    ItemPredictionKind::Constrained => candidate_class_matches,
                }
            } else {
                match item.prediction {
                    ItemPredictionKind::Exact | ItemPredictionKind::Constrained => true,
                    ItemPredictionKind::Variable => include_variable,
                }
            };

            class_matches
                && constraint
                    .min_level
                    .is_none_or(|minimum| item.level.is_some_and(|level| level >= minimum))
        })
}

mod quest_candidates;
use quest_candidates::widen_quest_candidates;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mode_tests;
