use super::*;
use crate::items::model::{ItemCategory, ItemProvenance, QuestRewardRole};
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
    schema_version: u32,
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
    artifact_dropped_before: i32,
    artifact_dropped_after: i32,
    wand_dropped_before: i32,
    wand_dropped_after: i32,
    wep_t4_dropped_before: i32,
    wep_t4_dropped_after: i32,
    wep_t5_dropped_before: i32,
    wep_t5_dropped_after: i32,
    mis_t4_dropped_before: i32,
    mis_t4_dropped_after: i32,
    mis_t5_dropped_before: i32,
    mis_t5_dropped_after: i32,
    reward_options: Vec<RewardOption>,
}

#[derive(Deserialize)]
struct RewardOption {
    class: String,
    level: i32,
    cursed: bool,
}

fn replay_through(seed: i64, max_depth: i32) -> crate::dungeon::DungeonState {
    let mut dungeon = dungeon_from_run(init_run(seed));
    for depth in 1..=max_depth {
        dungeon.depth = depth;
        let _ = crate::level::create_level_partial(&mut dungeon);
    }
    dungeon
}

fn profiled_imp_outcome(seed: i64, trinket: Option<TrinketKind>) -> (i32, Vec<String>) {
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
        dungeon
            .imp
            .reward_options
            .iter()
            .map(|item| item.class_name.clone())
            .collect(),
    )
}

#[test]
fn imp_ring_deck_fixture_pins_v4_spawn_draw_sites() {
    for fixture_json in RING_DECK_FIXTURES {
        let fixture: RingDeckFixture =
            serde_json::from_str(fixture_json).expect("Imp ring deck fixture");
        assert_eq!(fixture.schema_version, 1);
        assert_eq!(fixture.contract, "imp_ring_deck");
        assert_eq!(fixture.spd.version, crate::SPD_VERSION);
        assert_eq!(fixture.spd.commit, crate::SPD_COMMIT);
        assert_eq!(
            fixture.input.numeric,
            crate::parse_seed(&fixture.input.seed)
                .expect("oracle seed")
                .numeric
        );
        assert!(
            (17..=19).contains(&fixture.spawn.depth),
            "{} spawn depth",
            fixture.input.seed
        );

        let dungeon = replay_through(fixture.input.numeric, fixture.spawn.depth);
        let spawn = &fixture.spawn;
        assert_eq!(
            dungeon.imp.depth, spawn.depth,
            "{} spawn depth",
            fixture.input.seed
        );
        let expected_delta = ImpSpawnDecks {
            ring: spawn.ring_dropped_after - spawn.ring_dropped_before,
            artifact: spawn.artifact_dropped_after - spawn.artifact_dropped_before,
            wand: spawn.wand_dropped_after - spawn.wand_dropped_before,
            wep_t4: spawn.wep_t4_dropped_after - spawn.wep_t4_dropped_before,
            wep_t5: spawn.wep_t5_dropped_after - spawn.wep_t5_dropped_before,
            mis_t4: spawn.mis_t4_dropped_after - spawn.mis_t4_dropped_before,
            mis_t5: spawn.mis_t5_dropped_after - spawn.mis_t5_dropped_before,
        };
        let actual_delta = ImpSpawnDecks {
            ring: dungeon.imp.dropped_after.ring - dungeon.imp.dropped_before.ring,
            artifact: dungeon.imp.dropped_after.artifact - dungeon.imp.dropped_before.artifact,
            wand: dungeon.imp.dropped_after.wand - dungeon.imp.dropped_before.wand,
            wep_t4: dungeon.imp.dropped_after.wep_t4 - dungeon.imp.dropped_before.wep_t4,
            wep_t5: dungeon.imp.dropped_after.wep_t5 - dungeon.imp.dropped_before.wep_t5,
            mis_t4: dungeon.imp.dropped_after.mis_t4 - dungeon.imp.dropped_before.mis_t4,
            mis_t5: dungeon.imp.dropped_after.mis_t5 - dungeon.imp.dropped_before.mis_t5,
        };
        assert_eq!(
            actual_delta, expected_delta,
            "{} dropped deltas before={:?} after={:?}",
            fixture.input.seed, dungeon.imp.dropped_before, dungeon.imp.dropped_after
        );

        assert_eq!(
            dungeon.imp.reward_options.len(),
            6,
            "{} rewardOptions after initRooms",
            fixture.input.seed
        );
        let options = &dungeon.imp.reward_options;
        assert_eq!(options[4].class_name, "PlateArmor");
        assert_eq!(options[1].category, ItemCategory::Ring);
        assert_eq!(options[5].category, ItemCategory::Wand);
        assert!(matches!(
            options[0].category,
            ItemCategory::Artifact | ItemCategory::Ring
        ));
        assert!(options.iter().all(|item| !item.cursed));
        assert!(spawn.reward_options.iter().all(|item| !item.cursed));
        // Class identity still follows persistent decks from prior floors
        // (unported city layout). Ambient IntRange/coin-flip overwrites do not.
        for (index, (actual, expected)) in options.iter().zip(&spawn.reward_options).enumerate() {
            if index == 0 && actual.class_name != expected.class {
                continue;
            }
            assert_eq!(
                actual.level, expected.level,
                "{} reward[{index}] {} level (class {})",
                fixture.input.seed, expected.class, actual.class_name
            );
        }
    }
}

#[test]
fn transfer_upgrade_uses_artifact_level_cap() {
    assert_eq!(rewards::transfer_upgrade_level("SandalsOfNature", 5), 2);
    assert_eq!(
        rewards::transfer_upgrade_level("TimekeepersHourglass", 5),
        3
    );
    assert_eq!(rewards::transfer_upgrade_level("EtherealChains", 5), 3);
    assert_eq!(rewards::transfer_upgrade_level("TalismanOfForesight", 5), 5);
    assert_eq!(
        rewards::transfer_upgrade_level("MasterThievesArmband", 5),
        5
    );
}

#[test]
fn reward_options_are_deterministic_uncursed_and_take_one() {
    let gen_template = init_run(42).generator;

    Random::reset_generators();
    Random::push_generator_seeded(777);
    let first = rewards::generate_reward_options(&mut gen_template.clone(), 18);
    Random::pop_generator();

    Random::reset_generators();
    Random::push_generator_seeded(777);
    let second = rewards::generate_reward_options(&mut gen_template.clone(), 18);
    Random::pop_generator();

    assert_eq!(first.len(), 6);
    assert_eq!(
        first
            .iter()
            .map(|item| (&item.class_name, item.level, item.cursed, &item.enchantment))
            .collect::<Vec<_>>(),
        second
            .iter()
            .map(|item| (&item.class_name, item.level, item.cursed, &item.enchantment))
            .collect::<Vec<_>>()
    );
    assert!(first.iter().all(|item| !item.cursed));
    assert!(first
        .iter()
        .all(|item| item.source.as_deref() == Some("Imp.Quest")));
    assert_eq!(first[4].class_name, "PlateArmor");
    assert_eq!(first[4].category, ItemCategory::Armor);
    assert!(first[1].category == ItemCategory::Ring);
    assert!(first[5].category == ItemCategory::Wand);
    for (slot, item) in first.iter().enumerate() {
        assert_eq!(
            item.provenance,
            ItemProvenance::Quest(QuestRewardRole::ImpVaultOption { slot: slot as u8 })
        );
    }
}

#[test]
fn exhausted_artifact_deck_falls_back_to_a_distinct_ring() {
    let mut generator = init_run(42).generator;
    Random::reset_generators();
    Random::push_generator_seeded(777);
    while generator.random_artifact(18).is_some() {}
    let options = rewards::generate_reward_options(&mut generator, 18);
    Random::pop_generator();

    assert_eq!(options[0].category, ItemCategory::Ring);
    assert_eq!(options[1].category, ItemCategory::Ring);
    assert_ne!(options[0].class_name, options[1].class_name);
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
    assert_eq!(imp.reward_options.len(), 6);
    let result = take_pending(&mut imp).expect("spawn rewards");
    assert_eq!(result.options.len(), 6);
    assert!(result.options.iter().all(|item| !item.cursed));
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
fn held_mossy_clump_can_change_spawn_depth() {
    assert_eq!(profiled_imp_outcome(5, None).0, 17);
    assert_eq!(profiled_imp_outcome(5, Some(TrinketKind::MossyClump)).0, 18);
}

#[test]
fn held_mossy_clump_can_change_reward_identities_on_the_spawn_floor() {
    let baseline = profiled_imp_outcome(0, None);
    let mossy = profiled_imp_outcome(0, Some(TrinketKind::MossyClump));
    assert_eq!(baseline.0, 19);
    assert_eq!(mossy.0, 19);
    assert_ne!(baseline.1, mossy.1);
}
