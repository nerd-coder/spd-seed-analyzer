//! Port of `Imp.Quest` (Ambitious Imp, city floors 17–19).
//!
//! `Imp.Quest.spawn` runs at the end of `CityLevel.initRooms` (before shuffle)
//! and generates the cursed +2…+4 ring reward immediately (unlike Wandmaker,
//! which generates wands in `createMobs`).

use crate::generator::{Category, GeneratorState};
use crate::items::model::GeneratedItem;
use crate::random::Random;
use crate::rooms::types::{RoomKind, RoomSpec};

#[derive(Debug, Clone, Default)]
pub struct ImpQuestState {
    pub spawned: bool,
    /// `true` = monks (alternative), `false` = golems.
    pub alternative: bool,
    pub depth: i32,
    /// Ring category draw index immediately before reward generation.
    pub reward_ring_draw_index: Option<i32>,
    /// Ring category draw index immediately after the curse-reroll loop.
    pub reward_ring_draw_end: Option<i32>,
    /// Fixed-profile reward level retained after the report drains the item.
    pub reward_level: Option<i32>,
    /// Ring generated at spawn; drained once into the floor report.
    pub pending_reward: Option<GeneratedItem>,
    pub pending_summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImpSpawnResult {
    pub alternative: bool,
    /// The fixed target class for this spawn depth (Monk or Golem).
    pub target: ImpQuestTarget,
    /// Dwarf tokens required by `WndImp` before the reward can be claimed.
    pub required_tokens: u8,
    pub reward: GeneratedItem,
    pub summary: String,
}

/// Target and token contract used by `Imp.Quest.process` / `WndImp`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpQuestTarget {
    Monks,
    Golems,
}

impl ImpQuestTarget {
    fn from_alternative(alternative: bool) -> Self {
        if alternative {
            Self::Monks
        } else {
            Self::Golems
        }
    }

    pub fn required_tokens(self) -> u8 {
        match self {
            Self::Monks => 5,
            Self::Golems => 4,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Monks => "Monks",
            Self::Golems => "Golems",
        }
    }
}

/// `Imp.Quest.spawn(rooms)` — city only; call before room shuffle.
///
/// SPD: `!spawned && depth > 16 && Random.Int(20 - depth) == 0`.
/// Depth 19 always succeeds if not yet spawned.
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
    imp.alternative = match depth {
        17 => true,  // monks
        19 => false, // golems
        // 18: 50/50
        _ => Random::int_max(2) == 0,
    };

    imp.reward_ring_draw_index = Some(generator.deck_dropped(Category::Ring));
    let reward = generate_reward(generator, depth);
    imp.reward_ring_draw_end = Some(generator.deck_dropped(Category::Ring));
    imp.reward_level = Some(reward.level);
    let target = ImpQuestTarget::from_alternative(imp.alternative);
    let summary = format!("Ambitious Imp ({}) — {}", target.as_str(), reward.title());

    imp.pending_reward = Some(reward);
    imp.pending_summary = Some(summary);
    true
}

/// Take the reward produced on the floor where the Imp room was just added.
pub fn take_pending(imp: &mut ImpQuestState) -> Option<ImpSpawnResult> {
    let reward = imp.pending_reward.take()?;
    let summary = imp
        .pending_summary
        .take()
        .unwrap_or_else(|| "Ambitious Imp".into());
    let target = ImpQuestTarget::from_alternative(imp.alternative);
    Some(ImpSpawnResult {
        alternative: imp.alternative,
        target,
        required_tokens: target.required_tokens(),
        reward,
        summary,
    })
}

fn generate_reward(generator: &mut GeneratorState, depth: i32) -> GeneratedItem {
    // do { reward = random(RING) } while (reward.cursed);
    let mut reward = loop {
        let r = generator.random_category(Category::Ring, depth);
        if !r.cursed {
            break r;
        }
    };
    // reward.upgrade(2) — Ring.upgrade: level++, Random.Int(3)==0 curse clear
    for _ in 0..2 {
        reward.level += 1;
        if Random::int_max(3) == 0 {
            reward.cursed = false;
        }
    }
    reward.cursed = true;
    reward.source = Some("Imp.Quest".into());
    let ring_classes = Category::Ring.def().classes;
    // Runtime history can move the ring deck an unbounded number of draws
    // before the quest (player-state-dependent levelgen mimic prizes and the
    // ring fallback after runtime sources exhaust the artifact deck). Keep
    // every ring class in internal candidate metadata. Public projection keeps
    // only the ring category because the set cannot rule out a concrete class.
    reward.candidate_classes = ring_classes
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    reward.provenance =
        crate::items::model::ItemProvenance::Quest(crate::items::model::QuestRewardRole::ImpRing);
    reward
}

#[cfg(test)]
#[path = "imp/tests.rs"]
mod tests;
