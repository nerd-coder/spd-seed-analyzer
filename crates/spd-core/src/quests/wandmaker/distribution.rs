//! Deck-index window for the Old Wandmaker's two wand options.
//!
//! The wand deck order is fixed by the category seed. Run history only chooses
//! wand 1's index. Wand 2 is the next card that is not wand 1's class; rejected
//! duplicates stay drawn (`undoDrop` does not restore a concrete wand). Each
//! reject also burns floor RNG inside `Wand.random`, so wand 2's +N stays at
//! the fresh value only when every offset rejects the same number of duplicates.

use crate::generator::{Category, GeneratorState};
use crate::items::model::GeneratedItem;
use crate::report::{ImpSlotCandidate, WandmakerRewardSlot};

/// Deck offsets either side of wand 1's fresh index.
pub const DEFAULT_DRIFT: i32 = 3;

/// Extra deck cards after the furthest wand-1 index. A same-class run cannot
/// exceed one reset boundary (each wand's weight is 3), but the walk is not
/// capped at the drift window: a reset streak still has to resolve.
const REPLAY_SLACK: usize = 64;

/// `draws` are already ordered nearest-offset first.
struct OffsetDraw {
    offset: i32,
    wand1_class: String,
    wand2_class: String,
    rejects: u32,
}

/// Build the two public slots. `generator` may already be past the reward
/// draws: the class sequence replays from index 0 under an isolated deck.
pub fn build_reward_slots(
    generator: &GeneratorState,
    wand1_index: i32,
    wand1: &GeneratedItem,
    wand2: &GeneratedItem,
    depth: i32,
    drift: i32,
) -> Vec<WandmakerRewardSlot> {
    let draws = replay_pairs(generator, wand1_index, depth, drift);
    let scenario_count = draws.len() as u32;
    let fresh_rejects = draws
        .iter()
        .find(|draw| draw.offset == 0)
        .map(|draw| draw.rejects);
    // Same reject count means wand 2's `random` + `upgrade` see the same
    // floor RNG at every offset. A different count moves that +N.
    let wand2_level_pinned =
        fresh_rejects.is_some_and(|fresh| draws.iter().all(|draw| draw.rejects == fresh));

    vec![
        slot_for(0, wand1, &draws, scenario_count, true, |draw| {
            &draw.wand1_class
        }),
        slot_for(
            1,
            wand2,
            &draws,
            scenario_count,
            wand2_level_pinned,
            |draw| &draw.wand2_class,
        ),
    ]
}

fn slot_for(
    slot: u8,
    item: &GeneratedItem,
    draws: &[OffsetDraw],
    scenario_count: u32,
    level_pinned: bool,
    class_of: impl Fn(&OffsetDraw) -> &str,
) -> WandmakerRewardSlot {
    WandmakerRewardSlot {
        slot,
        baseline_class: item.class_name.clone(),
        baseline_level: item.level,
        level_pinned,
        candidates: candidates_for(draws, class_of),
        scenario_count,
    }
}

fn candidates_for(
    draws: &[OffsetDraw],
    class_of: impl Fn(&OffsetDraw) -> &str,
) -> Vec<ImpSlotCandidate> {
    let mut collapsed: Vec<ImpSlotCandidate> = Vec::new();
    for draw in draws {
        let class_name = class_of(draw);
        match collapsed
            .iter_mut()
            .find(|candidate| candidate.class_name == class_name)
        {
            Some(candidate) => {
                candidate.drifts.push(draw.offset);
                candidate.weight += 1;
            }
            None => collapsed.push(ImpSlotCandidate {
                class_name: class_name.to_string(),
                drifts: vec![draw.offset],
                weight: 1,
            }),
        }
    }
    // The fresh class leads even when another class occupies more offsets.
    // Remaining rows are by share of the window, then nearest offset.
    collapsed.sort_by_key(|candidate| {
        (
            !candidate.drifts.contains(&0),
            std::cmp::Reverse(candidate.weight),
            candidate
                .drifts
                .first()
                .copied()
                .map_or(i32::MAX, |drift| drift.abs()),
        )
    });
    collapsed
}

fn replay_pairs(
    generator: &GeneratorState,
    wand1_index: i32,
    depth: i32,
    drift: i32,
) -> Vec<OffsetDraw> {
    let offsets = offsets_for(wand1_index, drift);
    if offsets.is_empty() {
        return Vec::new();
    }
    let furthest = offsets
        .iter()
        .map(|offset| wand1_index + offset)
        .max()
        .unwrap_or(0) as usize;
    let mut slack = REPLAY_SLACK;
    loop {
        let sequence =
            generator.category_class_sequence(Category::Wand, furthest + 1 + slack, depth);
        if let Some(draws) = pairs_from_sequence(&sequence, wand1_index, &offsets) {
            return draws;
        }
        if slack >= 512 {
            break;
        }
        slack *= 2;
    }
    panic!("wand deck replay did not resolve a distinct second wand (index {wand1_index})");
}

fn pairs_from_sequence(
    sequence: &[String],
    wand1_index: i32,
    offsets: &[i32],
) -> Option<Vec<OffsetDraw>> {
    let mut draws = Vec::with_capacity(offsets.len());
    for offset in offsets {
        let start = (wand1_index + offset) as usize;
        let (wand1_class, wand2_class, rejects) = accepting_pair(sequence, start)?;
        draws.push(OffsetDraw {
            offset: *offset,
            wand1_class,
            wand2_class,
            rejects,
        });
    }
    Some(draws)
}

/// Walk forward from the card after wand 1 until the class differs.
fn accepting_pair(sequence: &[String], wand1_at: usize) -> Option<(String, String, u32)> {
    let wand1_class = sequence.get(wand1_at)?.clone();
    for (rejects, class_name) in sequence.iter().skip(wand1_at + 1).enumerate() {
        if class_name != &wand1_class {
            return Some((wand1_class, class_name.clone(), rejects as u32));
        }
    }
    None
}

fn offsets_for(wand1_index: i32, drift: i32) -> Vec<i32> {
    let mut offsets: Vec<i32> = (-drift..=drift)
        .filter(|offset| wand1_index + offset >= 0)
        .collect();
    // Nearest first, so grouped drift lists read baseline, then ±1, ±2, …
    offsets.sort_by_key(|offset| (offset.abs(), *offset));
    offsets
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{Category, GeneratorState};
    use crate::items::model::GeneratedItem;
    use crate::run::{dungeon_from_run, init_run};

    struct Spawned {
        generator: GeneratorState,
        index: i32,
        depth: i32,
        contract: crate::report::OldWandmakerQuestContract,
        wand1: GeneratedItem,
        wand2: GeneratedItem,
    }

    fn spawned_wandmaker(seed: i64) -> Option<Spawned> {
        let mut dungeon = dungeon_from_run(init_run(seed));
        for depth in 1..=9 {
            dungeon.depth = depth;
            let state = crate::level::create_level_partial(&mut dungeon);
            let Some(contract) = state.quests.into_iter().find_map(|quest| match quest {
                crate::report::QuestReport::OldWandmaker { contract, .. } => Some(contract),
                _ => None,
            }) else {
                continue;
            };
            let index = super::super::take_wand1_index().expect("wand1 deck index captured");
            let wand1 = dungeon.wandmaker.wand1.clone().expect("wand1");
            let wand2 = dungeon.wandmaker.wand2.clone().expect("wand2");
            return Some(Spawned {
                generator: dungeon.generator.clone(),
                index,
                depth,
                contract,
                wand1,
                wand2,
            });
        }
        let _ = super::super::take_wand1_index();
        None
    }

    /// Offsets in the ±`DEFAULT_DRIFT` window whose deck index is in range.
    fn included_offsets(index: i32) -> Vec<i32> {
        (-DEFAULT_DRIFT..=DEFAULT_DRIFT)
            .filter(|offset| index + offset >= 0)
            .collect()
    }

    /// Independent of `build_reward_slots`: count duplicate rejects per offset.
    fn duplicate_reject_counts(spawned: &Spawned) -> Vec<(i32, u32)> {
        let offsets = included_offsets(spawned.index);
        let furthest = offsets
            .iter()
            .map(|offset| spawned.index + offset)
            .max()
            .unwrap_or(0) as usize;
        let sequence = spawned.generator.category_class_sequence(
            Category::Wand,
            furthest + 1 + 96,
            spawned.depth,
        );
        offsets
            .into_iter()
            .map(|offset| {
                let start = (spawned.index + offset) as usize;
                let class_name = &sequence[start];
                for (rejects, next) in sequence.iter().skip(start + 1).enumerate() {
                    if next != class_name {
                        return (offset, rejects as u32);
                    }
                }
                panic!(
                    "seed replay too short for wand index {} offset {offset}",
                    spawned.index
                );
            })
            .collect()
    }

    #[test]
    fn window_puts_the_fresh_class_first() {
        let spawned = spawned_wandmaker(0).expect("seed 0 spawns the wandmaker by depth 9");
        assert_eq!(spawned.contract.slots.len(), 2);
        let wand1 = &spawned.contract.slots[0];
        let wand2 = &spawned.contract.slots[1];
        assert_eq!((wand1.slot, wand2.slot), (0, 1));
        assert!(wand1.level_pinned);
        assert_eq!(wand1.baseline_class, spawned.wand1.class_name);
        assert_eq!(wand2.baseline_class, spawned.wand2.class_name);
        assert_eq!(wand1.baseline_level, spawned.wand1.level);
        assert_eq!(wand2.baseline_level, spawned.wand2.level);
        assert!(spawned.wand1.candidate_classes.is_empty());
        assert!(spawned.wand2.candidate_classes.is_empty());

        let included = included_offsets(spawned.index).len() as u32;
        for slot in &spawned.contract.slots {
            let first = slot
                .candidates
                .first()
                .unwrap_or_else(|| panic!("slot {} has no candidates", slot.slot));
            assert!(
                first.drifts.contains(&0),
                "slot {} drift 0 must lead, got {first:?}",
                slot.slot
            );
            assert_eq!(first.class_name, slot.baseline_class);
            let weight: u32 = slot
                .candidates
                .iter()
                .map(|candidate| candidate.weight)
                .sum();
            assert_eq!(weight, slot.scenario_count);
            assert_eq!(slot.scenario_count, included);
        }

        let drift0 = wand2
            .candidates
            .iter()
            .find(|candidate| candidate.drifts.contains(&0))
            .expect("wand2 drift 0");
        assert_eq!(drift0.class_name, spawned.wand2.class_name);
    }

    #[test]
    fn wand2_level_unpins_when_duplicate_rejects_differ() {
        let mut checked = 0u32;
        let mut unpinned_seed = None;
        for seed in 0..40 {
            let Some(spawned) = spawned_wandmaker(seed) else {
                continue;
            };
            checked += 1;
            let counts = duplicate_reject_counts(&spawned);
            let fresh = counts
                .iter()
                .find(|(offset, _)| *offset == 0)
                .map(|(_, rejects)| *rejects)
                .expect("fresh offset");
            let differs = counts.iter().any(|(_, rejects)| *rejects != fresh);
            let wand1 = &spawned.contract.slots[0];
            let wand2 = &spawned.contract.slots[1];
            assert!(wand1.level_pinned, "seed {seed}");
            assert_eq!(
                wand2.level_pinned, !differs,
                "seed {seed} duplicate rejects {counts:?}"
            );
            if differs {
                unpinned_seed = Some(seed);
            }
        }
        assert!(checked > 0, "no seed in 0..40 spawned the wandmaker");
        assert!(
            unpinned_seed.is_some(),
            "no seed in 0..40 had a shifted wand index reject a different number of duplicate wands"
        );
    }
}
