//! T3 vault treasure rooms: many scans, multi-ghoul, hard lasers.

use crate::items::model::ItemCategory;
use crate::level::terrain::{EMPTY_SP, PEDESTAL, WALL};
use crate::random::Random;

use super::super::geometry;
use super::super::hazards::place_laser;
use super::super::mobs::VaultMob;
use super::super::paint::{PaintCtx, CHEST, HEAP};

pub(in crate::level::vault) fn paint_many_scans(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    geometry::fill_margin(ctx.map, &room, 1, EMPTY_SP);
    let c = geometry::center(&room);
    let w = ctx.width();
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    let door_cell = ctx.point_cell(entrance);
    let corners = [
        ctx.cell(room.left + 1, room.top + 1),
        ctx.cell(c.x, room.top + 1),
        ctx.cell(room.right - 1, room.top + 1),
        ctx.cell(room.left + 1, c.y),
        ctx.cell(room.right - 1, c.y),
        ctx.cell(room.left + 1, room.bottom - 1),
        ctx.cell(c.x, room.bottom - 1),
        ctx.cell(room.right - 1, room.bottom - 1),
    ];
    for &cell in &corners {
        if geometry::true_distance(ctx.map, cell, door_cell) >= 2.0 {
            ctx.place_npc(cell, "VaultSentry");
        }
    }
    geometry::set(ctx.map, c.x, c.y, PEDESTAL);
    let center = ctx.cell(c.x, c.y);
    let treasure = ctx.create_equipment(3);
    ctx.drop(center, CHEST, treasure);
    let n8 = geometry::neighbours8(w);
    let consumable = ctx.create_consumable(3);
    ctx.drop(
        (center as i32 + n8[Random::int_max(8) as usize]) as usize,
        HEAP,
        consumable,
    );
    ctx.drop_new(
        (center as i32 + n8[Random::int_max(8) as usize]) as usize,
        "DwarfToken",
    );
    ctx.add_item_to_spawn("PotionOfInvisibility", ItemCategory::Potion);
}

pub(in crate::level::vault) fn paint_multiple_enemy(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    let c = geometry::center(&room);
    let w = ctx.width();
    let treasure = if entrance.x == room.left || entrance.x == room.right {
        let area_top = geometry::gate(room.top + 1, entrance.y - 2, room.bottom - 5);
        geometry::fill_wh(ctx.map, room.left + 1, area_top + 1, 9, 3, EMPTY_SP);
        geometry::fill_wh(ctx.map, room.left + 4, area_top, 3, 5, EMPTY_SP);
        place_ghoul(ctx, c.x, area_top);
        place_ghoul(ctx, c.x, area_top + 4);
        if entrance.x == room.left {
            place_ghoul(ctx, c.x + 1, area_top + 2) + 2
        } else {
            place_ghoul(ctx, c.x - 1, area_top + 2) - 2
        }
    } else {
        let area_left = geometry::gate(room.left + 1, entrance.x - 2, room.right - 5);
        geometry::fill_wh(ctx.map, area_left + 1, room.top + 1, 3, 9, EMPTY_SP);
        geometry::fill_wh(ctx.map, area_left, room.top + 4, 5, 3, EMPTY_SP);
        place_ghoul(ctx, area_left, c.y);
        place_ghoul(ctx, area_left + 4, c.y);
        if entrance.y == room.top {
            (place_ghoul(ctx, area_left + 2, c.y + 1) as i32 + 2 * w) as usize
        } else {
            (place_ghoul(ctx, area_left + 2, c.y - 1) as i32 - 2 * w) as usize
        }
    };
    geometry::set_cell(ctx.map, treasure, PEDESTAL);
    let item = ctx.create_equipment(3);
    ctx.drop(treasure, CHEST, item);
    let n8 = geometry::neighbours8(w);
    let offset = loop {
        let i = n8[Random::int_max(8) as usize];
        let cell = treasure as i32 + i;
        if cell >= 0 && (cell as usize) < ctx.map.len() && ctx.map.map[cell as usize] != WALL {
            break cell as usize;
        }
    };
    let consumable = ctx.create_consumable(3);
    ctx.drop(offset, HEAP, consumable);
}

/// `new VaultGhoul()` placed directly, so it keeps the default `SLEEPING` state.
fn place_ghoul(ctx: &mut PaintCtx<'_>, x: i32, y: i32) -> usize {
    let cell = ctx.cell(x, y);
    ctx.place_enemy(cell, VaultMob::Ghoul, false);
    cell
}

pub(in crate::level::vault) fn paint_hard_laser(ctx: &mut PaintCtx<'_>, ri: usize) {
    let room = ctx.room(ri).clone();
    geometry::fill_room(ctx.map, &room, WALL);
    let entrance = ctx.entrance_door(ri);
    ctx.set_regular_doors(ri);
    let item_place = if entrance.x == room.left || entrance.x == room.right {
        let area_top = geometry::gate(room.top + 1, entrance.y - 2, room.bottom - 5);
        geometry::fill_wh(ctx.map, room.left + 1, area_top, 9, 5, EMPTY_SP);
        if entrance.x == room.left {
            for x in ((room.left + 2)..(room.right - 1)).rev() {
                place_laser(ctx, x, area_top);
                place_laser(ctx, x, area_top + 4);
            }
            (room.right - 1, area_top, room.right - 1, area_top + 4)
        } else {
            for x in (room.left + 2)..(room.right - 1) {
                place_laser(ctx, x, area_top);
                place_laser(ctx, x, area_top + 4);
            }
            (room.left + 1, area_top, room.left + 1, area_top + 4)
        }
    } else {
        let area_left = geometry::gate(room.left + 1, entrance.x - 2, room.right - 5);
        geometry::fill_wh(ctx.map, area_left, room.top + 1, 5, 9, EMPTY_SP);
        if entrance.y == room.top {
            for y in ((room.top + 2)..(room.bottom - 1)).rev() {
                place_laser(ctx, area_left, y);
                place_laser(ctx, area_left + 4, y);
            }
            (area_left, room.bottom - 1, area_left + 4, room.bottom - 1)
        } else {
            for y in (room.top + 2)..(room.bottom - 1) {
                place_laser(ctx, area_left, y);
                place_laser(ctx, area_left + 4, y);
            }
            (area_left, room.top + 1, area_left + 4, room.top + 1)
        }
    };
    geometry::fill_wh(
        ctx.map,
        item_place.0,
        item_place.1,
        item_place.2 - item_place.0 + 1,
        item_place.3 - item_place.1 + 1,
        EMPTY_SP,
    );
    // Java draws the chest cell before `createEquipment(3)`.
    let first = ctx.random_rect_cell(item_place);
    let treasure = ctx.create_equipment(3);
    ctx.drop(first, CHEST, treasure);
    let consumable = ctx.create_consumable(3);
    let second = ctx.random_free_rect_cell(item_place);
    ctx.drop(second, HEAP, consumable);
    let third = ctx.random_free_rect_cell(item_place);
    ctx.drop_new(third, "DwarfToken");
    ctx.add_item_to_spawn("StoneOfBlink", ItemCategory::Stone);
}
