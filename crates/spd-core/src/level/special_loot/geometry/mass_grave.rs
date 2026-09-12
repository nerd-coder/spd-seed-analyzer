//! Pinned `MassGraveRoom.paint` terrain, deco, skeletons, and loot.

use crate::dungeon::DungeonState;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, CUSTOM_DECO_EMPTY, STATUE, WALL, WALL_DECO};
use crate::random::Random;
use crate::rooms::room::Room;

const PRISON_QUEST: &str = "prison_quest";
#[rustfmt::skip]
const DECO_RENDER: [u8; 81] = [
    0, 0, 0, 1, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 1, 1, 1, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 1, 1, 1,
    1, 1, 0, 0, 0, 0, 0, 1, 1,
];

pub fn paint(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));

    fill_room(map, room, WALL);
    fill_margin(map, room, 1, CUSTOM_DECO_EMPTY);
    fill(map, room.left + 1, room.top + 1, 3, 1, WALL);
    fill(map, room.left + 1, room.top + 2, 2, 1, WALL);
    fill(map, room.right - 3, room.top + 1, 3, 1, WALL);
    fill(map, room.right - 2, room.top + 2, 2, 1, WALL);
    set(map, room.left + 5, room.top, WALL_DECO);
    set(map, room.left + 3, room.top + 2, STATUE);
    set(map, room.right - 3, room.top + 2, STATUE);

    let tile_w = room.width() - 2;
    let tile_h = room.height() - 1;
    map.record_custom_tile(
        "MassGraveDeco",
        PRISON_QUEST,
        (room.left + 1, room.top, tile_w, tile_h),
        deco_data(tile_w, tile_h),
    );
    map.record_custom_terrain(
        "StatueRaised",
        PRISON_QUEST,
        (room.left + 3, room.top + 2, 1, 1),
        vec![4],
    );
    map.record_custom_terrain(
        "StatueRaised",
        PRISON_QUEST,
        (room.right - 3, room.top + 2, 1, 1),
        vec![4],
    );

    let mut i = 0;
    while i <= Random::int_max(2) {
        let cell = clamped_cell(map, room, 3, |cell| map.mob_occupied[cell]);
        map.mob_occupied[cell] = true;
        map.known_mobs[cell] = Some("Skeleton");
        i += 1;
    }

    let mut items: Vec<GeneratedItem> = Vec::new();
    items.push(GeneratedItem::new("CorpseDust", ItemCategory::Other));
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.6 {
        items.push(dungeon.generator.random(dungeon.depth));
    }
    if Random::float() <= 0.3 {
        items.push(
            dungeon
                .generator
                .random_armor(dungeon.depth / 5, dungeon.depth),
        );
    }

    let mut out = Vec::new();
    for mut item in items {
        let cell = clamped_cell(map, room, 5, |cell| map.heap_occupied[cell]);
        item.source = Some("MassGraveRoom".into());
        map.record_heap(cell, "skeleton", item.clone());
        out.push(PlacedLoot {
            item,
            heap_type: "skeleton",
        });
    }
    out
}

fn clamped_cell(
    map: &TerrainMap,
    room: &Room,
    max_y: i32,
    occupied: impl Fn(usize) -> bool,
) -> usize {
    loop {
        let mut p = room.random_margin(1);
        if p.y <= room.top + 2 {
            p.x = Random::int_range_inclusive(room.left + 4, room.right - 4);
        }
        let cell = map
            .point_to_cell(p.x, p.y)
            .expect("MassGraveRoom point is on map");
        if p.y <= room.top + max_y && !occupied(cell) {
            return cell;
        }
    }
}

fn deco_data(tile_w: i32, tile_h: i32) -> Vec<i16> {
    let mut data = map_simple_image(tile_w, tile_h, 5, 0, 256);
    for (i, slot) in data.iter_mut().enumerate() {
        if DECO_RENDER.get(i).copied() == Some(0) {
            *slot = -1;
        }
    }
    data
}

fn map_simple_image(tile_w: i32, tile_h: i32, tx: i32, ty: i32, tex_w: i32) -> Vec<i16> {
    let tex_tile_width = tex_w / 16;
    let mut data = Vec::with_capacity((tile_w * tile_h) as usize);
    let mut x = tx;
    let mut y = ty;
    for _ in 0..tile_w * tile_h {
        data.push((x + tex_tile_width * y) as i16);
        x += 1;
        if x - tx == tile_w {
            x = tx;
            y += 1;
        }
    }
    data
}

fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

fn fill(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, terrain: i32) {
    for dy in 0..h {
        for dx in 0..w {
            set(map, x + dx, y + dy, terrain);
        }
    }
}

fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    fill(
        map,
        room.left,
        room.top,
        room.width(),
        room.height(),
        terrain,
    );
}

fn fill_margin(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill(
        map,
        room.left + margin,
        room.top + margin,
        room.width() - margin * 2,
        room.height() - margin * 2,
        terrain,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deco_mask_clears_unrendered_simple_image_cells() {
        let data = deco_data(9, 9);
        assert_eq!(data.len(), 81);
        assert_eq!(data[0], -1);
        assert_eq!(data[3], 8);
        assert_eq!(data[11], 23);
        assert_eq!(data[80], 141);
        assert_eq!(data[74], -1);
    }
}
