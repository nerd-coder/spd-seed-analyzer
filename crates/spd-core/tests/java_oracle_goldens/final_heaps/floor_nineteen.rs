use super::*;

use std::ffi::OsStr;

#[test]
fn aaa_floor_nineteen_matches_through_final_mobs() {
    let name = OsStr::new("aaa-aaa-aaa-final-heaps-floor-19.json");
    let path = fixture_paths()
        .into_iter()
        .find(|path| path.file_name().is_some_and(|file| file == name))
        .expect("missing AAA floor-19 fixture");
    let fixture = read_fixture(&path);
    let expected = fixture.floors.first().expect("floor-19 oracle facts");

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1_i32..=19 {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("floor-19 replay");
    let mut rooms = actual.rooms.clone();
    rooms.sort();
    assert_eq!(rooms, expected.rooms, "floor-19 room classes");

    let bounds: Vec<_> = actual
        .room_bounds
        .iter()
        .map(|room| OracleRoomFact {
            class_name: room.class_name.clone(),
            left: room.left,
            top: room.top,
            right: room.right,
            bottom: room.bottom,
        })
        .collect();
    assert_eq!(bounds, expected.room_bounds, "floor-19 normalized bounds");
    assert_eq!(
        actual.pre_paint_rng_probe, expected.pre_paint_rng,
        "floor-19 pre-paint RNG boundary"
    );
    assert_eq!(
        actual.pre_mobs_rng_probe, expected.pre_mobs_rng,
        "floor-19 pre-mobs RNG boundary"
    );
    assert_eq!(
        actual.pre_items_rng_probe, expected.pre_items_rng,
        "floor-19 pre-items RNG boundary"
    );
    let mobs: Vec<_> = actual
        .map
        .as_ref()
        .expect("floor-19 map")
        .mobs
        .iter()
        .map(|mob| OracleMob {
            cell: mob.cell,
            class_name: mob.class_name.clone(),
        })
        .collect();
    assert_eq!(mobs, expected.final_mobs, "floor-19 final mobs");
}
