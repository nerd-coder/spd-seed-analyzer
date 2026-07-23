use super::*;

pub(super) fn assert_abc_magical_fire_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "ABC-DEF-GHI" {
        return;
    }

    let fire_room_heaps: Vec<_> = map
        .heaps
        .iter()
        .filter(|heap| matches!(heap.cell, 887 | 889 | 964))
        .map(|heap| {
            (
                heap.cell,
                heap.heap_type.as_str(),
                heap.items[0].class_name.as_str(),
            )
        })
        .collect();
    assert_eq!(
        fire_room_heaps,
        [
            (887, "heap", "PotionOfToxicGas"),
            (889, "heap", "Pasty"),
            (964, "heap", "Honeypot"),
        ],
        "pinned MagicalFireRoom heap associations in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "ABC-DEF-GHI no longer uses the legacy room-loot marker in {context}"
    );
}
