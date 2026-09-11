//! T1 vault treasure rooms: flame path, laser gauntlet, circle scan.

use crate::level::terrain::{EMPTY, EMPTY_SP, PEDESTAL, STATUE, WALL};
use crate::random::Random;

use super::super::geometry;
use super::super::paint::PaintCtx;

const LEFT: i32 = 0;
const UP: i32 = 1;
const RIGHT: i32 = 2;
const DOWN: i32 = 3;

pub(in crate::level::vault) fn paint_flame_path(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    let c = geometry::center(&room);

    let left_side = (room.left + 1, room.top + 1, room.left + 3, room.bottom - 1);
    let top_side = (room.left + 1, room.top + 1, room.right - 1, room.top + 3);
    let right_side = (
        room.right - 3,
        room.top + 1,
        room.right - 1,
        room.bottom - 1,
    );
    let bottom_side = (
        room.left + 1,
        room.bottom - 3,
        room.right - 1,
        room.bottom - 1,
    );

    let treasure = if entrance.x == room.left {
        if entrance.y < c.y {
            fill_group(ctx, bottom_side, RIGHT);
            fill_group(ctx, right_side, UP);
            fill_group(ctx, top_side, LEFT);
            geometry::set(ctx.map, room.left + 1, room.top + 6, EMPTY_SP);
        } else {
            fill_group(ctx, top_side, RIGHT);
            fill_group(ctx, right_side, DOWN);
            fill_group(ctx, bottom_side, LEFT);
            geometry::set(ctx.map, room.left + 1, room.top + 4, EMPTY_SP);
        }
        (room.left + 1, room.top + 5, room.left + 5, room.top + 5)
    } else if entrance.y == room.top {
        if entrance.x < c.x {
            fill_group(ctx, right_side, DOWN);
            fill_group(ctx, bottom_side, LEFT);
            fill_group(ctx, left_side, UP);
            geometry::set(ctx.map, room.left + 6, room.top + 1, EMPTY_SP);
        } else {
            fill_group(ctx, left_side, DOWN);
            fill_group(ctx, bottom_side, RIGHT);
            fill_group(ctx, right_side, UP);
            geometry::set(ctx.map, room.left + 4, room.top + 1, EMPTY_SP);
        }
        (room.left + 5, room.top + 1, room.left + 5, room.top + 5)
    } else if entrance.x == room.right {
        if entrance.y < c.y {
            fill_group(ctx, bottom_side, LEFT);
            fill_group(ctx, left_side, UP);
            fill_group(ctx, top_side, RIGHT);
            geometry::set(ctx.map, room.right - 1, room.top + 6, EMPTY_SP);
        } else {
            fill_group(ctx, top_side, LEFT);
            fill_group(ctx, left_side, DOWN);
            fill_group(ctx, bottom_side, RIGHT);
            geometry::set(ctx.map, room.right - 1, room.top + 4, EMPTY_SP);
        }
        (room.right - 5, room.top + 5, room.right - 1, room.top + 5)
    } else if entrance.x < c.x {
        fill_group(ctx, right_side, UP);
        fill_group(ctx, top_side, LEFT);
        fill_group(ctx, left_side, DOWN);
        geometry::set(ctx.map, room.left + 6, room.bottom - 1, EMPTY_SP);
        (
            room.left + 5,
            room.bottom - 5,
            room.left + 5,
            room.bottom - 1,
        )
    } else {
        fill_group(ctx, left_side, UP);
        fill_group(ctx, top_side, RIGHT);
        fill_group(ctx, right_side, DOWN);
        geometry::set(ctx.map, room.left + 4, room.bottom - 1, EMPTY_SP);
        (
            room.left + 5,
            room.bottom - 5,
            room.left + 5,
            room.bottom - 1,
        )
    };

    // `width()+1` because `Rect.getPoints` is inclusive of right/bottom.
    geometry::fill_wh(
        ctx.map,
        treasure.0,
        treasure.1,
        treasure.2 - treasure.0 + 1,
        treasure.3 - treasure.1 + 1,
        EMPTY_SP,
    );
    drop_t1_triple(ctx, treasure);
}

fn fill_group(ctx: &mut PaintCtx<'_>, space: (i32, i32, i32, i32), dir: i32) {
    let (left, top, right, bottom) = space;
    let mut prior = Vec::new();
    if dir == LEFT {
        for y in top..=bottom {
            let ofs = unique_ofs(&prior);
            for (delay, x) in (left..=right).rev().enumerate() {
                geometry::setup_flame_trap(ctx.map, ctx.cell(x, y), delay as i32 + ofs, 5, 2);
            }
            prior.push(ofs);
        }
    } else if dir == RIGHT {
        for y in top..=bottom {
            let ofs = unique_ofs(&prior);
            for (delay, x) in (left..=right).enumerate() {
                geometry::setup_flame_trap(ctx.map, ctx.cell(x, y), delay as i32 + ofs, 5, 2);
            }
            prior.push(ofs);
        }
    } else if dir == UP {
        for x in left..=right {
            let ofs = unique_ofs(&prior);
            for (delay, y) in (top..=bottom).rev().enumerate() {
                geometry::setup_flame_trap(ctx.map, ctx.cell(x, y), delay as i32 + ofs, 5, 2);
            }
            prior.push(ofs);
        }
    } else {
        for x in left..=right {
            let ofs = unique_ofs(&prior);
            for (delay, y) in (top..=bottom).enumerate() {
                geometry::setup_flame_trap(ctx.map, ctx.cell(x, y), delay as i32 + ofs, 5, 2);
            }
            prior.push(ofs);
        }
    }
}

fn unique_ofs(prior: &[i32]) -> i32 {
    loop {
        let ofs = Random::int_max(5);
        if !prior.contains(&ofs) {
            return ofs;
        }
    }
}

pub(in crate::level::vault) fn paint_laser(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    let w = ctx.width();
    let item_place = if entrance.x == room.left || entrance.x == room.right {
        let area_top = geometry::gate(room.top + 2, entrance.y - 2, room.bottom - 6);
        geometry::fill_wh(ctx.map, room.left + 1, area_top, 9, 5, EMPTY);
        if entrance.x == room.left {
            for x in ((room.left + 2)..(room.right - 1)).rev() {
                laser_pair_vertical(ctx, x, area_top, room.right - x, w, 5);
            }
            (room.right - 1, area_top, room.right - 1, area_top + 4)
        } else {
            for x in (room.left + 2)..(room.right - 1) {
                laser_pair_vertical(ctx, x, area_top, x - room.left, w, 5);
            }
            (room.left + 1, area_top, room.left + 1, area_top + 4)
        }
    } else {
        let area_left = geometry::gate(room.left + 2, entrance.x - 2, room.right - 6);
        geometry::fill_wh(ctx.map, area_left, room.top + 1, 5, 9, EMPTY);
        if entrance.y == room.top {
            for y in ((room.top + 2)..(room.bottom - 1)).rev() {
                laser_pair_horizontal(ctx, area_left, y, room.bottom - y, w, 5);
            }
            (area_left, room.bottom - 1, area_left + 4, room.bottom - 1)
        } else {
            for y in (room.top + 2)..(room.bottom - 1) {
                laser_pair_horizontal(ctx, area_left, y, y - room.top, w, 5);
            }
            (area_left, room.top + 1, area_left + 4, room.top + 1)
        }
    };
    geometry::draw_inside(ctx.map, &room, entrance, 1, EMPTY);
    geometry::fill_wh(
        ctx.map,
        item_place.0,
        item_place.1,
        item_place.2 - item_place.0 + 1,
        item_place.3 - item_place.1 + 1,
        EMPTY_SP,
    );
    drop_t1_triple(ctx, item_place);
}

fn laser_pair_vertical(
    ctx: &mut PaintCtx<'_>,
    x: i32,
    area_top: i32,
    _cooldown: i32,
    _w: i32,
    second_row: i32,
) {
    geometry::set(ctx.map, x, area_top - 1, PEDESTAL);
    geometry::set(ctx.map, x, area_top + second_row, PEDESTAL);
}

fn laser_pair_horizontal(
    ctx: &mut PaintCtx<'_>,
    area_left: i32,
    y: i32,
    _cooldown: i32,
    _w: i32,
    second_row: i32,
) {
    geometry::set(ctx.map, area_left - 1, y, PEDESTAL);
    geometry::set(ctx.map, area_left + second_row, y, PEDESTAL);
}

pub(in crate::level::vault) fn paint_circle_scan(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 2, EMPTY);
    geometry::fill_insets(ctx.map, &room, 4, 1, 4, 1, EMPTY);
    geometry::fill_insets(ctx.map, &room, 1, 4, 1, 4, EMPTY);
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    geometry::draw_inside(ctx.map, &room, entrance, 3, EMPTY);
    let c = geometry::center(&room);
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);

    let treasure = if entrance.x == room.left {
        geometry::set(ctx.map, c.x + 2, c.y, WALL);
        geometry::set(ctx.map, c.x + 1, c.y, STATUE);
        geometry::set(ctx.map, c.x - 1, c.y, STATUE);
        let mut treasure = (room.left + 1, c.y - 1, c.x - 2, c.y + 1);
        geometry::fill_wh(
            ctx.map,
            treasure.0,
            treasure.1,
            treasure.2 - treasure.0 + 1,
            treasure.3 - treasure.1 + 1,
            WALL,
        );
        treasure.2 -= 1;
        if entrance.y < c.y {
            treasure.1 += 1;
        } else {
            treasure.3 -= 1;
        }
        treasure
    } else if entrance.y == room.top {
        geometry::set(ctx.map, c.x, c.y + 2, WALL);
        geometry::set(ctx.map, c.x, c.y + 1, STATUE);
        geometry::set(ctx.map, c.x, c.y - 1, STATUE);
        let mut treasure = (c.x - 1, room.top + 1, c.x + 1, room.top + 3);
        geometry::fill_wh(
            ctx.map,
            treasure.0,
            treasure.1,
            treasure.2 - treasure.0 + 1,
            treasure.3 - treasure.1 + 1,
            WALL,
        );
        treasure.3 -= 1;
        if entrance.x < c.x {
            treasure.0 += 1;
        } else {
            treasure.2 -= 1;
        }
        treasure
    } else if entrance.x == room.right {
        geometry::set(ctx.map, c.x - 2, c.y, WALL);
        geometry::set(ctx.map, c.x - 1, c.y, STATUE);
        geometry::set(ctx.map, c.x + 1, c.y, STATUE);
        let mut treasure = (room.right - 3, c.y - 1, room.right - 1, c.y + 1);
        geometry::fill_wh(
            ctx.map,
            treasure.0,
            treasure.1,
            treasure.2 - treasure.0 + 1,
            treasure.3 - treasure.1 + 1,
            WALL,
        );
        treasure.0 += 1;
        if entrance.y < c.y {
            treasure.1 += 1;
        } else {
            treasure.3 -= 1;
        }
        treasure
    } else {
        geometry::set(ctx.map, c.x, c.y - 2, WALL);
        geometry::set(ctx.map, c.x, c.y - 1, STATUE);
        geometry::set(ctx.map, c.x, c.y + 1, STATUE);
        let mut treasure = (c.x - 1, room.bottom - 3, c.x + 1, room.bottom - 1);
        geometry::fill_wh(
            ctx.map,
            treasure.0,
            treasure.1,
            treasure.2 - treasure.0 + 1,
            treasure.3 - treasure.1 + 1,
            WALL,
        );
        treasure.1 += 1;
        if entrance.x < c.x {
            treasure.0 += 1;
        } else {
            treasure.2 -= 1;
        }
        treasure
    };
    geometry::fill_wh(
        ctx.map,
        treasure.0,
        treasure.1,
        treasure.2 - treasure.0 + 1,
        treasure.3 - treasure.1 + 1,
        EMPTY_SP,
    );
    drop_t1_triple(ctx, treasure);
}

fn drop_t1_triple(ctx: &mut PaintCtx<'_>, rect: (i32, i32, i32, i32)) {
    // First cell is drawn before createEquipment; a spent-tier refill would steal that Int().
    let first = ctx.occupy_random_rect(rect.0, rect.1, rect.2, rect.3, &[]);
    ctx.create_equipment(1);
    ctx.find_t2_solve_or_consumable(1);
    let second = ctx.occupy_random_rect(rect.0, rect.1, rect.2, rect.3, &[first]);
    ctx.occupy_random_rect(rect.0, rect.1, rect.2, rect.3, &[first, second]);
}
