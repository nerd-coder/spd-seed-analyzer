use super::*;

#[test]
fn hkt_floor_seven_create_mobs_and_wandmaker_match_oracle() {
    let path = fixture_paths()
        .into_iter()
        .find(|path| {
            path.file_name()
                .is_some_and(|name| name == "hkt-jzn-xqq-final-heaps-floor-7.json")
        })
        .expect("HKT floor-7 schema-v3 fixture");
    let fixture = read_fixture(&path);
    assert_eq!(fixture.floors.len(), 1);
    let expected = fixture.floors.first().expect("floor-7 oracle facts");

    assert_eq!(fixture.schema_version, FINAL_HEAPS_SCHEMA_VERSION);
    assert_eq!(fixture.contract.as_deref(), Some("final_placed_heaps"));
    assert_eq!(fixture.input.depths, [7]);
    assert_eq!(expected.depth, 7);
    assert_eq!(
        expected.quest_rewards,
        [
            OracleItem {
                class_name: "WandOfPrismaticLight".into(),
                quantity: 1,
                level: 1,
                cursed: false,
            },
            OracleItem {
                class_name: "WandOfCorrosion".into(),
                quantity: 1,
                level: 1,
                cursed: false,
            },
        ],
        "pinned Java Wandmaker rewards"
    );

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1..=7 {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("floor-7 Rust facts");
    let mut actual_rooms = actual.rooms.clone();
    actual_rooms.sort();
    assert_eq!(actual_rooms, expected.rooms, "HKT floor-7 room classes");
    assert_eq!(
        actual.pre_paint_rng_probe, expected.pre_paint_rng,
        "HKT floor-7 pre-paint RNG boundary"
    );
    assert_eq!(
        actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
        "HKT floor-7 pre-mobs RNG boundary"
    );
    assert_eq!(
        actual.pre_items_rng_probe, expected.pre_items_rng,
        "HKT floor-7 pre-items RNG boundary"
    );
    let actual_mobs: Vec<_> = actual
        .map
        .as_ref()
        .expect("HKT floor-7 map")
        .mobs
        .iter()
        .map(|mob| OracleMob {
            cell: mob.cell,
            class_name: mob.class_name.clone(),
        })
        .collect();
    assert_eq!(actual_mobs, expected.final_mobs, "HKT floor-7 mobs");
    let actual_rewards: Vec<_> = actual
        .placed_items
        .iter()
        .filter(|item| item.source.as_deref() == Some("Wandmaker.Quest"))
        .map(|item| OracleItem {
            class_name: item.class_name.clone(),
            quantity: item.quantity,
            level: item.level,
            cursed: item.cursed,
        })
        .collect();

    assert_eq!(
        actual_rewards, expected.quest_rewards,
        "HKT floor-7 Wandmaker rewards"
    );
    assert_eq!(
        actual.quests,
        ["Old Wandmaker (Elemental Embers) — wand Of Prismatic Light +1 / wand Of Corrosion +1"],
        "HKT floor-7 Wandmaker report summary"
    );
}
