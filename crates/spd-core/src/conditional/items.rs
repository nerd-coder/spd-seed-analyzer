use crate::report::{
    FloorReport, ItemDependencyCondition, ItemEntry, ItemPredictionKind, ItemSpawnCondition,
};

use super::DiscoveredRoute;

pub(crate) fn merge_possible_items(
    baseline: &mut [FloorReport],
    baseline_condition: ItemSpawnCondition,
    alternatives: &[(DiscoveredRoute, Vec<FloorReport>)],
) {
    for floor in baseline {
        let (mut baseline_samples, baseline_items): (Vec<_>, Vec<_>) =
            std::mem::take(&mut floor.items)
                .into_iter()
                .partition(|item| item.prediction == ItemPredictionKind::Baseline);
        let alternative_items = alternatives
            .iter()
            .filter_map(|(route, floors)| {
                (floor.depth <= route.max_depth)
                    .then(|| {
                        floors
                            .iter()
                            .find(|candidate| candidate.depth == floor.depth)
                    })
                    .flatten()
                    .map(|candidate| {
                        let items = candidate
                            .items
                            .iter()
                            .filter(|item| item.prediction != ItemPredictionKind::Baseline)
                            .cloned()
                            .collect::<Vec<_>>();
                        (&route.condition, items)
                    })
            })
            .collect::<Vec<_>>();
        let contexts = std::iter::once((&baseline_condition, &baseline_items))
            .chain(
                alternative_items
                    .iter()
                    .map(|(condition, items)| (*condition, items)),
            )
            .collect::<Vec<_>>();
        floor.items = merge_contexts(&contexts);
        floor.items.append(&mut baseline_samples);
    }
}

fn merge_contexts(contexts: &[(&ItemSpawnCondition, &Vec<ItemEntry>)]) -> Vec<ItemEntry> {
    let mut merged = Vec::<(ItemEntry, Vec<usize>)>::new();
    for (context_index, (_, items)) in contexts.iter().enumerate() {
        for item in *items {
            if let Some((candidate, present_in)) =
                merged.iter_mut().find(|(candidate, present_in)| {
                    items_equivalent(candidate, item) && !present_in.contains(&context_index)
                })
            {
                for condition in &item.conditions {
                    if !candidate.conditions.contains(condition) {
                        candidate.conditions.push(condition.clone());
                    }
                }
                present_in.push(context_index);
            } else {
                merged.push((item.clone(), vec![context_index]));
            }
        }
    }

    merged
        .into_iter()
        .map(|(mut item, present_in)| {
            if present_in.len() < contexts.len() {
                let intrinsic = item.spawn_conditions.clone();
                item.spawn_conditions = simplify_profile_conditions(&present_in, contexts)
                    .into_iter()
                    .flat_map(|profile| combine_conditions(&intrinsic, &profile))
                    .collect();
                deduplicate_conditions(&mut item.spawn_conditions);
            }
            item
        })
        .collect()
}

fn simplify_profile_conditions(
    present_in: &[usize],
    contexts: &[(&ItemSpawnCondition, &Vec<ItemEntry>)],
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
    contexts: &[(&ItemSpawnCondition, &Vec<ItemEntry>)],
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

fn combine_conditions(
    intrinsic: &[ItemSpawnCondition],
    profile: &ItemSpawnCondition,
) -> Vec<ItemSpawnCondition> {
    if intrinsic.is_empty() {
        return vec![profile.clone()];
    }
    intrinsic
        .iter()
        .map(|condition| {
            let mut all_of = condition.all_of.clone();
            all_of.extend(profile.all_of.clone());
            ItemSpawnCondition { all_of }
        })
        .collect()
}

fn deduplicate_conditions(conditions: &mut Vec<ItemSpawnCondition>) {
    for condition in conditions.iter_mut() {
        let mut unique = Vec::new();
        for dependency in condition.all_of.drain(..) {
            if !unique.contains(&dependency) {
                unique.push(dependency);
            }
        }
        condition.all_of = unique;
    }
    let mut unique = Vec::new();
    for condition in conditions.drain(..) {
        if !unique.contains(&condition) {
            unique.push(condition);
        }
    }
    let candidates = unique.clone();
    unique.retain(|condition| {
        !candidates.iter().any(|candidate| {
            candidate != condition
                && candidate.all_of.len() < condition.all_of.len()
                && candidate
                    .all_of
                    .iter()
                    .all(|dependency| condition.all_of.contains(dependency))
        })
    });
    *conditions = unique;
}

fn items_equivalent(left: &ItemEntry, right: &ItemEntry) -> bool {
    let mut left = left.clone();
    let mut right = right.clone();
    left.source = None;
    right.source = None;
    left.spawn_conditions.clear();
    right.spawn_conditions.clear();
    left.conditions.clear();
    right.conditions.clear();
    left.notes.clear();
    right.notes.clear();
    left == right
}
