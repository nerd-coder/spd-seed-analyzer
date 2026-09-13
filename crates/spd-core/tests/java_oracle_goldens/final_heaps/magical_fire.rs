use super::*;

pub(super) fn assert_abc_magical_fire_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "ABC-DEF-GHI" {
        return;
    }
    let Some(floor) = fixture.floors.first() else {
        return;
    };
    if !floor.rooms.iter().any(|room| room == "MagicalFireRoom") {
        return;
    }
    let expected: Vec<_> = floor
        .final_heaps
        .iter()
        .filter_map(|heap| {
            let item = heap.items.first()?;
            matches!(
                item.class_name.as_str(),
                "PotionOfToxicGas" | "Pasty" | "Honeypot"
            )
            .then_some((heap.cell, heap.heap_type.as_str(), item.class_name.as_str()))
        })
        .collect();
    if expected.is_empty() {
        return;
    }

    let fire_room_heaps: Vec<_> = map
        .heaps
        .iter()
        .filter(|heap| expected.iter().any(|(cell, _, _)| *cell == heap.cell))
        .map(|heap| {
            (
                heap.cell,
                heap.heap_type.as_str(),
                heap.items[0].class_name.as_str(),
            )
        })
        .collect();
    assert_eq!(
        fire_room_heaps, expected,
        "pinned MagicalFireRoom heap associations in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "ABC-DEF-GHI no longer uses the legacy room-loot marker in {context}"
    );
}
