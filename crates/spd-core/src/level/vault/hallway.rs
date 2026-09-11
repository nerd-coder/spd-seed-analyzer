//! `VaultHallwayRoom.paint`

use crate::geom::Point;
use crate::level::terrain::{EMPTY, WALL};
use crate::random::Random;

use super::geometry;
use super::loot;
use super::paint::PaintCtx;

pub(super) fn paint(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    let wide = geometry::wide(&room);
    geometry::fill_room(ctx.map, &room, WALL);
    if wide {
        geometry::fill_wh(
            ctx.map,
            room.left + 1,
            room.top + 4,
            room.width() - 2,
            3,
            EMPTY,
        );
        geometry::fill_wh(
            ctx.map,
            room.left + 6,
            room.top + 1,
            1,
            room.height() - 2,
            EMPTY,
        );
        geometry::fill_wh(
            ctx.map,
            room.right - 7,
            room.top + 1,
            1,
            room.height() - 2,
            EMPTY,
        );
    } else {
        geometry::fill_wh(
            ctx.map,
            room.left + 4,
            room.top + 1,
            3,
            room.height() - 2,
            EMPTY,
        );
        geometry::fill_wh(
            ctx.map,
            room.left + 1,
            room.top + 6,
            room.width() - 2,
            1,
            EMPTY,
        );
        geometry::fill_wh(
            ctx.map,
            room.left + 1,
            room.bottom - 7,
            room.width() - 2,
            1,
            EMPTY,
        );
    }
    let c = geometry::center(&room);
    let mut loot_positions = if wide {
        vec![
            (room.left + 2 + c.y * ctx.width()) as usize,
            (room.right - 2 + c.y * ctx.width()) as usize,
        ]
    } else {
        vec![
            (c.x + (room.top + 2) * ctx.width()) as usize,
            (c.x + (room.bottom - 2) * ctx.width()) as usize,
        ]
    };
    let doors = ctx.door_points(ri);
    loot_positions.retain(|&cell| {
        doors
            .iter()
            .all(|d| geometry::chebyshev(ctx.map, cell, ctx.point_cell(*d)) > 6)
    });
    if loot_positions.is_empty() {
        loot_positions.push(ctx.point_cell(c));
    }
    if loot::find_prize_equipable(&mut ctx.dungeon.items_to_spawn).is_some() {
        ctx.occupy_heap(loot_positions[0]);
    }

    let _ = ctx.create_mob();
    let _ = Random::int_max(2);
    ctx.occupy_mob(ctx.point_cell(c));

    ctx.set_regular_doors(ri);
    for door in ctx.door_points(ri) {
        if wide {
            paint_wide_door(ctx, &room, c, door);
        } else {
            paint_tall_door(ctx, &room, c, door);
        }
    }
}

fn paint_wide_door(ctx: &mut PaintCtx<'_>, room: &crate::rooms::room::Room, c: Point, door: Point) {
    if door.x == room.left {
        geometry::draw_line(
            ctx.map,
            Point::new(door.x + 1, door.y),
            Point::new(room.left + 1, c.y),
            EMPTY,
        );
    } else if door.x == room.right {
        geometry::draw_line(
            ctx.map,
            Point::new(door.x - 1, door.y),
            Point::new(room.right - 1, c.y),
            EMPTY,
        );
    } else if door.x <= room.left + 3 || door.x >= room.right - 3 {
        geometry::draw_inside(ctx.map, room, door, 5, EMPTY);
    } else {
        let closest_x = if door.x < c.x || (door.x == c.x && Random::int_max(2) == 0) {
            room.left + 6
        } else {
            room.right - 7
        };
        if door.y == room.top {
            geometry::draw_line(
                ctx.map,
                Point::new(door.x, door.y + 1),
                Point::new(closest_x, door.y + 1),
                EMPTY,
            );
        } else {
            geometry::draw_line(
                ctx.map,
                Point::new(door.x, door.y - 1),
                Point::new(closest_x, door.y - 1),
                EMPTY,
            );
        }
    }
}

fn paint_tall_door(ctx: &mut PaintCtx<'_>, room: &crate::rooms::room::Room, c: Point, door: Point) {
    if door.y == room.top {
        geometry::draw_line(
            ctx.map,
            Point::new(door.x, door.y + 1),
            Point::new(c.x, room.top + 1),
            EMPTY,
        );
    } else if door.y == room.bottom {
        geometry::draw_line(
            ctx.map,
            Point::new(door.x, door.y - 1),
            Point::new(c.x, room.bottom - 1),
            EMPTY,
        );
    } else if door.y <= room.top + 3 || door.y >= room.bottom - 3 {
        geometry::draw_inside(ctx.map, room, door, 5, EMPTY);
    } else {
        let closest_y = if door.y < c.y || (door.y == c.y && Random::int_max(2) == 0) {
            room.top + 6
        } else {
            room.bottom - 7
        };
        if door.x == room.left {
            geometry::draw_line(
                ctx.map,
                Point::new(door.x + 1, door.y),
                Point::new(door.x + 1, closest_y),
                EMPTY,
            );
        } else {
            geometry::draw_line(
                ctx.map,
                Point::new(door.x - 1, door.y),
                Point::new(door.x - 1, closest_y),
                EMPTY,
            );
        }
    }
}
