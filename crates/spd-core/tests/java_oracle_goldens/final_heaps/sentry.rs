use super::*;

pub(super) fn assert_aaz_sentry_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "AAA-AAA-AAZ" {
        return;
    }

    let heap = map
        .heaps
        .iter()
        .find(|heap| heap.cell == 717)
        .expect("pinned AAA-AAZ SentryRoom chest");
    assert_eq!(heap.heap_type, "chest", "SentryRoom heap type in {context}");
    assert_eq!(heap.items.len(), 1, "SentryRoom stack size in {context}");
    let item = &heap.items[0];
    assert_eq!(
        (
            item.class_name.as_str(),
            item.quantity,
            item.level,
            item.cursed
        ),
        ("ScaleArmor", 1, 1, false),
        "pinned SentryRoom prize in {context}"
    );
    assert!(
        map.mobs
            .iter()
            .any(|mob| mob.cell == 819 && mob.class_name == "Sentry"),
        "pinned room-painted Sentry in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "SentryRoom must not emit legacy Room loot in {context}"
    );
}
