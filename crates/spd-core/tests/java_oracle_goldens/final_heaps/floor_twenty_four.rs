use super::floor_twenty_three::assert_halls_paint_trace;

#[test]
fn aaa_floor_twenty_four_halls_paint_trace_matches_preserved_run() {
    assert_halls_paint_trace(
        "AAA-AAA-AAA",
        "aaa-aaa-aaa-floor-24-halls-paint.json",
        24,
        0,
        18,
        None,
    );
}

#[test]
fn abc_floor_twenty_four_halls_paint_trace_matches_preserved_run() {
    assert_halls_paint_trace(
        "ABC-DEF-GHI",
        "abc-def-ghi-floor-24-halls-paint.json",
        24,
        0,
        15,
        None,
    );
}
