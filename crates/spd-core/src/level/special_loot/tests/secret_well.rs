use super::super::gardens::secret_well;
use super::test_room;
use crate::geom::Point;
use crate::level::terrain::{paint_minimal, CHASM, EMPTY, WALL, WELL};
use crate::random::Random;

#[test]
fn paints_chasm_ringed_well_opposite_hidden_entrance() {
    Random::reset_generators();
    Random::push_generator_seeded(5);
    let room = test_room("SecretWellRoom", 7, 7);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("well map");
    let loot = secret_well(&room, &mut map, Point::new(4, 0));
    Random::pop_generator();

    let at = |x: i32, y: i32| map.map[map.point_to_cell(x, y).unwrap()];
    assert_eq!(at(4, 5), WELL);
    for (x, y) in [(3, 4), (5, 4), (3, 5), (5, 5), (3, 6), (4, 6), (5, 6)] {
        assert_eq!(at(x, y), CHASM, "({x}, {y})");
    }
    for y in 0..=4 {
        assert_eq!(at(4, y), EMPTY, "path at y={y}");
    }
    assert_eq!(at(1, 1), WALL);
    assert_eq!(at(6, 2), WALL);

    assert_eq!(loot.len(), 1);
    assert_eq!(loot[0].item.source.as_deref(), Some("SecretWellRoom"));
    assert_eq!(map.known_blobs.len(), 1);
    let blob = &map.known_blobs[0];
    assert_eq!(blob.class_name, loot[0].item.class_name);
    assert!(blob.always_visible);
    let well_cell = map.point_to_cell(4, 5).unwrap();
    assert_eq!(blob.cells, vec![(well_cell, 1)]);
}
