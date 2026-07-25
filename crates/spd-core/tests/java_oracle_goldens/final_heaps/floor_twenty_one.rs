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
    assert_eq!(
        actual.pre_paint_rng_probe,
        [
            -1_716_804_766,
            -1_527_035_254,
            -349_652_823,
            -966_449_763,
            -421_397_997,
            -824_908_010,
            -138_537_706,
            1_360_956_948,
        ],
        "floor-21 post-ChasmRoom correction boundary"
    );
    assert_ne!(
        actual.pre_paint_rng_probe, expected.pre_paint_rng,
        "floor-21 still has a later init/build divergence"
    );
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
            "TunnelRoom",
        ],
        "floor-21 exact post-ChasmRoom room classes"
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
    assert_ne!(
        bounds, expected.room_bounds,
        "floor-21 FigureEightBuilder layout remains the next divergence"
    );
}
