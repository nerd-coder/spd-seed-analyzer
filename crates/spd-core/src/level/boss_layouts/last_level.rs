//! Pinned SPD v4.0.2 `LastLevel` structural generation.

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::{map_facts, terrain};
use crate::random::Random;
use crate::report::{FloorMap, MapCustomTile, MapHeap};

const WIDTH: usize = 16;
const HEIGHT: usize = 64;
/// `LastLevel.AMULET_POS`: `12*WIDTH + MID`.
const AMULET_POS: u32 = 12 * WIDTH as u32 + 8;

pub(super) fn build(
    dungeon: &mut DungeonState,
    mut tiles: Vec<u16>,
    discoverable: Vec<bool>,
    depth_seed: i64,
) -> FloorMap {
    // `LastLevel` is not a boss level in Java, so `Level.create` advances the
    // food deck and rolls a normal feeling before `LastLevel.build` resets it.
    let _ = dungeon
        .generator
        .random_category(Category::Food, dungeon.depth);
    if dungeon.pos_needed() {
        dungeon.limited.strength_potions += 1;
    }
    let _ = Random::int_max(14);
    let _ = Random::float();
    let _ = Random::float();
    // LastLevel decorates its initial shaft before repainting the amulet room.
    // The repaint is visible in the final terrain but its prior RNG calls still
    // advance the level generator.
    for (cell, tile) in tiles.iter_mut().enumerate() {
        if initial_empty(cell) && Random::int_max(5) == 0 && !amulet_room(cell) {
            *tile = terrain::EMPTY_DECO as u16;
        }
    }

    let tile_variance = map_facts::tile_variance(WIDTH * HEIGHT, depth_seed);
    let custom_tiles = custom_tiles(&tiles, &tile_variance);
    FloorMap {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        tileset: terrain::tileset_for_depth(26).to_string(),
        tile_variance,
        tiles,
        // v4 `LastLevel.cleanWalls` starts at `(height-ROOM_TOP+2)*width`, so the
        // entrance wall row stays on ordinary `Level.cleanWalls` discoverability.
        discoverable,
        markers: Vec::new(),
        heaps: vec![MapHeap {
            cell: AMULET_POS,
            heap_type: "heap".into(),
            items: vec![map_facts::heap_item(&GeneratedItem::new(
                "Amulet",
                ItemCategory::Other,
            ))],
        }],
        mobs: Vec::new(),
        transitions: super::transitions(26),
        traps: Vec::new(),
        plants: Vec::new(),
        blobs: Vec::new(),
        custom_tiles,
        custom_terrain: Vec::new(),
        custom_walls: custom_walls(),
        uncertain_cells: Vec::new(),
        hidden_cells: Vec::new(),
        concealed_cells: Vec::new(),
        per_run_cells: Vec::new(),
        sentry_scans: Vec::new(),
        laser_beams: Vec::new(),
        history_cells: Vec::new(),
        constrained_equipment_cells: Vec::new(),
    }
}

fn initial_empty(cell: usize) -> bool {
    let (x, y) = (cell % WIDTH, cell / WIDTH);
    ((x as isize) - 8).unsigned_abs() <= 1 && (10..=53).contains(&y)
        || (56..=63).contains(&y) && !(y == 56 && (7..=9).contains(&x))
}

fn amulet_room(cell: usize) -> bool {
    let (x, y) = (cell % WIDTH, cell / WIDTH);
    (y == 9 && (6..=10).contains(&x))
        || ((10..=14).contains(&y) && (5..=11).contains(&x))
        || (y == 15 && (6..=10).contains(&x))
}

/// `CustomFloor.CANDLES`, painted from `AMULET_POS - 3 - 3*width`.
const CANDLES: [i16; 49] = [
    -1, 42, 46, 46, 46, 43, -1, 42, 46, 46, 46, 46, 46, 43, 46, 46, 45, 19, 44, 46, 46, 46, 46, 19,
    19, 19, 46, 46, 46, 46, 43, 19, 42, 46, 46, 44, 46, 46, 19, 46, 46, 45, -1, 44, 45, 19, 44, 45,
    -1,
];
/// `CustomFloor.setRect(5, 0, 7, height - ROOM_TOP)`.
const FLOOR_LEFT: usize = 5;
const FLOOR_WIDTH: usize = 7;
const FLOOR_HEIGHT: usize = 54;

/// `LastLevel.CustomFloor.create` before the Amulet is obtained.
fn custom_floor(map: &[u16], tile_variance: &[u8]) -> Vec<i16> {
    let candles_start = AMULET_POS as usize - 3 - 3 * WIDTH;
    let chasm = terrain::CHASM as u16;
    let mut data = vec![0i16; FLOOR_WIDTH * FLOOR_HEIGHT];
    let row_start = |i: usize| FLOOR_LEFT + (i / FLOOR_WIDTH) * WIDTH;
    let mut cell = FLOOR_LEFT;
    let mut i = 0;
    while i < data.len() {
        if i % FLOOR_WIDTH == 0 {
            cell = row_start(i);
        }
        if cell == candles_start {
            for candle in CANDLES {
                if data[i] == 0 {
                    data[i] = candle;
                }
                if data[i] == 46 && tile_variance[cell] >= 50 {
                    data[i] += 1;
                }
                if map[cell] != chasm && map[cell + WIDTH] == chasm {
                    data[i + FLOOR_WIDTH] = 6;
                }
                i += 1;
                cell += 1;
                if i % FLOOR_WIDTH == 0 {
                    cell = row_start(i);
                }
            }
        }
        if map[cell] == terrain::EMPTY_DECO as u16 {
            data[i] = 27;
        } else if map[cell] == terrain::EMPTY as u16 {
            data[i] = 19;
        } else if data[i] == 0 {
            data[i] = -1;
        }
        cell += 1;
        i += 1;
    }
    data
}

fn custom_tiles(map: &[u16], tile_variance: &[u8]) -> Vec<MapCustomTile> {
    vec![
        MapCustomTile {
            class_name: "CenterPieceVisuals".into(),
            texture: "halls_special".into(),
            x: 0,
            y: 54,
            width: 16,
            height: 10,
            static_data: vec![
                -1, -1, -1, -1, -1, -1, -1, -1, 19, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 8, 9,
                10, 11, 19, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 31, 19, 31, 20, 21,
                22, 0, 0, 0, 0, 0, 0, 0, 24, 25, 26, 19, 19, 19, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0,
                24, 25, 26, 19, 19, 19, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 24, 25, 26, 19, 19, 19,
                28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 24, 25, 34, 35, 35, 35, 34, 29, 30, 0, 0, 0, 0, 0,
                0, 0, 40, 41, 36, 36, 36, 36, 36, 40, 41, 0, 0, 0, 0, 0, 0, 0, 48, 49, 36, 36, 36,
                36, 36, 48, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        },
        MapCustomTile {
            class_name: "CustomFloor".into(),
            texture: "halls_special".into(),
            x: FLOOR_LEFT as u32,
            y: 0,
            width: FLOOR_WIDTH as u32,
            height: FLOOR_HEIGHT as u32,
            static_data: custom_floor(map, tile_variance),
        },
    ]
}

fn custom_walls() -> Vec<MapCustomTile> {
    vec![MapCustomTile {
        class_name: "CenterPieceWalls".into(),
        texture: "halls_special".into(),
        x: 0,
        y: 53,
        width: 16,
        height: 9,
        static_data: {
            let mut data = vec![
                4, 4, 4, 4, 4, 4, 4, 5, 7, 3, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 1, 15, 2, 0,
                0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, 23, -1, -1, -1, -1, -1, -1, -1,
            ];
            data.extend(std::iter::repeat_n(-1, 64));
            data.extend([
                -1, -1, -1, -1, 32, 33, -1, -1, -1, -1, -1, 32, 33, -1, -1, -1, -1, -1, -1, -1, 40,
                41, -1, -1, -1, -1, -1, 40, 41, -1, -1, -1,
            ]);
            data
        },
    }]
}

#[cfg(test)]
mod tests {
    #[test]
    fn last_level_entrance_wall_row_stays_discoverable() {
        let floor = super::super::fixed_layout(26).expect("last level");
        let width = super::WIDTH;
        for x in 0..width {
            assert!(
                floor.discoverable[55 * width + x],
                "v4 cleanWalls leaves row 55 discoverable at x={x}"
            );
        }
        for x in 0..4 {
            assert!(
                !floor.discoverable[56 * width + x],
                "lower chamber edge x={x}"
            );
        }
    }
}
