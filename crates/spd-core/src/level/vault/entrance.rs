//! `VaultEntranceRoom.paint`

use crate::geom::Point;
use crate::level::terrain::{CUSTOM_DECO_EMPTY, EMPTY, REGION_DECO, WALL};
use crate::random::Random;

use super::geometry::{self, CITY_QUEST_TEX};
use super::paint::PaintCtx;

const DARKNESS: i32 = 32;

pub(super) fn paint(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    let depth = ctx.dungeon.depth;
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 2, EMPTY);
    let c = geometry::center(&room);

    geometry::set(ctx.map, c.x - 3, c.y - 3, WALL);
    geometry::set(ctx.map, c.x + 3, c.y - 3, WALL);
    geometry::set(ctx.map, c.x - 3, c.y + 3, WALL);
    geometry::set(ctx.map, c.x + 3, c.y + 3, WALL);

    let doors = ctx.door_points(ri);
    ctx.set_regular_doors(ri);
    for door in &doors {
        geometry::draw_inside(ctx.map, &room, *door, 3, EMPTY);
    }

    geometry::set(ctx.map, c.x - 2, c.y - 2, REGION_DECO);
    geometry::set(ctx.map, c.x + 2, c.y - 2, REGION_DECO);
    geometry::set(ctx.map, c.x - 2, c.y + 2, REGION_DECO);
    geometry::set(ctx.map, c.x + 2, c.y + 2, REGION_DECO);

    geometry::fill_wh(
        ctx.map,
        room.left + 2,
        room.top + 4,
        7,
        3,
        CUSTOM_DECO_EMPTY,
    );
    geometry::fill_wh(
        ctx.map,
        room.left + 4,
        room.top + 2,
        3,
        7,
        CUSTOM_DECO_EMPTY,
    );
    geometry::add_carpet(ctx.map, room.left + 2, room.top + 4, 7, 3, depth, &[]);
    geometry::add_carpet(ctx.map, room.left + 4, room.top + 2, 3, 7, depth, &[]);

    ctx.map.record_custom_tile(
        "QuestEntranceInternal",
        CITY_QUEST_TEX,
        (c.x - 1, c.y - 1, 3, 3),
        geometry::map_simple_image(3, 3, 8, 1, 256),
    );
    ctx.map.record_custom_terrain(
        "WallBanners",
        CITY_QUEST_TEX,
        (room.left + 2, room.top + 1, 7, 3),
        Vec::new(),
    );

    let candidates = [
        Point::new(room.left + 2, c.y),
        Point::new(room.right - 2, c.y),
        Point::new(c.x, room.top + 2),
        Point::new(c.x, room.bottom - 2),
    ];
    let mut furthest = candidates[0];
    let mut furthest_dist = 0.0f32;
    for p in candidates {
        let dist: f32 = doors.iter().map(|d| geometry::point_distance(p, *d)).sum();
        if dist > furthest_dist {
            furthest = p;
            furthest_dist = dist;
        }
    }
    let ofs = if furthest.x == c.x { 1 } else { ctx.width() };
    let base = ctx.point_cell(furthest) as i32;
    let add_torch = ctx.dungeon.challenges & DARKNESS != 0;
    let torch_slot = Random::int_max(3);
    for (slot, cell) in [base - ofs, base, base + ofs].into_iter().enumerate() {
        let class_name = if slot as i32 == torch_slot {
            "Torch"
        } else {
            "VaultBeacon"
        };
        if class_name == "VaultBeacon" || add_torch {
            ctx.drop_new(cell as usize, class_name);
        }
    }

    let entrance = ctx.point_cell(c);
    ctx.map.branch_entrances.push(entrance);
}
