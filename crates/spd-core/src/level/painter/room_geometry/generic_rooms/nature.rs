//! Plants and aquarium standard rooms.

use crate::generator::{Category, GeneratorState};
use crate::geom::Point;
use crate::level::terrain::{TerrainMap, GRASS, HIGH_GRASS, WALL, WATER};
use crate::random::Random;
use crate::rooms::room::Room;

use super::{center, fill_margin, fill_room, set};

pub(super) fn paint_plants(
    map: &mut TerrainMap,
    room: &Room,
    generator: &mut GeneratorState,
    depth: i32,
) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, GRASS);
    fill_margin(map, room, 2, HIGH_GRASS);
    if room.width().min(room.height()) >= 7 {
        fill_margin(map, room, 3, GRASS);
    }

    let center = center(room);
    let plants = if room.width().max(room.height()) >= 9 {
        if room.width().min(room.height()) >= 11 {
            draw_horizontal(map, room.left + 2, room.right - 2, center.y, HIGH_GRASS);
            draw_vertical(map, center.x, room.top + 2, room.bottom - 2, HIGH_GRASS);
            vec![
                Point::new(center.x - 1, center.y - 1),
                Point::new(center.x + 1, center.y - 1),
                Point::new(center.x - 1, center.y + 1),
                Point::new(center.x + 1, center.y + 1),
            ]
        } else if room.width() > room.height()
            || (room.width() == room.height() && Random::int_max(2) == 0)
        {
            draw_vertical(map, center.x, room.top + 2, room.bottom - 2, HIGH_GRASS);
            vec![
                Point::new(center.x - 1, center.y),
                Point::new(center.x + 1, center.y),
            ]
        } else {
            draw_horizontal(map, room.left + 2, room.right - 2, center.y, HIGH_GRASS);
            vec![
                Point::new(center.x, center.y - 1),
                Point::new(center.x, center.y + 1),
            ]
        }
    } else {
        vec![center]
    };

    for plant in plants {
        random_non_firebloom_seed(generator, depth);
        if let Some(cell) = map.point_to_cell(plant.x, plant.y) {
            // Level.plant converts HIGH_GRASS/EMPTY/EMBERS to GRASS.
            map.map[cell] = GRASS;
            map.item_allowed[cell] = false;
        }
    }
}

fn random_non_firebloom_seed(generator: &mut GeneratorState, depth: i32) {
    loop {
        let seed = generator.random_using_defaults(Category::Seed, depth);
        if seed.class_name != "FirebloomSeed" {
            return;
        }
    }
}

pub(super) fn paint_aquarium(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, crate::level::terrain::EMPTY);
    fill_margin(map, room, 2, crate::level::terrain::EMPTY_SP);
    fill_margin(map, room, 3, WATER);

    for y in (room.top + 1)..room.bottom {
        for x in (room.left + 1)..room.right {
            if let Some(cell) = map.point_to_cell(x, y) {
                if map.map[cell] == WATER {
                    map.item_allowed[cell] = false;
                }
            }
        }
    }

    let num_fish = (room.width().min(room.height()) - 4) / 3;
    let mut fish_cells = Vec::with_capacity(num_fish.max(0) as usize);
    for _ in 0..num_fish {
        // Piranha.random always rolls for the phantom-piranha variant.
        let _ = Random::float();
        loop {
            let x = Random::int_range_inclusive(room.left + 3, room.right - 3);
            let y = Random::int_range_inclusive(room.top + 3, room.bottom - 3);
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            if map.map[cell] == WATER && !fish_cells.contains(&cell) {
                fish_cells.push(cell);
                break;
            }
        }
    }
}

pub(super) fn paint_grassy_grave(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, GRASS);
}

fn draw_horizontal(map: &mut TerrainMap, left: i32, right: i32, y: i32, terrain: i32) {
    for x in left..=right {
        set(map, x, y, terrain);
    }
}

fn draw_vertical(map: &mut TerrainMap, x: i32, top: i32, bottom: i32, terrain: i32) {
    for y in top..=bottom {
        set(map, x, y, terrain);
    }
}
