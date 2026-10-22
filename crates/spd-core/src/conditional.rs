//! Seed-reachable profiles for structured conditional item results.

use crate::report::{FloorReport, ItemDependencyCondition, ItemPredictionKind, ItemSpawnCondition};
use crate::trinkets::{
    Challenge, MapProfile, TrinketEvent, TrinketEventAction, TrinketKind, TrinketSelectionReport,
};

mod items;
pub(crate) use items::merge_possible_items;
mod rooms;
pub(crate) use rooms::merge_possible_rooms;

const VERIFIED_DEPTH: u32 = 4;

#[derive(Debug, Clone)]
pub(crate) struct DiscoveredRoute {
    pub condition: ItemSpawnCondition,
    pub profile: MapProfile,
    pub max_depth: u32,
}

pub(crate) fn default_condition() -> ItemSpawnCondition {
    condition_for_profile(&MapProfile::default(), false)
}

/// Base condition and alternatives for a report pinned to the player's
/// `profile`. With undeclared challenges only Forbidden Runes stays open;
/// declared challenges leave nothing open.
pub(crate) fn pinned_profile_routes(
    profile: &MapProfile,
    floors_requested: u32,
    challenges_declared: bool,
) -> (ItemSpawnCondition, Vec<DiscoveredRoute>) {
    let base = condition_for_profile(profile, challenges_declared);
    if challenges_declared {
        return (base, Vec::new());
    }
    let forbidden_runes = MapProfile {
        challenges: vec![Challenge::ForbiddenRunes],
        ..profile.clone()
    };
    (base, vec![route(forbidden_runes, floors_requested, false)])
}

/// Trinket routes to merge into a report. `declared` is the player's
/// challenge list; `None` keeps Forbidden Runes open as a condition.
pub(crate) fn discover_routes(
    selection: &TrinketSelectionReport,
    floors: &[FloorReport],
    floors_requested: u32,
    declared: Option<&[Challenge]>,
) -> Vec<DiscoveredRoute> {
    let mut routes = Vec::new();
    if declared.is_none() {
        let forbidden_profile = MapProfile {
            challenges: vec![Challenge::ForbiddenRunes],
            ..MapProfile::default()
        };
        routes.push(route(forbidden_profile, floors_requested, false));
    }

    let scope_end = floors_requested.min(VERIFIED_DEPTH);
    if selection.first_effective_depth > scope_end {
        return routes;
    }

    let exact_scroll_depths = exact_transmutation_scroll_depths(floors);
    let mut acquisitions = direct_acquisitions(selection);
    acquisitions.extend(transmuted_acquisitions(
        selection,
        &exact_scroll_depths,
        scope_end,
    ));

    for acquisition in acquisitions {
        for levels in nondecreasing_levels(acquisition.effective_depth, scope_end) {
            let mut events = acquisition.events.clone();
            let mut previous_level = 0;
            for (offset, level) in levels.into_iter().enumerate() {
                let depth = acquisition.effective_depth + offset as u32;
                events.extend((previous_level..level).map(|_| TrinketEvent {
                    before_depth: depth,
                    action: TrinketEventAction::Upgraded,
                }));
                previous_level = level;
            }

            let challenge_sets = match declared {
                Some(challenges) => vec![challenges.to_vec()],
                None => vec![Vec::new(), vec![Challenge::ForbiddenRunes]],
            };
            for challenges in challenge_sets {
                let profile = MapProfile {
                    challenges,
                    trinket_events: events.clone(),
                    ..MapProfile::default()
                };
                routes.push(route(profile, scope_end, declared.is_some()));
            }
        }
    }
    routes
}

fn route(profile: MapProfile, max_depth: u32, challenges_declared: bool) -> DiscoveredRoute {
    DiscoveredRoute {
        condition: condition_for_profile(&profile, challenges_declared),
        profile,
        max_depth,
    }
}

/// Spawn condition for items seen under `profile`. Declared challenges are
/// the player's own, so they never appear as conditions.
pub(crate) fn condition_for_profile(
    profile: &MapProfile,
    challenges_declared: bool,
) -> ItemSpawnCondition {
    let mut all_of = Vec::new();
    if !challenges_declared {
        all_of.push(ItemDependencyCondition::Challenge {
            challenge: Challenge::ForbiddenRunes,
            enabled: profile.challenges.contains(&Challenge::ForbiddenRunes),
        });
    }
    all_of.push(ItemDependencyCondition::Trinket {
        events: profile.trinket_events.clone(),
    });
    ItemSpawnCondition { all_of }
}

#[derive(Debug, Clone)]
struct AcquisitionRoute {
    effective_depth: u32,
    events: Vec<TrinketEvent>,
}

fn direct_acquisitions(selection: &TrinketSelectionReport) -> Vec<AcquisitionRoute> {
    selection
        .catalyst_options
        .iter()
        .filter_map(|class_name| TrinketKind::from_class_name(class_name.class_name()))
        .filter(|trinket| is_replayed(*trinket))
        .map(|trinket| AcquisitionRoute {
            effective_depth: selection.first_effective_depth,
            events: vec![TrinketEvent {
                before_depth: selection.first_effective_depth,
                action: TrinketEventAction::Acquired {
                    trinket,
                    min_upgrades: None,
                },
            }],
        })
        .collect()
}

fn transmuted_acquisitions(
    selection: &TrinketSelectionReport,
    scroll_depths: &[u32],
    scope_end: u32,
) -> Vec<AcquisitionRoute> {
    let Some(initial) = selection
        .catalyst_options
        .iter()
        .filter_map(|class_name| TrinketKind::from_class_name(class_name.class_name()))
        .find(|trinket| !is_unsupported_stateful(*trinket))
    else {
        return Vec::new();
    };

    let mut events = vec![TrinketEvent {
        before_depth: selection.first_effective_depth,
        action: TrinketEventAction::Acquired {
            trinket: initial,
            min_upgrades: None,
        },
    }];
    let mut routes = Vec::new();
    let mut current = initial;
    let mut current_since = selection.first_effective_depth;

    for (index, class_name) in selection.transmutation_sequence.iter().enumerate() {
        let Some(scroll_depth) = scroll_depths.get(index).copied() else {
            break;
        };
        let before_depth = selection.first_effective_depth.max(scroll_depth + 1);
        if before_depth > scope_end
            || (is_unsupported_stateful(current) && current_since < before_depth)
        {
            break;
        }
        let Some(next) = TrinketKind::from_class_name(class_name.class_name()) else {
            break;
        };
        events.push(TrinketEvent {
            before_depth,
            action: TrinketEventAction::Transmuted { trinket: next },
        });
        current = next;
        current_since = before_depth;
        if is_replayed(next) {
            routes.push(AcquisitionRoute {
                effective_depth: before_depth,
                events: events.clone(),
            });
        }
    }
    routes
}

fn exact_transmutation_scroll_depths(floors: &[FloorReport]) -> Vec<u32> {
    let mut depths = Vec::new();
    for floor in floors.iter().filter(|floor| floor.depth < VERIFIED_DEPTH) {
        for item in floor.items.iter().flat_map(|group| &group.variants) {
            if item.prediction == ItemPredictionKind::Exact
                && item.item == Some(spd_objects::ItemId::ScrollOfTransmutation)
            {
                depths.extend(std::iter::repeat_n(
                    floor.depth,
                    item.quantity.max(0) as usize,
                ));
            }
        }
    }
    depths.sort_unstable();
    depths
}

fn is_replayed(trinket: TrinketKind) -> bool {
    matches!(
        trinket,
        TrinketKind::MossyClump
            | TrinketKind::TrapMechanism
            | TrinketKind::MimicTooth
            | TrinketKind::RatSkull
    )
}

fn is_unsupported_stateful(trinket: TrinketKind) -> bool {
    matches!(trinket, TrinketKind::CrackedSpyglass)
}

fn nondecreasing_levels(first_depth: u32, last_depth: u32) -> Vec<Vec<u8>> {
    fn extend(result: &mut Vec<Vec<u8>>, current: &mut Vec<u8>, remaining: usize, min: u8) {
        if remaining == 0 {
            result.push(current.clone());
            return;
        }
        for level in min..=3 {
            current.push(level);
            extend(result, current, remaining - 1, level);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let len = last_depth.saturating_sub(first_depth) as usize + 1;
    extend(&mut result, &mut Vec::new(), len, 0);
    result
}

#[cfg(test)]
#[path = "conditional_tests.rs"]
mod tests;
