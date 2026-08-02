use std::collections::BTreeMap;

use crate::report::{FloorReport, ItemDependencyCondition, ItemSpawnCondition, PossibleRoom};

use super::DiscoveredRoute;

/// Adds a profile-conditioned room projection only when a replay actually
/// changes the baseline room multiset. The ordered `FloorReport.rooms` field
/// remains the no-extra-player-state baseline.
pub(crate) fn merge_possible_rooms(
    baseline: &mut [FloorReport],
    baseline_condition: ItemSpawnCondition,
    alternatives: &[(DiscoveredRoute, Vec<FloorReport>)],
) {
    for floor in baseline {
        let contexts = std::iter::once((&baseline_condition, &floor.rooms))
            .chain(alternatives.iter().filter_map(|(route, floors)| {
                (floor.depth <= route.max_depth)
                    .then(|| {
                        floors
                            .iter()
                            .find(|candidate| candidate.depth == floor.depth)
                    })
                    .flatten()
                    .map(|candidate| (&route.condition, &candidate.rooms))
            }))
            .collect::<Vec<_>>();

        if contexts
            .iter()
            .skip(1)
            .all(|(_, rooms)| *rooms == contexts[0].1)
        {
            continue;
        }

        floor.possible_rooms = merge_contexts(&contexts);
    }
}

fn merge_contexts(contexts: &[(&ItemSpawnCondition, &Vec<String>)]) -> Vec<PossibleRoom> {
    let mut present_in = BTreeMap::<(String, u32), Vec<usize>>::new();
    for (context_index, (_, rooms)) in contexts.iter().enumerate() {
        for (class_name, quantity) in counts(rooms) {
            present_in
                .entry((class_name, quantity))
                .or_default()
                .push(context_index);
        }
    }

    present_in
        .into_iter()
        .map(|((class_name, quantity), present_in)| PossibleRoom {
            class_name,
            quantity,
            spawn_conditions: if present_in.len() < contexts.len() {
                simplify_profile_conditions(&present_in, contexts)
            } else {
                Vec::new()
            },
        })
        .collect()
}

fn counts(rooms: &[String]) -> BTreeMap<String, u32> {
    let mut result = BTreeMap::new();
    for room in rooms {
        *result.entry(room.clone()).or_default() += 1;
    }
    result
}

fn simplify_profile_conditions(
    present_in: &[usize],
    contexts: &[(&ItemSpawnCondition, &Vec<String>)],
) -> Vec<ItemSpawnCondition> {
    let mut clauses = Vec::new();
    let challenge_values = unique_dependencies(contexts, is_challenge);
    let trinket_values = unique_dependencies(contexts, is_trinket);

    for dependency in challenge_values.into_iter().chain(trinket_values) {
        let matching = contexts
            .iter()
            .enumerate()
            .filter(|(_, (condition, _))| condition.all_of.contains(&dependency))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if !matching.is_empty() && matching.iter().all(|index| present_in.contains(index)) {
            clauses.push(ItemSpawnCondition {
                all_of: vec![dependency],
            });
        }
    }

    for index in present_in {
        let profile = contexts[*index].0;
        if !clauses.iter().any(|clause| {
            clause
                .all_of
                .iter()
                .all(|dependency| profile.all_of.contains(dependency))
        }) {
            clauses.push(profile.clone());
        }
    }
    clauses
}

fn unique_dependencies(
    contexts: &[(&ItemSpawnCondition, &Vec<String>)],
    predicate: fn(&ItemDependencyCondition) -> bool,
) -> Vec<ItemDependencyCondition> {
    let mut result = Vec::new();
    for dependency in contexts
        .iter()
        .flat_map(|(condition, _)| &condition.all_of)
        .filter(|dependency| predicate(dependency))
    {
        if !result.contains(dependency) {
            result.push(dependency.clone());
        }
    }
    result
}

fn is_challenge(dependency: &ItemDependencyCondition) -> bool {
    matches!(dependency, ItemDependencyCondition::Challenge { .. })
}

fn is_trinket(dependency: &ItemDependencyCondition) -> bool {
    matches!(dependency, ItemDependencyCondition::Trinket { .. })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_room_type_retains_its_profile_specific_count() {
        let baseline = vec!["EmptyRoom".into(), "EmptyRoom".into()];
        let alternate = vec!["EmptyRoom".into(), "TunnelRoom".into()];
        let plain = ItemSpawnCondition::default();
        let trinket = ItemSpawnCondition {
            all_of: vec![ItemDependencyCondition::Trinket { events: vec![] }],
        };
        let rooms = merge_contexts(&[(&plain, &baseline), (&trinket, &alternate)]);

        assert!(rooms.iter().any(|room| {
            room.class_name == "EmptyRoom" && room.quantity == 2 && room.spawn_conditions.len() == 1
        }));
        assert!(rooms.iter().any(|room| {
            room.class_name == "TunnelRoom"
                && room.quantity == 1
                && room.spawn_conditions.len() == 1
        }));
    }
}
