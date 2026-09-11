//! Cross, circle, laser, and alternating-fire vault rooms.

use crate::level::terrain::{EMPTY, PEDESTAL, WALL};
use crate::random::Random;

use super::geometry;
use super::paint::PaintCtx;

pub(super) fn paint_cross(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_insets(ctx.map, &room, 4, 1, 4, 1, EMPTY);
    geometry::fill_insets(ctx.map, &room, 1, 4, 1, 4, EMPTY);
    let c = geometry::center(&room);
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);
    ctx.set_regular_doors(ri);
}

pub(super) fn paint_circle(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 2, EMPTY);
    geometry::fill_insets(ctx.map, &room, 4, 1, 4, 1, EMPTY);
    geometry::fill_insets(ctx.map, &room, 1, 4, 1, 4, EMPTY);
    let c = geometry::center(&room);
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);
    let _ = Random::int_max(4);
    ctx.set_regular_doors(ri);
    for door in ctx.door_points(ri) {
        geometry::draw_inside(ctx.map, &room, door, 4, EMPTY);
    }
}

pub(super) fn paint_lasers(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 2, EMPTY);
    for door in ctx.door_points(ri) {
        geometry::draw_inside(ctx.map, &room, door, 2, EMPTY);
    }
    ctx.set_regular_doors(ri);

    for x in room.left + 2..=room.right - 2 {
        if geometry::terrain_at(ctx.map, x, room.top + 1) == WALL
            && geometry::terrain_at(ctx.map, x, room.bottom - 1) == WALL
        {
            if Random::int_max(2) == 0 {
                geometry::set(ctx.map, x, room.top + 1, PEDESTAL);
            } else {
                geometry::set(ctx.map, x, room.bottom - 1, PEDESTAL);
            }
            let after = Random::int_range_inclusive(3, 7);
            let _ = Random::int_range_inclusive(1, after);
        }
    }
    for y in room.top + 2..=room.bottom - 2 {
        if geometry::terrain_at(ctx.map, room.left + 1, y) == WALL
            && geometry::terrain_at(ctx.map, room.right - 1, y) == WALL
        {
            if Random::int_max(2) == 0 {
                geometry::set(ctx.map, room.left + 1, y, PEDESTAL);
            } else {
                geometry::set(ctx.map, room.right - 1, y, PEDESTAL);
            }
            let after = Random::int_range_inclusive(3, 7);
            let _ = Random::int_range_inclusive(1, after);
        }
    }
}

pub(super) fn paint_alternating_fire(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    ctx.set_regular_doors(ri);
    let c = geometry::center(&room);
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);
    ctx.create_equipment(0);
    ctx.occupy_heap(ctx.point_cell(c));
    let mut alternate = false;
    for x in room.left + 1..=room.right - 1 {
        for y in room.top + 1..=room.bottom - 1 {
            let cell = ctx.cell(x, y);
            if ctx.map.map[cell] != PEDESTAL {
                geometry::setup_flame_trap(ctx.map, cell, i32::from(alternate), 2, 1);
            }
            alternate = !alternate;
        }
    }
}
