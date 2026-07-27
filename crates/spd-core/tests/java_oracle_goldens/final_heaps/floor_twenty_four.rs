use super::floor_twenty_three::assert_halls_paint_trace;
use spd_core::rooms::init_rooms::BuilderKind;

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

#[test]
fn gfx_floor_twenty_four_halls_paint_trace_matches_loop_builder_history() {
    assert_halls_paint_trace(
        "GFX-PZH-DCH",
        "gfx-pzh-dch-floor-24-halls-paint.json",
        24,
        0,
        20,
        Some(BuilderKind::Loop),
    );
}
