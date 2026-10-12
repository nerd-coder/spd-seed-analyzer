//! Quest-aware widening for deep search.

use crate::report::ItemPredictionKind;

/// Fill seed-only (`Constrained`) Imp and Wandmaker entries with every class
/// a deck-index shift can reach.
///
/// Those entries carry no concrete class, and `Constrained` only ever matches
/// through `candidate_classes`. The classes come from each quest contract's
/// per-slot window, already computed for the report. Baseline rows are left
/// unchanged.
pub(super) fn widen_quest_candidates(floors: &mut [crate::FloorReport]) {
    for floor in floors {
        widen_imp(floor);
        widen_wandmaker(floor);
    }
}

fn widen_imp(floor: &mut crate::FloorReport) {
    let Some(slots) = floor.quests.iter().find_map(|quest| match quest {
        crate::QuestReport::AmbitiousImp { contract, .. } => Some(contract.slots.clone()),
        _ => None,
    }) else {
        return;
    };

    // Both the `Constrained` and `Baseline` projections are emitted per
    // slot, in slot order. Only the seed-only half is widened.
    let mut slots = slots.into_iter();
    for group in floor.items.iter_mut() {
        if group.source.as_deref() != Some("Imp.Quest") {
            continue;
        }
        for item in group
            .variants
            .iter_mut()
            .filter(|item| item.prediction == ItemPredictionKind::Constrained)
        {
            let Some(slot) = slots.next() else { break };
            item.candidate_classes = match slot.fixed_class {
                Some(fixed) => vec![fixed],
                None => slot
                    .candidates
                    .into_iter()
                    .map(|candidate| candidate.class_name)
                    .collect(),
            };
        }
    }
}

fn widen_wandmaker(floor: &mut crate::FloorReport) {
    let Some(slots) = floor.quests.iter().find_map(|quest| match quest {
        crate::QuestReport::OldWandmaker { contract, .. } => Some(contract.slots.clone()),
        _ => None,
    }) else {
        return;
    };

    // Constrained rows are wand 1 then wand 2. Baseline rows stay empty.
    let mut slots = slots.into_iter();
    for group in floor.items.iter_mut() {
        if group.source.as_deref() != Some("Wandmaker.Quest") {
            continue;
        }
        for item in group
            .variants
            .iter_mut()
            .filter(|item| item.prediction == ItemPredictionKind::Constrained)
        {
            let Some(slot) = slots.next() else { break };
            item.candidate_classes = slot
                .candidates
                .into_iter()
                .map(|candidate| candidate.class_name)
                .collect();
        }
    }
}
