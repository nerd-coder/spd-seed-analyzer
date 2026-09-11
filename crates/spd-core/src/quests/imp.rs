//! Port of `Imp.Quest` (Ambitious Imp, city floors 17–19).
//!
//! `Imp.Quest.spawn` runs at the end of `CityLevel.initRooms` (before shuffle)
//! and immediately fills `rewardOptions` (six take-one items). v4 does not
//! roll monks/golems on a fresh run.

use crate::generator::{Category, GeneratorState};
use crate::items::model::GeneratedItem;
use crate::random::Random;
use crate::rooms::types::{RoomKind, RoomSpec};

#[path = "imp/rewards.rs"]
mod rewards;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ImpSpawnDecks {
    pub ring: i32,
    pub artifact: i32,
    pub wand: i32,
    pub wep_t4: i32,
    pub wep_t5: i32,
    pub mis_t4: i32,
    pub mis_t5: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ImpQuestState {
    pub spawned: bool,
    pub depth: i32,
    /// Retained after the floor report drains `pending_options`.
    pub reward_options: Vec<GeneratedItem>,
    pub pending_options: Vec<GeneratedItem>,
    pub dropped_before: ImpSpawnDecks,
    pub dropped_after: ImpSpawnDecks,
}

#[derive(Debug, Clone)]
pub struct ImpSpawnResult {
    pub options: Vec<GeneratedItem>,
}

/// `Imp.Quest.spawn(rooms)` — city only; call before room shuffle.
///
/// SPD: `!spawned && depth > 16 && Random.Int(20 - depth) == 0`.
/// Depth 19 always succeeds if not yet spawned. v4 does not roll `alternative`.
pub fn try_spawn(
    imp: &mut ImpQuestState,
    generator: &mut GeneratorState,
    depth: i32,
    specs: &mut Vec<RoomSpec>,
) -> bool {
    // CityLevel only (depths 16–19). Spawn condition requires depth > 16.
    if !(16..=19).contains(&depth) {
        return false;
    }
    if imp.spawned {
        return false;
    }
    if depth <= 16 || Random::int_max(20 - depth) != 0 {
        return false;
    }

    specs.push(RoomSpec {
        name: "AmbitiousImpRoom".into(),
        kind: RoomKind::Special,
        size_factor: 1,
        max_connections: 1,
    });

    imp.spawned = true;
    imp.depth = depth;
    imp.dropped_before = snapshot_decks(generator);
    let options = rewards::generate_reward_options(generator, depth);
    imp.dropped_after = snapshot_decks(generator);
    imp.reward_options = options.clone();
    imp.pending_options = options;
    true
}

/// Take the pool produced on the floor where the Imp room was just added.
pub fn take_pending(imp: &mut ImpQuestState) -> Option<ImpSpawnResult> {
    if !imp.spawned || imp.pending_options.is_empty() {
        return None;
    }
    Some(ImpSpawnResult {
        options: std::mem::take(&mut imp.pending_options),
    })
}

fn snapshot_decks(generator: &GeneratorState) -> ImpSpawnDecks {
    ImpSpawnDecks {
        ring: generator.deck_dropped(Category::Ring),
        artifact: generator.deck_dropped(Category::Artifact),
        wand: generator.deck_dropped(Category::Wand),
        wep_t4: generator.deck_dropped(Category::WepT4),
        wep_t5: generator.deck_dropped(Category::WepT5),
        mis_t4: generator.deck_dropped(Category::MisT4),
        mis_t5: generator.deck_dropped(Category::MisT5),
    }
}

#[cfg(test)]
#[path = "imp/tests.rs"]
mod tests;
