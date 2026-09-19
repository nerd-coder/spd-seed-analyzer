//! Per-slot candidate distributions for the Imp's six take-out options.
//!
//! A category's draw sequence is a pure function of the seed: `cat.seed` is
//! fixed at `Generator.fullReset` (`Generator.java:624-635`) and `probs`
//! evolves deterministically from the draw sequence (`Generator.java:712-728`).
//! Run history only chooses *which index* a draw lands on, so the candidate set
//! for a slot is a window of that sequence around the fresh-baseline index.
//!
//! Drift is bidirectional — a held trinket can remove earlier deck-using sites
//! as well as add them (see `tests::drift_matches_profile_replay_in_both_directions`).

use super::rewards::{ImpSlotDraw, ImpSlotKind};
use crate::generator::{Category, GeneratorState};
use crate::items::model::GeneratedItem;
use crate::report::{ImpLevelRule, ImpRewardSlot, ImpSlotCandidate, ImpSlotRole};

/// Deck offsets either side of the baseline index for the standard report.
pub const DEFAULT_DRIFT: i32 = 3;
/// Wider window used when the caller opts into deep search.
pub const DEEP_DRIFT: i32 = 8;

/// Build the public per-slot projection.
///
/// `generator` may be the post-spawn state: sequences always replay from index
/// 0, so the deck counter at call time does not matter.
pub fn build_reward_slots(
    generator: &GeneratorState,
    draws: &[ImpSlotDraw],
    options: &[GeneratedItem],
    depth: i32,
    drift: i32,
) -> Vec<ImpRewardSlot> {
    draws
        .iter()
        .zip(options)
        .enumerate()
        .map(|(slot, (draw, item))| build_slot(generator, draw, item, slot as u8, depth, drift))
        .collect()
}

fn build_slot(
    generator: &GeneratorState,
    draw: &ImpSlotDraw,
    item: &GeneratedItem,
    slot: u8,
    depth: i32,
    drift: i32,
) -> ImpRewardSlot {
    let role = role_of(draw.kind);
    let level_rule = level_rule_for(draw, slot);
    let enchanted = matches!(
        draw.kind,
        ImpSlotKind::Weapon | ImpSlotKind::Missile | ImpSlotKind::Armor
    );

    let Some(category) = draw.category else {
        // `new PlateArmor()` — no deck, so the class cannot vary at all.
        return ImpRewardSlot {
            slot,
            role,
            baseline_class: item.class_name.clone(),
            level_rule,
            enchanted,
            fixed_class: Some(item.class_name.clone()),
            candidates: Vec::new(),
            scenario_count: 1,
        };
    };

    let (candidates, scenario_count) =
        candidates_for(generator, category, draw.deck_index, depth, drift);

    ImpRewardSlot {
        slot,
        role,
        baseline_class: item.class_name.clone(),
        level_rule,
        enchanted,
        fixed_class: None,
        candidates,
        scenario_count,
    }
}

/// Collapse the drift window into weighted classes, most likely first.
fn candidates_for(
    generator: &GeneratorState,
    category: Category,
    baseline_index: i32,
    depth: i32,
    drift: i32,
) -> (Vec<ImpSlotCandidate>, u32) {
    let first = (baseline_index - drift).max(0);
    let last = baseline_index + drift;
    let sequence = generator.category_class_sequence(category, (last + 1) as usize, depth);

    // Offsets are relative to the baseline draw; nearest-first keeps the
    // baseline at drift 0 and makes the annotation readable.
    let mut collapsed: Vec<ImpSlotCandidate> = Vec::new();
    let mut offsets: Vec<i32> = (first..=last).map(|index| index - baseline_index).collect();
    offsets.sort_by_key(|offset| (offset.abs(), *offset));

    for offset in &offsets {
        let index = (baseline_index + offset) as usize;
        let Some(class_name) = sequence.get(index) else {
            continue;
        };
        match collapsed
            .iter_mut()
            .find(|candidate| &candidate.class_name == class_name)
        {
            Some(candidate) => {
                candidate.drifts.push(*offset);
                candidate.weight += 1;
            }
            None => collapsed.push(ImpSlotCandidate {
                class_name: class_name.clone(),
                drifts: vec![*offset],
                weight: 1,
            }),
        }
    }

    let scenario_count: u32 = collapsed.iter().map(|candidate| candidate.weight).sum();
    // The fresh-run draw always leads: it is the concrete answer for a player
    // who has not shifted the deck, so ranking a drift alternative above it
    // purely because that class occupies more offsets would read as if the
    // alternative were the likelier reward. Remaining rows are ordered by how
    // much of the window they hold, then by nearest offset.
    collapsed.sort_by_key(|candidate| {
        (
            !candidate.drifts.contains(&0),
            std::cmp::Reverse(candidate.weight),
            candidate.drifts.first().map_or(i32::MAX, |d| d.abs()),
        )
    });
    (collapsed, scenario_count.max(1))
}

fn role_of(kind: ImpSlotKind) -> ImpSlotRole {
    match kind {
        ImpSlotKind::Artifact => ImpSlotRole::Artifact,
        ImpSlotKind::Ring => ImpSlotRole::Ring,
        ImpSlotKind::Weapon => ImpSlotRole::Weapon,
        ImpSlotKind::Missile => ImpSlotRole::Missile,
        ImpSlotKind::Armor => ImpSlotRole::Armor,
        ImpSlotKind::Wand => ImpSlotRole::Wand,
    }
}

/// Level rules are fixed in `Imp.java:318-350` and do not depend on the seed.
/// Slots 2 and 3 carry the tier-5/tier-4 pair chosen by `Random.Int(2)`: the
/// tier-5 half rolls `IntRange(2, 4)` and the tier-4 half `IntRange(3, 5)`.
fn level_rule_for(draw: &ImpSlotDraw, slot: u8) -> ImpLevelRule {
    match draw.kind {
        // Slot 0 is only an artifact when the ARTIFACT deck still had a card.
        ImpSlotKind::Artifact => ImpLevelRule::TransferUpgrade { transfer: 5 },
        _ if slot == 3 => ImpLevelRule::Range { min: 3, max: 5 },
        _ => ImpLevelRule::Range { min: 2, max: 4 },
    }
}

#[cfg(test)]
#[path = "distribution_tests.rs"]
mod tests;
