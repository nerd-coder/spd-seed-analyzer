//! T2 vault treasure rooms: single enemy, bookcase, flame ellipse.

use crate::items::model::ItemCategory;
use crate::level::terrain::{BOOKSHELF, EMPTY, EMPTY_SP, PEDESTAL, WALL};
use crate::random::Random;

use super::super::geometry;
use super::super::paint::PaintCtx;

pub(in crate::level::vault) fn paint_single_enemy(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_ellipse_margin(ctx.map, &room, 3, EMPTY);
    let entrance = ctx.entrance_door(ri);
    geometry::draw_inside(ctx.map, &room, entrance, 3, EMPTY);

    let _ = ctx.create_t2_mob();
    let mut enemy = ctx.point_cell(geometry::center(&room)) as i32;
    let w = ctx.width();
    let treasure = if entrance.x == room.left {
        let pos = enemy + 2;
        enemy += 1;
        pos
    } else if entrance.y == room.top {
        let pos = enemy + 2 * w;
        enemy += w;
        pos
    } else if entrance.x == room.right {
        let pos = enemy - 2;
        enemy -= 1;
        pos
    } else {
        let pos = enemy - 2 * w;
        enemy -= w;
        pos
    };
    ctx.create_equipment(2);
    ctx.occupy_heap(treasure as usize);
    ctx.occupy_mob(enemy as usize);

    let n4 = geometry::neighbours4(w);
    let offset = loop {
        let i = n4[Random::int_max(4) as usize];
        let cell = treasure + i;
        if cell >= 0
            && (cell as usize) < ctx.map.len()
            && ctx.map.map[cell as usize] != WALL
            && cell != enemy
        {
            break cell as usize;
        }
    };
    ctx.create_consumable(2);
    ctx.occupy_heap(offset);
    ctx.set_regular_doors(ri);
}

pub(in crate::level::vault) fn paint_bookcase(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    let w = ctx.width();
    let (first, second) = if entrance.x == room.left || entrance.x == room.right {
        let book_top = geometry::gate(room.top + 1, entrance.y - 2, room.bottom - 5);
        geometry::fill_wh(ctx.map, room.left + 1, book_top, 9, 5, BOOKSHELF);
        geometry::fill_wh(ctx.map, room.left + 2, book_top + 1, 3, 3, EMPTY_SP);
        geometry::fill_wh(ctx.map, room.left + 6, book_top + 1, 3, 3, EMPTY_SP);
        if entrance.x == room.left {
            (
                ctx.cell(room.left + 3, book_top + 2),
                ctx.cell(room.right - 3, book_top + 2),
            )
        } else {
            (
                ctx.cell(room.right - 3, book_top + 2),
                ctx.cell(room.left + 3, book_top + 2),
            )
        }
    } else {
        let book_left = geometry::gate(room.left + 1, entrance.x - 2, room.right - 5);
        geometry::fill_wh(ctx.map, book_left, room.top + 1, 5, 9, BOOKSHELF);
        geometry::fill_wh(ctx.map, book_left + 1, room.top + 2, 3, 3, EMPTY_SP);
        geometry::fill_wh(ctx.map, book_left + 1, room.top + 6, 3, 3, EMPTY_SP);
        if entrance.y == room.top {
            (
                ctx.cell(book_left + 2, room.top + 3),
                ctx.cell(book_left + 2, room.bottom - 3),
            )
        } else {
            (
                ctx.cell(book_left + 2, room.bottom - 3),
                ctx.cell(book_left + 2, room.top + 3),
            )
        }
    };
    geometry::set_cell(ctx.map, first, PEDESTAL);
    geometry::set_cell(ctx.map, second, PEDESTAL);
    ctx.find_prize_any();
    ctx.occupy_heap(first);
    ctx.create_equipment(2);
    ctx.occupy_heap(second);
    ctx.find_t3_solve_or_consumable(2);
    let n8 = geometry::neighbours8(w);
    ctx.occupy_heap((second as i32 + n8[Random::int_max(8) as usize]) as usize);
    ctx.occupy_heap((second as i32 + n8[Random::int_max(8) as usize]) as usize);
    ctx.add_item_to_spawn("PotionOfLiquidFlame", ItemCategory::Potion);
    geometry::draw_inside(ctx.map, &room, entrance, 2, EMPTY_SP);
}

pub(in crate::level::vault) fn paint_flames(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_ellipse_margin(ctx.map, &room, 2, EMPTY);
    let c = geometry::center(&room);
    let w = ctx.width();
    let center = ctx.point_cell(c) as i32;
    let positions = [
        center - 3 * w,
        center + 3,
        center + 3 * w,
        center - 3,
        center - 3 * w,
        center + 3,
    ];
    let entrance = ctx.entrance_door(ri);
    let idx = if entrance.x == room.left {
        1
    } else if entrance.y == room.top {
        2
    } else if entrance.x == room.right {
        3
    } else {
        4
    };
    geometry::set_cell(ctx.map, positions[idx - 1] as usize, PEDESTAL);
    geometry::set_cell(ctx.map, positions[idx] as usize, PEDESTAL);
    geometry::set_cell(ctx.map, positions[idx + 1] as usize, PEDESTAL);
    ctx.create_equipment(2);
    ctx.occupy_heap(positions[idx] as usize);

    for x in room.left + 2..=room.right - 2 {
        for y in room.top + 2..=room.bottom - 2 {
            let cell = ctx.cell(x, y);
            if ctx.map.map[cell] == EMPTY {
                geometry::setup_flame_trap(ctx.map, cell, 1, 1, 1);
            }
        }
    }

    geometry::draw_inside(
        ctx.map,
        &room,
        entrance,
        if (c.x - entrance.x).abs() <= 1 || (c.y - entrance.y).abs() <= 1 {
            1
        } else {
            2
        },
        EMPTY,
    );

    ctx.find_t3_solve_or_consumable(2);
    let _ = Random::int_max(2);
    ctx.occupy_heap(positions[idx - 1] as usize);
    ctx.occupy_heap(positions[idx + 1] as usize);
    ctx.add_item_to_spawn("PotionOfPurity", ItemCategory::Potion);
    ctx.set_regular_doors(ri);
}
