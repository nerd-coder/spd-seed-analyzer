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
