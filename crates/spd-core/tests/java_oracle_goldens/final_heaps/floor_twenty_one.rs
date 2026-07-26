use super::*;

use std::ffi::OsStr;

#[test]
fn aaa_floor_twenty_one_pins_first_generation_divergence_fix() {
    let name = OsStr::new("aaa-aaa-aaa-final-heaps-floor-21.json");
    let path = fixture_paths()
        .into_iter()
        .find(|path| path.file_name().is_some_and(|file| file == name))
        .expect("missing AAA floor-21 fixture");
    let fixture = read_fixture(&path);
    let expected = fixture.floors.first().expect("floor-21 oracle facts");

    let mut dungeon = dungeon_from_run(init_run(fixture.input.numeric));
    let mut actual = None;
    for depth in 1_i32..=21 {
        dungeon.depth = depth;
        actual = Some(create_level_partial(&mut dungeon));
    }
    let actual = actual.expect("floor-21 replay");
    assert_eq!(actual.initial_forced_items[0].class_name, "Torch");
    assert_eq!(actual.initial_forced_items[1].class_name, "Torch");
    assert_eq!(
        actual.initial_forced_items[2].category,
        spd_core::items::model::ItemCategory::Food
    );
    assert_eq!(
        expected
            .final_heaps
            .iter()
            .flat_map(|heap| &heap.items)
            .filter(|item| item.class_name == "Torch")
            .count(),
        2,
        "pinned Java final heaps retain both Halls torches"
    );
    assert_eq!(actual.pre_paint_rng_probe, expected.pre_paint_rng);
    let mut rooms = actual.rooms.clone();
    rooms.sort();
    assert_eq!(
        rooms,
        [
            "ArmoryRoom",
            "ChasmExitRoom",
            "ChasmRoom",
            "CrystalVaultRoom",
            "DemonSpawnerRoom",
            "RegionDecoPatchEntranceRoom",
            "RegionDecoPatchRoom",
            "RitualRoom",
            "RitualRoom",
            "StripedRoom",
            "TunnelRoom",
            "TunnelRoom",
            "TunnelRoom",
            "TunnelRoom",
            "TunnelRoom",
            "TunnelRoom",
        ],
        "floor-21 exact room classes"
    );
    let ordinary_rooms = rooms
        .iter()
        .filter(|room| room.as_str() != "TunnelRoom")
        .collect::<Vec<_>>();
    let expected_ordinary_rooms = expected
        .rooms
        .iter()
        .filter(|room| room.as_str() != "TunnelRoom")
        .collect::<Vec<_>>();
    assert_eq!(
        ordinary_rooms, expected_ordinary_rooms,
        "floor-21 non-connection room selection now matches Java"
    );
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
    assert_eq!(bounds, expected.room_bounds);
}
