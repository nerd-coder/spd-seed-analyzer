//! `VaultTokensRoom.paint`

use crate::level::terrain::{EMPTY, EMPTY_SP, LOCKED_DOOR, REGION_DECO_ALT, WALL};
use crate::random::Random;

use super::geometry::{self, CITY_PEDESTAL_TL, CITY_PEDESTAL_TR};
use super::paint::PaintCtx;

pub(super) fn paint(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    let depth = ctx.dungeon.depth;
    let wide = geometry::wide(&room);
    geometry::fill_room(ctx.map, &room, WALL);
    let c = geometry::center(&room);
    let doors = ctx.door_points(ri);

    if wide {
        let mut left_door = 0;
        let mut right_door = 0;
        for d in &doors {
            if d.x < c.x {
                left_door = 1;
                geometry::fill_wh(
                    ctx.map,
                    room.left + 1,
                    room.top + 1,
                    (room.width() - 2) / 2,
                    room.height() - 2,
                    EMPTY,
                );
            } else {
                right_door = 1;
                geometry::fill_wh(
                    ctx.map,
                    c.x + 1,
                    room.top + 1,
                    (room.width() - 2) / 2,
                    room.height() - 2,
                    EMPTY,
                );
            }
        }
        geometry::fill_diamond_wh(ctx.map, room.left + 3, room.top + 1, 9, 9, WALL);
        geometry::fill_diamond_wh(ctx.map, room.left + 9, room.top + 1, 9, 9, WALL);
        geometry::fill_diamond_wh(ctx.map, room.left + 5, room.top + 1, 9, 9, EMPTY_SP);
        geometry::fill_diamond_wh(ctx.map, room.left + 7, room.top + 1, 9, 9, EMPTY_SP);
        geometry::fill_wh(
            ctx.map,
            room.left + 4 - left_door,
            c.y,
            13 + left_door + right_door,
            1,
            EMPTY,
        );
        geometry::fill_wh(ctx.map, room.left + 4, c.y, 13, 1, EMPTY_SP);
    } else {
        let mut top_door = 0;
        let mut bottom_door = 0;
        for d in &doors {
            if d.y < c.y {
                top_door = 1;
                geometry::fill_wh(
                    ctx.map,
                    room.left + 1,
                    room.top + 1,
                    room.width() - 2,
                    (room.height() - 2) / 2,
                    EMPTY,
                );
            } else {
                bottom_door = 1;
                geometry::fill_wh(
                    ctx.map,
                    room.left + 1,
                    c.y + 1,
                    room.width() - 2,
                    (room.height() - 2) / 2,
                    EMPTY,
                );
            }
        }
        geometry::fill_diamond_wh(ctx.map, room.left + 1, room.top + 3, 9, 9, WALL);
        geometry::fill_diamond_wh(ctx.map, room.left + 1, room.top + 9, 9, 9, WALL);
        geometry::fill_diamond_wh(ctx.map, room.left + 1, room.top + 5, 9, 9, EMPTY_SP);
        geometry::fill_diamond_wh(ctx.map, room.left + 1, room.top + 7, 9, 9, EMPTY_SP);
        geometry::fill_wh(
            ctx.map,
            c.x,
            room.top + 4 - top_door,
            1,
            13 + top_door + bottom_door,
            EMPTY,
        );
        geometry::fill_wh(ctx.map, c.x, room.top + 4, 1, 13, EMPTY_SP);
    }

    geometry::fill_diamond_wh(ctx.map, c.x - 3, c.y - 3, 7, 7, WALL);
    geometry::fill_wh(ctx.map, c.x - 1, c.y - 1, 3, 3, EMPTY_SP);
    geometry::fill_wh(ctx.map, c.x - 2, c.y, 5, 1, EMPTY_SP);
    geometry::fill_wh(ctx.map, c.x, c.y - 1, 1, 5, EMPTY_SP);
    geometry::set(ctx.map, c.x - 1, c.y - 1, REGION_DECO_ALT);
    geometry::set(ctx.map, c.x + 1, c.y - 1, REGION_DECO_ALT);
    geometry::add_carpet(
        ctx.map,
        c.x - 1,
        c.y - 1,
        3,
        3,
        depth,
        &[(0, 0, CITY_PEDESTAL_TL), (2, 0, CITY_PEDESTAL_TR)],
    );

    geometry::set(ctx.map, c.x, c.y + 3, LOCKED_DOOR);
    // VaultTokenDoor / VaultMirror are NPCs; consume mirror reward RNG only.
    let seed = Random::long();
    Random::push_generator_seeded(seed);
    Random::pop_generator();

    if Random::int_max(2) == 0 {
        ctx.create_equipment(3);
        ctx.occupy_heap(ctx.cell(c.x - 2, c.y));
        ctx.create_consumable(3);
        ctx.occupy_heap(ctx.cell(c.x + 2, c.y));
    } else {
        ctx.create_equipment(3);
        ctx.occupy_heap(ctx.cell(c.x + 2, c.y));
        ctx.create_consumable(3);
        ctx.occupy_heap(ctx.cell(c.x - 2, c.y));
    }

    let mut returned = Vec::new();
    let enemy = loop {
        let enemy = ctx.create_mob();
        if enemy.large() {
            returned.push(enemy);
            continue;
        }
        break enemy;
    };
    let _ = Random::int_max(2);
    let idx = Random::int_max(4) as usize;
    let wander = [
        ctx.cell(c.x - 4, c.y),
        ctx.cell(c.x, c.y - 4),
        ctx.cell(c.x + 4, c.y),
        ctx.cell(c.x, c.y + 4),
    ];
    ctx.occupy_mob(wander[idx]);
    let _ = enemy;
    for mob in returned {
        ctx.return_mob(mob);
    }
}
