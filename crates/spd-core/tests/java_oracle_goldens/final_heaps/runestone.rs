use super::*;

pub(super) fn assert_aaa_afu_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "AAA-AAA-AFU" {
        return;
    }

    let expected = fixture
        .floors
        .first()
        .expect("AAA-AFU floor-1 oracle facts")
        .final_heaps
        .iter()
        .filter_map(|heap| {
            let item = heap.items.first()?;
            matches!(
                item.class_name.as_str(),
                "StoneOfBlast" | "StoneOfDeepSleep"
            )
            .then_some((heap.cell, heap.heap_type.as_str(), item.class_name.as_str()))
        })
        .collect::<Vec<_>>();
    let runestones: Vec<_> = map
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
        runestones, expected,
        "pinned RunestoneRoom heap associations in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "AAA-AFU no longer uses the legacy room-loot marker in {context}"
    );
}
