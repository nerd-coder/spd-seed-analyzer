use super::*;

pub(super) fn assert_aaa_afu_facts(
    fixture: &OracleFixture,
    map: &spd_core::report::FloorMap,
    context: &impl std::fmt::Display,
) {
    if fixture.input.seed != "AAA-AAA-AFU" {
        return;
    }

    let runestones: Vec<_> = map
        .heaps
        .iter()
        .filter(|heap| matches!(heap.cell, 542 | 579))
        .map(|heap| {
            (
                heap.cell,
                heap.heap_type.as_str(),
                heap.items[0].class_name.as_str(),
            )
        })
        .collect();
    assert_eq!(
        runestones,
        [
            (542, "heap", "StoneOfBlast"),
            (579, "heap", "StoneOfDeepSleep"),
        ],
        "pinned RunestoneRoom heap associations in {context}"
    );
    assert!(
        map.markers.iter().all(|marker| marker.label != "Room loot"),
        "AAA-AFU no longer uses the legacy room-loot marker in {context}"
    );
}
