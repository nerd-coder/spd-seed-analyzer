//! Pinned SPD v3.3.8 `LastLevel` structural generation.

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::level::{map_facts, terrain};
use crate::random::Random;
use crate::report::{FloorMap, MapCustomTile};

const WIDTH: usize = 16;
const HEIGHT: usize = 64;

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

    FloorMap {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        tileset: terrain::tileset_for_depth(26).to_string(),
        tile_variance: map_facts::tile_variance(WIDTH * HEIGHT, depth_seed),
        tiles,
        discoverable,
        markers: Vec::new(),
        heaps: Vec::new(),
        mobs: Vec::new(),
        transitions: super::transitions(26),
        traps: Vec::new(),
        plants: Vec::new(),
        blobs: Vec::new(),
        custom_tiles: custom_tiles(),
        custom_walls: custom_walls(),
        runtime_sensitive_loot_cells: Vec::new(),
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

fn custom_tiles() -> Vec<MapCustomTile> {
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
            x: 5,
            y: 0,
            width: 7,
            height: 54,
            static_data: vec![
                -1, 42, 46, 46, 46, 43, -1, 42, 46, 46, 46, 46, 46, 43, 46, 46, 45, 19, 44, 46, 46,
                46, 46, 19, 19, 19, 46, 46, 46, 46, 43, 19, 42, 46, 46, 44, 46, 46, 19, 46, 46, 45,
                -1, 44, 45, 19, 44, 45, -1,
            ],
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
