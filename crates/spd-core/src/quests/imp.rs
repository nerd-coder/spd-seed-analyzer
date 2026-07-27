//! Port of `Imp.Quest` (Ambitious Imp, city floors 17–19).
//!
//! `Imp.Quest.spawn` runs at the end of `CityLevel.initRooms` (before shuffle)
//! and generates the cursed +2 ring reward immediately (unlike Wandmaker, which
//! generates wands in `createMobs`).

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
    /// Ring generated at spawn; drained once into the floor report.
    pub pending_reward: Option<GeneratedItem>,
    pub pending_summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImpSpawnResult {
    pub alternative: bool,
    pub reward: GeneratedItem,
    pub summary: String,
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
    let target = if imp.alternative { "Monks" } else { "Golems" };
    let summary = format!("Ambitious Imp ({target}) — {}", reward.title());

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
    Some(ImpSpawnResult {
        alternative: imp.alternative,
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
    reward.provenance =
        crate::items::model::ItemProvenance::Quest(crate::items::model::QuestRewardRole::ImpRing);
    reward
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::create_level_partial;
    use crate::run::{dungeon_from_run, init_run};
    use serde::Deserialize;

    const RING_DECK_FIXTURES: [&str; 5] = [
        include_str!(
            "../../../../tools/java-oracle/fixtures/generator/aaa-aaa-aaa-imp-ring-deck.json"
        ),
        include_str!(
            "../../../../tools/java-oracle/fixtures/generator/abc-def-ghi-imp-ring-deck.json"
        ),
        include_str!(
            "../../../../tools/java-oracle/fixtures/generator/gfx-pzh-dch-imp-ring-deck.json"
        ),
        include_str!(
            "../../../../tools/java-oracle/fixtures/generator/hkt-jzn-xqq-imp-ring-deck.json"
        ),
        include_str!(
            "../../../../tools/java-oracle/fixtures/generator/zzz-zzz-zzz-imp-ring-deck.json"
        ),
    ];

    #[derive(Deserialize)]
    struct RingDeckFixture {
        contract: String,
        spd: FixturePin,
        input: FixtureInput,
        spawn: FixtureSpawn,
    }

    #[derive(Deserialize)]
    struct FixturePin {
        version: String,
        commit: String,
    }

    #[derive(Deserialize)]
    struct FixtureInput {
        seed: String,
        numeric: i64,
    }

    #[derive(Deserialize)]
    struct FixtureSpawn {
        depth: i32,
        ring_dropped_before: i32,
        ring_dropped_after: i32,
    }

    #[test]
    fn ring_draw_index_matches_pinned_java_at_imp_spawn() {
        for fixture_json in RING_DECK_FIXTURES {
            let fixture: RingDeckFixture =
                serde_json::from_str(fixture_json).expect("Imp ring deck fixture");
            assert_eq!(fixture.contract, "imp_ring_deck");
            assert_eq!(fixture.spd.version, crate::SPD_VERSION);
            assert_eq!(fixture.spd.commit, crate::SPD_COMMIT);

            let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
            for depth in 1..=fixture.spawn.depth {
                dungeon.depth = depth;
                create_level_partial(&mut dungeon);
            }

            assert_eq!(
                dungeon.imp.depth, fixture.spawn.depth,
                "{} depth",
                fixture.input.seed
            );
            assert_eq!(
                dungeon.imp.reward_ring_draw_index,
                Some(fixture.spawn.ring_dropped_before),
                "{} pre-reward ring draw index",
                fixture.input.seed
            );
            assert_eq!(
                dungeon.imp.reward_ring_draw_end,
                Some(fixture.spawn.ring_dropped_after),
                "{} post-reward ring draw index",
                fixture.input.seed
            );
        }
    }

    #[test]
    fn reward_deterministic_and_cursed_plus_two() {
        let gen_template = init_run(42).generator;

        Random::reset_generators();
        Random::push_generator_seeded(777);
        let r1 = generate_reward(&mut gen_template.clone(), 18);
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(777);
        let r2 = generate_reward(&mut gen_template.clone(), 18);
        Random::pop_generator();

        assert_eq!(r1.class_name, r2.class_name);
        assert_eq!(r1.level, r2.level);
        assert!(r1.cursed);
        // randomize_ring level 0–2 then +2 → 2–4
        assert!((2..=4).contains(&r1.level), "level={}", r1.level);
    }

    #[test]
    fn altered_ring_deck_changes_identity_without_changing_level_or_rng_tail() {
        let mut fresh = init_run(42).generator;
        let mut altered = fresh.clone();
        Random::reset_generators();
        Random::push_generator_seeded(123);
        let _ = altered.random_category(Category::Ring, 18);
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(777);
        let fresh_reward = generate_reward(&mut fresh, 18);
        let fresh_tail = Random::peek_ints(8);
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(777);
        let altered_reward = generate_reward(&mut altered, 18);
        let altered_tail = Random::peek_ints(8);
        Random::pop_generator();

        assert_ne!(fresh_reward.class_name, altered_reward.class_name);
        assert_eq!(fresh_reward.level, altered_reward.level);
        assert_eq!(fresh_tail, altered_tail);
    }

    #[test]
    fn depth19_always_spawns_when_not_spawned() {
        Random::reset_generators();
        let mut imp = ImpQuestState::default();
        let mut gen = init_run(1).generator;
        Random::push_generator_seeded(1);
        let mut specs = Vec::new();
        assert!(try_spawn(&mut imp, &mut gen, 19, &mut specs));
        assert_eq!(specs[0].name, "AmbitiousImpRoom");
        assert!(!imp.alternative); // golems on 19
        assert!(imp.pending_reward.is_some());
        Random::pop_generator();
    }

    #[test]
    fn depth16_never_spawns() {
        Random::reset_generators();
        let mut imp = ImpQuestState::default();
        let mut gen = init_run(1).generator;
        let mut specs = Vec::new();
        assert!(!try_spawn(&mut imp, &mut gen, 16, &mut specs));
        assert!(specs.is_empty());
    }
}
