//! Cross, circle, laser, and alternating-fire vault rooms.

use crate::level::terrain::{EMPTY, PEDESTAL, WALL};
use crate::random::Random;

use super::geometry;
use super::lasers::LaserSpec;
use super::paint::{PaintCtx, HEAP};
use super::sentries;

pub(super) fn paint_cross(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_insets(ctx.map, &room, 4, 1, 4, 1, EMPTY);
    geometry::fill_insets(ctx.map, &room, 1, 4, 1, 4, EMPTY);
    let c = geometry::center(&room);
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);
    ctx.place_sentry(sentries::cross(ctx.point_cell(c), ctx.width()));
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
    let variant = Random::int_max(3);
    ctx.place_sentry(sentries::circle(ctx.point_cell(c), ctx.width(), variant));
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

    let w = ctx.width();
    for x in room.left + 2..=room.right - 2 {
        if geometry::terrain_at(ctx.map, x, room.top + 1) == WALL
            && geometry::terrain_at(ctx.map, x, room.bottom - 1) == WALL
        {
            let (y, step) = if Random::int_max(2) == 0 {
                (room.top + 1, w)
            } else {
                (room.bottom - 1, -w)
            };
            place_room_laser(ctx, x, y, step);
        }
    }
    for y in room.top + 2..=room.bottom - 2 {
        if geometry::terrain_at(ctx.map, room.left + 1, y) == WALL
            && geometry::terrain_at(ctx.map, room.right - 1, y) == WALL
        {
            let (x, step) = if Random::int_max(2) == 0 {
                (room.left + 1, 1)
            } else {
                (room.right - 1, -1)
            };
            place_room_laser(ctx, x, y, step);
        }
    }
}

/// `afterShotCooldown = IntRange(7, 9)`, then `curCooldown = IntRange(1, after)`.
fn place_room_laser(ctx: &mut PaintCtx<'_>, x: i32, y: i32, step: i32) {
    let cell = ctx.cell(x, y);
    let after = Random::int_range_inclusive(7, 9);
    let cur = Random::int_range_inclusive(1, after);
    let target = (cell as i32 + step) as usize;
    ctx.place_laser(x, y, Some(LaserSpec::room(cell, target, cur, after)));
}

pub(super) fn paint_alternating_fire(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    ctx.set_regular_doors(ri);
    let c = geometry::center(&room);
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);
    let prize = ctx.create_equipment(0);
    ctx.drop(ctx.point_cell(c), HEAP, prize);
    let mut alternate = false;
    for x in room.left + 2..=room.right - 2 {
        for y in room.top + 2..=room.bottom - 2 {
            let cell = ctx.cell(x, y);
            if ctx.map.map[cell] != PEDESTAL {
                geometry::setup_flame_trap(ctx.map, cell, i32::from(alternate), 2, 1);
            }
            alternate = !alternate;
        }
    }
}
