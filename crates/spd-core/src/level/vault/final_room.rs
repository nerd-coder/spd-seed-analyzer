//! `VaultFinalRoom.paint`

use crate::geom::Point;
use crate::level::terrain::{
    CUSTOM_DECO_EMPTY, DOOR, EMPTY_SP, LOCKED_DOOR, PEDESTAL, REGION_DECO, WALL,
};
use crate::random::Random;

use super::geometry::{self, CITY_PEDESTAL, CITY_QUEST_TEX};
use super::paint::PaintCtx;

pub(super) fn paint(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    let depth = ctx.dungeon.depth;
    let width = ctx.width();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_ellipse_margin(ctx.map, &room, 5, EMPTY_SP);
    let c = geometry::center(&room);

    ctx.map.record_custom_tile(
        "MarkerTiles",
        CITY_QUEST_TEX,
        (c.x - 4, c.y - 4, 9, 9),
        geometry::map_simple_image(9, 9, 5, 4, 256),
    );

    let doors = ctx.door_points(ri);
    let entrance = doors.first().copied().unwrap_or(Point::new(room.left, c.y));
    ctx.set_regular_doors(ri);

    // Java `new EmptyRoom()` twice: StandardRoom sizeCat still rolls chances({1,0,0}).
    let _ = Random::chances(&[1.0, 0.0, 0.0]);
    let _ = Random::chances(&[1.0, 0.0, 0.0]);

    let (entry_left, entry_top, entry_right, entry_bottom, entry_door, locked_door, treasure) =
        if entrance.x == room.left {
            (
                room.left + 1,
                room.top + 5,
                room.left + 3,
                room.bottom - 5,
                Point::new(room.left + 4, c.y),
                Point::new(room.right - 4, c.y),
                (
                    room.right - 3,
                    room.top + 3,
                    room.right - 1,
                    room.bottom - 3,
                ),
            )
        } else if entrance.x == room.right {
            (
                room.right - 3,
                room.top + 5,
                room.right - 1,
                room.bottom - 5,
                Point::new(room.right - 4, c.y),
                Point::new(room.left + 4, c.y),
                (room.left + 1, room.top + 3, room.left + 3, room.bottom - 3),
            )
        } else if entrance.y == room.top {
            (
                room.left + 5,
                room.top + 1,
                room.right - 5,
                room.top + 3,
                Point::new(c.x, room.top + 4),
                Point::new(c.x, room.bottom - 4),
                (
                    room.left + 3,
                    room.bottom - 3,
                    room.right - 3,
                    room.bottom - 1,
                ),
            )
        } else {
            (
                room.left + 5,
                room.bottom - 3,
                room.right - 5,
                room.bottom - 1,
                Point::new(c.x, room.bottom - 4),
                Point::new(c.x, room.top + 4),
                (room.left + 3, room.top + 1, room.right - 3, room.top + 3),
            )
        };

    geometry::set(ctx.map, entry_door.x, entry_door.y, DOOR);
    geometry::set(ctx.map, locked_door.x, locked_door.y, LOCKED_DOOR);
    geometry::fill_rect(
        ctx.map,
        entry_left,
        entry_top,
        entry_right,
        entry_bottom,
        CUSTOM_DECO_EMPTY,
    );
    let entry_w = entry_right - entry_left + 1;
    let entry_h = entry_bottom - entry_top + 1;
    let mut carpet_overrides = Vec::new();
    let mut treasure_spots = Vec::new();
    if entry_w > entry_h {
        for x in [1, 3, 7, 9] {
            geometry::set(ctx.map, entry_left + x, entry_top + 1, REGION_DECO);
            carpet_overrides.push((x, 1, CITY_PEDESTAL));
        }
        for x in [1, 3, 5, 7, 9, 11, 13] {
            treasure_spots.push(treasure.0 + x + (treasure.1 + 1) * width);
        }
    } else {
        for y in [1, 3, 7, 9] {
            geometry::set(ctx.map, entry_left + 1, entry_top + y, REGION_DECO);
            carpet_overrides.push((1, y, CITY_PEDESTAL));
        }
        for y in [1, 3, 5, 7, 9, 11, 13] {
            treasure_spots.push(treasure.0 + 1 + (treasure.1 + y) * width);
        }
    }
    geometry::add_carpet(
        ctx.map,
        entry_left,
        entry_top,
        entry_w,
        entry_h,
        depth,
        &carpet_overrides,
    );

    geometry::fill_rect(
        ctx.map, treasure.0, treasure.1, treasure.2, treasure.3, EMPTY_SP,
    );
    for &cell in &treasure_spots {
        geometry::set_cell(ctx.map, cell as usize, PEDESTAL);
    }

    // Center pedestal keeps the ImpStatue heap off the public map.
    let statue = treasure_spots.remove(3) as usize;
    ctx.occupy_heap(statue);
    Random::shuffle_list(&mut treasure_spots);
    let rewards = ctx.dungeon.imp.reward_options.len();
    for _ in 0..rewards {
        if treasure_spots.is_empty() {
            break;
        }
        ctx.occupy_heap(treasure_spots.remove(0) as usize);
    }
    ctx.dungeon.imp.reward_options.clear();
    ctx.dungeon.imp.pending_options.clear();

    let (tx, ty, tw, th) = (
        treasure.0,
        treasure.1 - 1,
        treasure.2 - treasure.0 + 1,
        treasure.3 - treasure.1 + 2,
    );
    let depth_seed = crate::dungeon::seed_for_depth(ctx.dungeon.seed, depth, 1);
    let variance = super::super::map_facts::tile_variance(ctx.map.len(), depth_seed);
    ctx.map.record_custom_tile(
        "VaultTreasure",
        CITY_QUEST_TEX,
        (tx, ty, tw, th),
        geometry::vault_treasure_map(ctx.map, tx, ty, tw, th, &variance),
    );
}
