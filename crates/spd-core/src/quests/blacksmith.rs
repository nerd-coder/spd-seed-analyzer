//! Port of `Blacksmith.Quest` (caves floors 12–14).
//!
//! `Blacksmith.Quest.spawn` runs at the end of `CavesLevel.initRooms` (before
//! shuffle) and immediately generates the smith reward pool
//! (`generateRewards(true)`), which is consistent for the seed.

use crate::generator::GeneratorState;
use crate::items::enchants;
use crate::items::model::GeneratedItem;
use crate::random::Random;
use crate::rooms::types::{RoomKind, RoomSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlacksmithQuestType {
    Crystal = 1,
    Gnoll = 2,
    /// Not fully implemented in SPD; cannot roll currently.
    Fungi = 3,
}

impl BlacksmithQuestType {
    pub fn as_str(self) -> &'static str {
        match self {
            BlacksmithQuestType::Crystal => "Crystal",
            BlacksmithQuestType::Gnoll => "Gnoll",
            BlacksmithQuestType::Fungi => "Fungi",
        }
    }

    fn from_int(v: i32) -> Self {
        match v {
            1 => BlacksmithQuestType::Crystal,
            2 => BlacksmithQuestType::Gnoll,
            _ => BlacksmithQuestType::Fungi,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlacksmithQuestState {
    pub spawned: bool,
    /// Quest type int (1=Crystal, 2=Gnoll).
    pub quest_type: i32,
    pub depth: i32,
    /// Pre-generated smith reward pool (2 weapons + missile + armor).
    pub smith_rewards: Vec<GeneratedItem>,
    /// Optional pre-rolled enchant/glyph (30% base; stored separately in SPD).
    pub smith_enchant: Option<String>,
    pub smith_glyph: Option<String>,
    /// Summary produced at spawn; drained once into the floor report.
    pub pending_summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BlacksmithSpawnResult {
    pub quest_type: BlacksmithQuestType,
    pub rewards: Vec<GeneratedItem>,
    pub smith_enchant: Option<String>,
    pub smith_glyph: Option<String>,
    pub summary: String,
}

/// `Blacksmith.Quest.spawn(rooms)` — caves only; call before room shuffle.
///
/// SPD: `!spawned && depth > 11 && Random.Int(15 - depth) == 0`.
/// Depth 14 always succeeds if not yet spawned.
pub fn try_spawn(
    blacksmith: &mut BlacksmithQuestState,
    generator: &mut GeneratorState,
    depth: i32,
    specs: &mut Vec<RoomSpec>,
) -> bool {
    // CavesLevel only (depths 11–14). Spawn condition requires depth > 11.
    if !(11..=14).contains(&depth) {
        return false;
    }
    if blacksmith.spawned {
        return false;
    }
    if depth <= 11 || Random::int_max(15 - depth) != 0 {
        return false;
    }

    // BlacksmithRoom extends StandardRoom (not SpecialRoom).
    specs.push(RoomSpec {
        name: "BlacksmithRoom".into(),
        kind: RoomKind::Standard,
        size_factor: 1,
        max_connections: 16,
    });

    blacksmith.spawned = true;
    blacksmith.depth = depth;
    // Currently cannot roll fungi (not fully implemented in SPD).
    blacksmith.quest_type = Random::int_range_inclusive(1, 2);

    generate_rewards(blacksmith, generator, depth, true);

    let qtype = BlacksmithQuestType::from_int(blacksmith.quest_type);
    let reward_titles: Vec<String> = blacksmith.smith_rewards.iter().map(|r| r.title()).collect();
    let summary = format!(
        "Blacksmith ({}) — {}",
        qtype.as_str(),
        reward_titles.join(", ")
    );
    blacksmith.pending_summary = Some(summary);
    true
}

/// Take the reward pool produced on the floor where the smith room was added.
pub fn take_pending(blacksmith: &mut BlacksmithQuestState) -> Option<BlacksmithSpawnResult> {
    if !blacksmith.spawned || blacksmith.pending_summary.is_none() {
        return None;
    }
    let summary = blacksmith.pending_summary.take()?;
    Some(BlacksmithSpawnResult {
        quest_type: BlacksmithQuestType::from_int(blacksmith.quest_type),
        rewards: blacksmith.smith_rewards.clone(),
        smith_enchant: blacksmith.smith_enchant.clone(),
        smith_glyph: blacksmith.smith_glyph.clone(),
        summary,
    })
}

/// `Blacksmith.Quest.generateRewards(useDecks)`.
///
/// Note: the Java parameter is named `useDecks` but is passed as
/// `randomWeapon(floorSet, useDefaults)` — `true` means **defaults**, not decks.
fn generate_rewards(
    blacksmith: &mut BlacksmithQuestState,
    generator: &mut GeneratorState,
    depth: i32,
    use_defaults: bool,
) {
    let mut rewards = Vec::new();
    rewards.push(generator.random_weapon(3, use_defaults, depth));
    rewards.push(generator.random_weapon(3, use_defaults, depth));

    let mut to_undo = Vec::new();
    while rewards[0].class_name == rewards[1].class_name {
        if use_defaults {
            to_undo.push(rewards[1].class_name.clone());
        }
        rewards.remove(1);
        rewards.push(generator.random_weapon(3, use_defaults, depth));
    }
    for class_name in &to_undo {
        generator.undo_drop(class_name);
    }

    rewards.push(generator.random_missile(3, use_defaults, depth));
    rewards.push(generator.random_armor(3, depth));

    // 30%:+0, 45%:+1, 20%:+2, 5%:+3
    let reward_level = {
        let roll = Random::float();
        if roll < 0.3 {
            0
        } else if roll < 0.75 {
            1
        } else if roll < 0.95 {
            2
        } else {
            3
        }
    };

    for item in &mut rewards {
        item.level = reward_level;
        item.enchantment = None;
        item.cursed = false;
        item.source = Some("Blacksmith.Quest".into());
    }

    // Always generate first so outcome doesn't affect RNG roll count.
    let mut smith_enchant = Some(enchants::random_weapon_enchant(None).to_string());
    let mut smith_glyph = Some(enchants::random_armor_glyph(None).to_string());
    // 30% base chance (ParchmentScrap multiplier = 1 without trinket).
    if Random::float() > 0.3 {
        smith_enchant = None;
        smith_glyph = None;
    }

    blacksmith.smith_rewards = rewards;
    blacksmith.smith_enchant = smith_enchant;
    blacksmith.smith_glyph = smith_glyph;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::init_run;

    #[test]
    fn rewards_deterministic() {
        let gen_template = init_run(42).generator;

        Random::reset_generators();
        Random::push_generator_seeded(999);
        let mut a = BlacksmithQuestState::default();
        generate_rewards(&mut a, &mut gen_template.clone(), 13, true);
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(999);
        let mut b = BlacksmithQuestState::default();
        generate_rewards(&mut b, &mut gen_template.clone(), 13, true);
        Random::pop_generator();

        assert_eq!(a.smith_rewards.len(), 4);
        assert_eq!(a.smith_rewards.len(), b.smith_rewards.len());
        for (x, y) in a.smith_rewards.iter().zip(b.smith_rewards.iter()) {
            assert_eq!(x.class_name, y.class_name);
            assert_eq!(x.level, y.level);
            assert!(!x.cursed);
            assert!(x.enchantment.is_none());
        }
        assert_eq!(a.smith_enchant, b.smith_enchant);
        assert_eq!(a.smith_glyph, b.smith_glyph);
        // Two weapons must differ in class
        assert_ne!(a.smith_rewards[0].class_name, a.smith_rewards[1].class_name);
    }

    #[test]
    fn depth14_always_spawns_when_not_spawned() {
        Random::reset_generators();
        let mut bs = BlacksmithQuestState::default();
        let mut gen = init_run(1).generator;
        Random::push_generator_seeded(1);
        let mut specs = Vec::new();
        assert!(try_spawn(&mut bs, &mut gen, 14, &mut specs));
        assert_eq!(specs[0].name, "BlacksmithRoom");
        assert!(matches!(bs.quest_type, 1 | 2));
        assert_eq!(bs.smith_rewards.len(), 4);
        assert!(bs.pending_summary.is_some());
        Random::pop_generator();
    }

    #[test]
    fn depth11_never_spawns() {
        Random::reset_generators();
        let mut bs = BlacksmithQuestState::default();
        let mut gen = init_run(1).generator;
        let mut specs = Vec::new();
        assert!(!try_spawn(&mut bs, &mut gen, 11, &mut specs));
        assert!(specs.is_empty());
    }
}
