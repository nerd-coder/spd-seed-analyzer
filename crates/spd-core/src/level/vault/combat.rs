//! Ring / pillar / enemy-center / quadrant / simple-enemy vault rooms.

use crate::geom::Point;
use crate::level::terrain::{EMPTY, EMPTY_SP, STATUE, WALL};
use crate::random::Random;

use super::geometry;
use super::paint::PaintCtx;

pub(super) fn paint_ring(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    geometry::fill_margin(ctx.map, &room, 4, WALL);
    ctx.set_regular_doors(ri);
    let _ = ctx.create_mob();
    let clockwise = Random::int_max(2) == 0;
    let idx = Random::int_max(4) as usize;
    let wander = if clockwise {
        [
            ctx.cell(room.left + 2, room.top + 2),
            ctx.cell(room.right - 2, room.top + 2),
            ctx.cell(room.right - 2, room.bottom - 2),
            ctx.cell(room.left + 2, room.bottom - 2),
        ]
    } else {
        [
            ctx.cell(room.left + 2, room.bottom - 2),
            ctx.cell(room.right - 2, room.bottom - 2),
            ctx.cell(room.right - 2, room.top + 2),
            ctx.cell(room.left + 2, room.top + 2),
        ]
    };
    ctx.occupy_mob(wander[idx]);
}

pub(super) fn paint_rings(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    geometry::fill_wh(ctx.map, room.left + 2, room.top + 2, 3, 3, WALL);
    geometry::fill_wh(ctx.map, room.right - 4, room.top + 2, 3, 3, WALL);
    geometry::fill_wh(ctx.map, room.left + 2, room.bottom - 4, 3, 3, WALL);
    geometry::fill_wh(ctx.map, room.right - 4, room.bottom - 4, 3, 3, WALL);
    ctx.set_regular_doors(ri);

    let mut returned = Vec::new();
    let enemy = loop {
        let enemy = ctx.create_mob();
        if enemy.large() {
            returned.push(enemy);
            continue;
        }
        break enemy;
    };
    let pos = loop {
        let p = room.random_margin(1);
        let cell = ctx.point_cell(p);
        if ctx.map.map[cell] != WALL {
            break cell;
        }
    };
    ctx.occupy_mob(pos);
    let mut wander = [
        ctx.cell(room.left + 1, room.top + 1),
        ctx.cell(room.left + 1, room.top + 5),
        ctx.cell(room.left + 1, room.top + 9),
        ctx.cell(room.left + 5, room.top + 1),
        ctx.cell(room.left + 5, room.top + 5),
        ctx.cell(room.left + 5, room.top + 9),
        ctx.cell(room.left + 9, room.top + 1),
        ctx.cell(room.left + 9, room.top + 5),
        ctx.cell(room.left + 9, room.top + 9),
    ];
    Random::shuffle(&mut wander);
    let _ = enemy;
    for mob in returned {
        ctx.return_mob(mob);
    }
}

pub(super) fn paint_long_rings(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    geometry::fill_margin(ctx.map, &room, 4, WALL);
    if geometry::wide(&room) {
        geometry::fill_insets(ctx.map, &room, 8, 4, 8, 4, EMPTY);
    } else {
        geometry::fill_insets(ctx.map, &room, 4, 8, 4, 8, EMPTY);
    }
    let c = geometry::center(&room);
    if loot::find_prize_equipable(&mut ctx.dungeon.items_to_spawn).is_some() {
        ctx.occupy_heap(ctx.point_cell(c));
    }
    for _ in 0..2 {
        let _ = ctx.create_mob();
        let mut prev = -1i32;
        let first = random_wander(ctx, &room, prev);
        prev = first as i32;
        ctx.occupy_mob(first);
        for _ in 1..20 {
            prev = random_wander(ctx, &room, prev) as i32;
        }
    }
    ctx.set_regular_doors(ri);
}

fn random_wander(ctx: &mut PaintCtx<'_>, room: &crate::rooms::room::Room, previous: i32) -> usize {
    let width = ctx.width();
    loop {
        let p = room.random_margin(1);
        let pos = ctx.point_cell(p) as i32;
        let tile = ctx.map.map[pos as usize];
        if tile == WALL || tile == crate::level::terrain::WALL_DECO {
            continue;
        }
        if java_cell_chebyshev(width, pos, previous) < 6 {
            continue;
        }
        return pos as usize;
    }
}

fn java_cell_chebyshev(width: i32, a: i32, b: i32) -> i32 {
    let ax = a % width;
    let ay = a / width;
    let bx = b % width;
    let by = b / width;
    (ax - bx).abs().max((ay - by).abs())
}

pub(super) fn paint_enemy_center(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    geometry::fill_margin(ctx.map, &room, 2, WALL);
    geometry::fill_margin(ctx.map, &room, 3, EMPTY);
    geometry::draw_line(
        ctx.map,
        Point::new(room.left + 1, room.top + 3),
        Point::new(room.right - 1, room.top + 3),
        EMPTY,
    );
    geometry::draw_line(
        ctx.map,
        Point::new(room.left + 1, room.bottom - 3),
        Point::new(room.right - 1, room.bottom - 3),
        EMPTY,
    );
    geometry::draw_line(
        ctx.map,
        Point::new(room.left + 3, room.top + 1),
        Point::new(room.left + 3, room.bottom - 1),
        EMPTY,
    );
    geometry::draw_line(
        ctx.map,
        Point::new(room.right - 3, room.top + 1),
        Point::new(room.right - 3, room.bottom - 1),
        EMPTY,
    );
    ctx.set_regular_doors(ri);
    let enemy = ctx.create_mob();
    let c = geometry::center(&room);
    let clockwise = Random::int_max(2) == 0;
    let wander = if clockwise {
        [
            ctx.cell(c.x - 1, c.y - 1),
            ctx.cell(c.x + 1, c.y - 1),
            ctx.cell(c.x + 1, c.y + 1),
            ctx.cell(c.x - 1, c.y + 1),
        ]
    } else {
        [
            ctx.cell(c.x - 1, c.y - 1),
            ctx.cell(c.x - 1, c.y + 1),
            ctx.cell(c.x + 1, c.y + 1),
            ctx.cell(c.x + 1, c.y - 1),
        ]
    };
    let idx = Random::int_max(4) as usize;
    ctx.occupy_mob(wander[idx]);
    ctx.create_equipment(enemy.tier());
    ctx.occupy_heap(ctx.point_cell(c));
}

pub(super) fn paint_quadrants(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    let c = geometry::center(&room);
    geometry::draw_inside(ctx.map, &room, Point::new(room.left, c.y), 3, WALL);
    geometry::draw_inside(ctx.map, &room, Point::new(room.right, c.y), 3, WALL);
    geometry::draw_inside(ctx.map, &room, Point::new(c.x, room.top), 3, WALL);
    geometry::draw_inside(ctx.map, &room, Point::new(c.x, room.bottom), 3, WALL);
    geometry::set(ctx.map, c.x, c.y, STATUE);
    ctx.set_regular_doors(ri);

    let mut spawn = vec![
        Point::new(room.left + 2, room.top + 2),
        Point::new(room.right - 2, room.top + 2),
        Point::new(room.right - 2, room.bottom - 2),
        Point::new(room.left + 2, room.bottom - 2),
    ];
    let doors = ctx.door_points(ri);
    spawn.retain(|p| doors.iter().all(|d| geometry::point_distance(*p, *d) > 3.0));
    if !spawn.is_empty() {
        let enemy = ctx.create_mob();
        let corner = *Random::element(&spawn).expect("spawn");
        ctx.occupy_mob(ctx.point_cell(corner));
        ctx.create_equipment(enemy.tier());
        let mut treasure = ctx.point_cell(corner) as i32;
        treasure += if corner.x < c.x { -1 } else { 1 };
        treasure += if corner.y < c.y {
            -ctx.width()
        } else {
            ctx.width()
        };
        ctx.occupy_heap(treasure as usize);
    }
}

pub(super) fn paint_simple_enemy(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY);
    let (enemy_pos, treasure_pos) = match Random::int_max(4) {
        0 => {
            geometry::fill_wh(ctx.map, room.left + 2, room.top + 2, 6, 6, WALL);
            geometry::fill_wh(ctx.map, room.left + 3, room.top + 3, 4, 4, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 4, room.top + 7, 2, 1, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 7, room.top + 4, 1, 2, EMPTY_SP);
            (
                ctx.cell(room.left + 4, room.top + 4),
                ctx.cell(room.left + 3, room.top + 3),
            )
        }
        1 => {
            geometry::fill_wh(ctx.map, room.left + 3, room.top + 2, 6, 6, WALL);
            geometry::fill_wh(ctx.map, room.left + 4, room.top + 3, 4, 4, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 5, room.top + 7, 2, 1, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 3, room.top + 4, 1, 2, EMPTY_SP);
            (
                ctx.cell(room.right - 4, room.top + 4),
                ctx.cell(room.right - 3, room.top + 3),
            )
        }
        2 => {
            geometry::fill_wh(ctx.map, room.left + 3, room.top + 3, 6, 6, WALL);
            geometry::fill_wh(ctx.map, room.left + 4, room.top + 4, 4, 4, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 5, room.top + 3, 2, 1, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 3, room.top + 5, 1, 2, EMPTY_SP);
            (
                ctx.cell(room.right - 4, room.bottom - 4),
                ctx.cell(room.right - 3, room.bottom - 3),
            )
        }
        _ => {
            geometry::fill_wh(ctx.map, room.left + 2, room.top + 3, 6, 6, WALL);
            geometry::fill_wh(ctx.map, room.left + 3, room.top + 4, 4, 4, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 4, room.top + 3, 2, 1, EMPTY_SP);
            geometry::fill_wh(ctx.map, room.left + 7, room.top + 5, 1, 2, EMPTY_SP);
            (
                ctx.cell(room.left + 4, room.bottom - 4),
                ctx.cell(room.left + 3, room.bottom - 3),
            )
        }
    };
    let mut returned = Vec::new();
    let enemy = loop {
        let enemy = ctx.create_mob();
        if enemy.tier() == 1 {
            returned.push(enemy);
            continue;
        }
        break enemy;
    };
    for mob in returned {
        ctx.return_mob(mob);
    }
    ctx.create_equipment(enemy.tier());
    ctx.occupy_heap(treasure_pos);
    ctx.set_regular_doors(ri);
    ctx.occupy_mob(enemy_pos);
}

use super::loot;
