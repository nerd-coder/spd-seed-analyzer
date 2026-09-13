use super::*;

pub(super) fn assert_aaz_sentry_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "AAA-AAA-AAZ" {
        return;
    }

    let expected_floor = fixture.floors.first().expect("AAA-AAZ floor-1 facts");
    let sentry_room = expected_floor
        .room_bounds
        .iter()
        .find(|room| room.class_name == "SentryRoom")
        .expect("AAA-AAZ SentryRoom bounds");
    let in_sentry_room = |cell: u32| {
        let x = cell as i32 % expected_floor.width as i32;
        let y = cell as i32 / expected_floor.width as i32;
        x > sentry_room.left
            && x < sentry_room.right
            && y > sentry_room.top
            && y < sentry_room.bottom
    };
    let expected_heap = expected_floor
        .final_heaps
        .iter()
        .find(|heap| heap.heap_type == "chest" && in_sentry_room(heap.cell))
        .expect("pinned AAA-AAZ SentryRoom chest");
    let heap = map
        .heaps
        .iter()
        .find(|heap| heap.cell == expected_heap.cell)
        .expect("pinned AAA-AAZ SentryRoom chest");
    assert_eq!(
        heap.heap_type, expected_heap.heap_type,
        "SentryRoom heap type in {context}"
    );
    assert_eq!(
        heap.items.len(),
        expected_heap.items.len(),
        "SentryRoom stack size in {context}"
    );
    let item = &heap.items[0];
    let expected_item = &expected_heap.items[0];
    assert_eq!(
        (
            item.class_name.as_str(),
            item.quantity,
            item.level,
            item.cursed
        ),
        (
            expected_item.class_name.as_str(),
            expected_item.quantity,
            expected_item.level,
            expected_item.cursed
        ),
        "pinned SentryRoom prize in {context}"
    );
    let expected_sentry = expected_floor
        .final_mobs
        .iter()
        .find(|mob| mob.class_name == "Sentry")
        .expect("pinned AAA-AAZ SentryRoom mob");
    assert!(
        map.mobs.iter().any(|mob| {
            mob.cell == expected_sentry.cell && mob.class_name == expected_sentry.class_name
        }),
        "pinned room-painted Sentry in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "SentryRoom must not emit legacy Room loot in {context}"
    );
}
