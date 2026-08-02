use super::*;
use crate::level::create_level_partial;
use crate::run::{dungeon_from_run, init_run};
use crate::{MapProfile, TrinketEvent, TrinketEventAction, TrinketKind};
use serde::Deserialize;

const RING_DECK_FIXTURES: [&str; 5] = [
    include_str!(
        "../../../../../tools/java-oracle/fixtures/generator/aaa-aaa-aaa-imp-ring-deck.json"
    ),
    include_str!(
        "../../../../../tools/java-oracle/fixtures/generator/abc-def-ghi-imp-ring-deck.json"
    ),
    include_str!(
        "../../../../../tools/java-oracle/fixtures/generator/gfx-pzh-dch-imp-ring-deck.json"
    ),
    include_str!(
        "../../../../../tools/java-oracle/fixtures/generator/hkt-jzn-xqq-imp-ring-deck.json"
    ),
    include_str!(
        "../../../../../tools/java-oracle/fixtures/generator/zzz-zzz-zzz-imp-ring-deck.json"
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

fn profiled_imp_outcome(seed: i64, trinket: Option<TrinketKind>) -> (i32, bool, Option<i32>) {
    let mut dungeon = dungeon_from_run(init_run(seed));
    let profile = MapProfile {
        trinket_events: trinket
            .into_iter()
            .flat_map(|trinket| {
                [
                    TrinketEvent {
                        before_depth: 17,
                        action: TrinketEventAction::Acquired {
                            trinket,
                            min_upgrades: None,
                        },
                    },
                    TrinketEvent {
                        before_depth: 17,
                        action: TrinketEventAction::Upgraded,
                    },
                    TrinketEvent {
                        before_depth: 17,
                        action: TrinketEventAction::Upgraded,
                    },
                    TrinketEvent {
                        before_depth: 17,
                        action: TrinketEventAction::Upgraded,
                    },
                ]
            })
            .collect(),
        ..MapProfile::default()
    };
    crate::level::analyze_floors_with_profile(&mut dungeon, 19, Some(&profile));
    (
        dungeon.imp.depth,
        dungeon.imp.alternative,
        dungeon.imp.reward_level,
    )
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
fn reward_candidates_cover_the_ring_category() {
    let mut generator = init_run(42).generator;
    Random::reset_generators();
    Random::push_generator_seeded(777);
    let reward = generate_reward(&mut generator, 18);
    Random::pop_generator();

    let crate::items::model::ItemProvenance::Quest(crate::items::model::QuestRewardRole::ImpRing) =
        reward.provenance
    else {
        panic!("Imp reward provenance");
    };
    let ring_classes = Category::Ring.def().classes;
    assert_eq!(reward.candidate_classes.len(), ring_classes.len());
    assert!(ring_classes.iter().all(|class_name| reward
        .candidate_classes
        .iter()
        .any(|candidate| candidate == class_name)));
}

#[test]
fn target_contract_matches_spawn_alternative() {
    assert_eq!(ImpQuestTarget::Monks.required_tokens(), 5);
    assert_eq!(ImpQuestTarget::Golems.required_tokens(), 4);
    assert_eq!(ImpQuestTarget::Monks.as_str(), "Monks");
    assert_eq!(ImpQuestTarget::Golems.as_str(), "Golems");
}

#[test]
fn depth19_always_spawns_when_not_spawned() {
    Random::reset_generators();
    let mut imp = ImpQuestState::default();
    let mut generator = init_run(1).generator;
    Random::push_generator_seeded(1);
    let mut specs = Vec::new();
    assert!(try_spawn(&mut imp, &mut generator, 19, &mut specs));
    assert_eq!(specs[0].name, "AmbitiousImpRoom");
    assert!(!imp.alternative); // golems on 19
    assert!(imp.pending_reward.is_some());
    let result = take_pending(&mut imp).expect("spawn reward");
    assert_eq!(result.target, ImpQuestTarget::Golems);
    assert_eq!(result.required_tokens, 4);
    Random::pop_generator();
}

#[test]
fn depth16_never_spawns() {
    Random::reset_generators();
    let mut imp = ImpQuestState::default();
    let mut generator = init_run(1).generator;
    let mut specs = Vec::new();
    assert!(!try_spawn(&mut imp, &mut generator, 16, &mut specs));
    assert!(specs.is_empty());
}

#[test]
fn held_mossy_clump_can_change_reward_level_on_the_spawn_floor() {
    assert_eq!(profiled_imp_outcome(0, None), (19, false, Some(3)));
    assert_eq!(
        profiled_imp_outcome(0, Some(TrinketKind::MossyClump)),
        (19, false, Some(4))
    );
}

#[test]
fn held_mossy_clump_can_change_spawn_depth_and_depth18_target() {
    assert_eq!(profiled_imp_outcome(5, None).0, 17);
    assert_eq!(profiled_imp_outcome(5, Some(TrinketKind::MossyClump)).0, 18);

    let baseline = profiled_imp_outcome(26, None);
    let mossy = profiled_imp_outcome(26, Some(TrinketKind::MossyClump));
    assert_eq!(baseline.0, 18);
    assert_eq!(mossy.0, 18);
    assert!(!baseline.1, "baseline target is Golems");
    assert!(mossy.1, "Mossy target is Monks");
}
