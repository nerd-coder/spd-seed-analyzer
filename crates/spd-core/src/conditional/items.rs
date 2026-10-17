use crate::report::{
    group_item_entries, FloorReport, ItemDependencyCondition, ItemEntry, ItemPredictionKind,
    ItemSpawnCondition,
};

use super::DiscoveredRoute;

/// Consumes the alternative floors' items; run it after every other merge
/// that reads them.
pub(crate) fn merge_possible_items(
    floors: &mut [FloorReport],
    default_condition: ItemSpawnCondition,
    alternatives: &mut [(DiscoveredRoute, Vec<FloorReport>)],
) {
    for floor in floors {
        let (mut variable_items, default_items): (Vec<_>, Vec<_>) =
            std::mem::take(&mut floor.items)
                .into_iter()
                .flat_map(|group| group.variants)
                .partition(|item| item.prediction == ItemPredictionKind::Variable);
        let mut contexts = vec![(&default_condition, default_items)];
        for (route, route_floors) in alternatives.iter_mut() {
            if floor.depth > route.max_depth {
                continue;
            }
            let Some(candidate) = route_floors
                .iter_mut()
                .find(|candidate| candidate.depth == floor.depth)
            else {
                continue;
            };
            let items = std::mem::take(&mut candidate.items)
                .into_iter()
                .flat_map(|group| group.variants)
                .filter(|item| item.prediction != ItemPredictionKind::Variable)
                .collect();
            contexts.push((&route.condition, items));
        }
        let mut entries = merge_contexts(contexts);
        entries.append(&mut variable_items);
        floor.items = group_item_entries(entries);
    }
}

fn merge_contexts(contexts: Vec<(&ItemSpawnCondition, Vec<ItemEntry>)>) -> Vec<ItemEntry> {
    let (conditions, items): (Vec<_>, Vec<_>) = contexts.into_iter().unzip();
    let mut merged = Vec::<(ItemEntry, Vec<usize>)>::new();
    for (context_index, items) in items.into_iter().enumerate() {
        for item in items {
            if let Some((candidate, present_in)) =
                merged.iter_mut().find(|(candidate, present_in)| {
                    items_equivalent(candidate, &item) && !present_in.contains(&context_index)
                })
            {
                for condition in item.conditions {
                    if !candidate.conditions.contains(&condition) {
                        candidate.conditions.push(condition);
                    }
                }
                present_in.push(context_index);
            } else {
                merged.push((item, vec![context_index]));
            }
        }
    }

    merged
        .into_iter()
        .map(|(mut item, present_in)| {
            if present_in.len() < conditions.len() {
                let intrinsic = item.spawn_conditions.clone();
                item.spawn_conditions = simplify_profile_conditions(&present_in, &conditions)
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
    conditions: &[&ItemSpawnCondition],
) -> Vec<ItemSpawnCondition> {
    let mut clauses = Vec::new();
    let challenge_values = unique_dependencies(conditions, is_challenge);
    let trinket_values = unique_dependencies(conditions, is_trinket);

    for dependency in challenge_values.into_iter().chain(trinket_values) {
        let matching = conditions
            .iter()
            .enumerate()
            .filter(|(_, condition)| condition.all_of.contains(&dependency))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if !matching.is_empty() && matching.iter().all(|index| present_in.contains(index)) {
            clauses.push(ItemSpawnCondition {
                all_of: vec![dependency],
            });
        }
    }

    for index in present_in {
        let profile = conditions[*index];
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
    conditions: &[&ItemSpawnCondition],
    predicate: fn(&ItemDependencyCondition) -> bool,
) -> Vec<ItemDependencyCondition> {
    let mut result = Vec::new();
    for dependency in conditions
        .iter()
        .flat_map(|condition| &condition.all_of)
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
