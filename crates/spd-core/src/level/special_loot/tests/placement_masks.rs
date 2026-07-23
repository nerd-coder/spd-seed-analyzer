//! Source-pinned room placement predicate coverage.

use super::super::quest_rooms::ritual_site_setup;
use super::test_room;
use crate::level::terrain::{paint_minimal, CUSTOM_DECO_EMPTY};
use crate::random::Random;

#[test]
fn ritual_site_blocks_exact_chebyshev_radius_without_extra_rng() {
    let mut room = test_room("RitualSiteRoom", 9, 9);
    room.kind = crate::rooms::types::RoomKind::Standard;
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("map");
    let mut items = Vec::new();

    Random::push_generator_seeded(0xC4AD1E);
    ritual_site_setup(&room, &mut map, &mut items);
    let actual_next = Random::int();
    Random::pop_generator();

    Random::push_generator_seeded(0xC4AD1E);
    let center = room.as_rect().center_room();
    let expected_next = Random::int();
    Random::pop_generator();

    assert_eq!(
        actual_next, expected_next,
        "predicate mask must not use RNG"
    );
    assert_eq!(items.len(), 4);
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let cell = map.point_to_cell(x, y).expect("room cell");
            let blocked = (x - center.x).abs().max((y - center.y).abs()) < 2;
            assert_eq!(map.item_allowed[cell], !blocked);
            assert_eq!(map.character_allowed[cell], !blocked);
            assert_eq!(map.map[cell] == CUSTOM_DECO_EMPTY, blocked);
        }
    }
}
