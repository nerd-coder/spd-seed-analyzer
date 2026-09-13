use super::*;

pub(super) fn assert_hello_traps_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "hello" {
        return;
    }
    if !fixture
        .floors
        .first()
        .is_some_and(|floor| floor.rooms.iter().any(|room| room == "TrapsRoom"))
    {
        return;
    }
    if !fixture.floors.first().is_some_and(|floor| {
        floor
            .final_heaps
            .iter()
            .flat_map(|heap| &heap.items)
            .any(|item| item.class_name == "PlateArmor")
    }) {
        return;
    }

    let heap = map
        .heaps
        .iter()
        .find(|heap| heap.cell == 928)
        .expect("pinned hello TrapsRoom chest");
    assert_eq!(heap.heap_type, "chest", "TrapsRoom heap type in {context}");
    assert_eq!(heap.items.len(), 1, "TrapsRoom stack size in {context}");
    let item = &heap.items[0];
    assert_eq!(
        (
            item.class_name.as_str(),
            item.quantity,
            item.level,
            item.cursed
        ),
        ("PlateArmor", 1, 1, false),
        "pinned TrapsRoom prize in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "TrapsRoom must not emit legacy Room loot in {context}"
    );
}
