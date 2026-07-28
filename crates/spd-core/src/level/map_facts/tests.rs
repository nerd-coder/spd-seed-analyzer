use super::*;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

#[test]
fn blob_projection_merges_seeded_cells_and_sorts_the_contract() {
    let mut room = Room::new(0, "EmptyRoom", RoomKind::Standard, 1, 4, 5, 5, 5, 5);
    room.left = 1;
    room.top = 1;
    room.right = 5;
    room.bottom = 5;
    let mut map = crate::level::terrain::paint_minimal(&[room]).expect("blob test map");

    map.record_blob_cell("ToxicGas", true, 20, 2);
    map.record_blob_cell("Alchemy", false, 12, 1);
    map.record_blob_cell("ToxicGas", true, 5, 3);
    map.record_blob_cell("Alchemy", false, 12, 4);
    map.record_blob_cell("Alchemy", false, 3, 2);

    assert_eq!(
        blobs(&map),
        [
            MapBlob {
                class_name: "Alchemy".into(),
                volume: 7,
                always_visible: false,
                cells: vec![
                    MapBlobCell { cell: 3, value: 2 },
                    MapBlobCell { cell: 12, value: 5 },
                ],
            },
            MapBlob {
                class_name: "ToxicGas".into(),
                volume: 5,
                always_visible: true,
                cells: vec![
                    MapBlobCell { cell: 5, value: 3 },
                    MapBlobCell { cell: 20, value: 2 },
                ],
            },
        ]
    );
}

#[test]
fn regular_floor_overlays_match_java_fixture_geometry() {
    for (seed, depth, expected) in [
        ("GFX-PZH-DCH", 12, vec![("QuestEntrance", 39, 28, 1, 1)]),
        (
            "GFX-PZH-DCH",
            17,
            vec![
                ("QuestEntrance", 20, 35, 5, 5),
                ("EntranceBarrier", 21, 36, 3, 3),
            ],
        ),
        (
            "GFX-PZH-DCH",
            21,
            vec![("CustomFloor", 20, 34, 5, 4), ("HiddenWell", 18, 35, 1, 1)],
        ),
    ] {
        let numeric = crate::parse_seed(seed).unwrap().numeric;
        let mut dungeon = crate::dungeon_from_run(crate::init_run(numeric));
        let mut level = None;
        for replay_depth in 1..=depth {
            dungeon.depth = replay_depth;
            level = Some(crate::level::create_level_partial(&mut dungeon));
        }
        let map = level.unwrap().map.expect("regular floor map");
        let actual: Vec<_> = map
            .custom_tiles
            .iter()
            .map(|layer| {
                (
                    layer.class_name.as_str(),
                    layer.x,
                    layer.y,
                    layer.width,
                    layer.height,
                )
            })
            .collect();
        assert_eq!(actual, expected, "{seed} depth {depth}");
        assert!(map
            .custom_tiles
            .iter()
            .all(|layer| layer.static_data.len() == (layer.width * layer.height) as usize));
    }
}
