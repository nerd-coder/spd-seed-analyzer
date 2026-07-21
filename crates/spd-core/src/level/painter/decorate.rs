//! Region-specific `decorate()` (visual terrain variance; consumes sub-generator RNG).

mod caves;

use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_DECO, EMPTY_SP, WALL, WALL_DECO, WATER};
use crate::random::Random;
use crate::rooms::room::Room;

use super::DoorMap;

pub fn decorate(
    map: &mut TerrainMap,
    rooms: &[Room],
    paint_order: &[usize],
    doors: &DoorMap,
    depth: i32,
) {
    match depth {
        1..=5 => decorate_sewers(map),
        6..=10 => decorate_prison(map, rooms),
        11..=15 => caves::decorate(map, rooms, paint_order, doors),
        16..=20 => decorate_city(map, depth),
        _ => decorate_halls(map),
    }
}

/// `SewerPainter.decorate`
fn decorate_sewers(map: &mut TerrainMap) {
    let w = map.width;
    let l = map.map.len() as i32;
    // top row wall over water
    for i in 0..w {
        let i = i as usize;
        let below = i + w as usize;
        if below < map.map.len()
            && map.map[i] == WALL
            && map.map[below] == WATER
            && Random::int_max(4) == 0
        {
            map.map[i] = WALL_DECO;
        }
    }
    for i in w..l - w {
        let i = i as usize;
        let above = i - w as usize;
        let below = i + w as usize;
        if map.map[i] == WALL
            && map.map[above] == WALL
            && map.map[below] == WATER
            && Random::int_max(2) == 0
        {
            map.map[i] = WALL_DECO;
        }
    }
    for i in (w + 1)..(l - w - 1) {
        let i = i as usize;
        if map.map[i] != EMPTY {
            continue;
        }
        let count = (map.map[i + 1] == WALL) as i32
            + (map.map[i - 1] == WALL) as i32
            + (map.map[i + w as usize] == WALL) as i32
            + (map.map[i - w as usize] == WALL) as i32;
        if Random::int_max(16) < count * count {
            map.map[i] = EMPTY_DECO;
        }
    }
}

/// `PrisonPainter.decorate` (chasm/region-deco room loops simplified — no CHASM rooms yet).
fn decorate_prison(map: &mut TerrainMap, _rooms: &[Room]) {
    let w = map.width;
    let l = map.map.len() as i32;
    for i in (w + 1)..(l - w - 1) {
        let i = i as usize;
        if map.map[i] != EMPTY {
            continue;
        }
        let mut c = 0.05f32;
        if map.map[i + 1] == WALL && map.map[i + w as usize] == WALL {
            c += 0.2;
        }
        if map.map[i - 1] == WALL && map.map[i + w as usize] == WALL {
            c += 0.2;
        }
        if map.map[i + 1] == WALL && map.map[i - w as usize] == WALL {
            c += 0.2;
        }
        if map.map[i - 1] == WALL && map.map[i - w as usize] == WALL {
            c += 0.2;
        }
        if Random::float() < c {
            map.map[i] = EMPTY_DECO;
        }
    }
    // Fissure/Chasm region deco skipped (no CHASM terrain from our painter yet).
    for i in 0..w {
        let i = i as usize;
        let below = i + w as usize;
        if below < map.map.len()
            && map.map[i] == WALL
            && (map.map[below] == EMPTY || map.map[below] == EMPTY_SP)
            && Random::int_max(6) == 0
        {
            map.map[i] = WALL_DECO;
        }
    }
    for i in w..l - w {
        let i = i as usize;
        if map.map[i] == WALL
            && map.map[i - w as usize] == WALL
            && (map.map[i + w as usize] == EMPTY || map.map[i + w as usize] == EMPTY_SP)
            && Random::int_max(3) == 0
        {
            map.map[i] = WALL_DECO;
        }
    }
}

/// `CityPainter.decorate`
fn decorate_city(map: &mut TerrainMap, depth: i32) {
    let w = map.width;
    let l = map.map.len() as i32;
    for i in 0..(l - w) {
        let i = i as usize;
        if map.map[i] == EMPTY && Random::int_max(10) == 0 {
            map.map[i] = EMPTY_DECO;
        } else if map.map[i] == WALL {
            // !wallStitcheable(map[i+w]) approx: below is not wall
            let below = map.map[i + w as usize];
            if below != WALL && below != WALL_DECO && Random::int_max(21 - depth) == 0 {
                map.map[i] = WALL_DECO;
            }
        }
    }
}

/// `HallsPainter.decorate` subset (neighbour merge skipped).
fn decorate_halls(map: &mut TerrainMap) {
    let w = map.width;
    let l = map.map.len() as i32;
    let neigh8: [i32; 8] = [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1];
    for i in (w + 1)..(l - w - 1) {
        let i = i as usize;
        if map.map[i] == EMPTY {
            let mut count = 0i32;
            for &d in &neigh8 {
                let n = (i as i32 + d) as usize;
                if n < map.map.len() && crate::level::terrain::is_passable_tile(map.map[n]) {
                    count += 1;
                }
            }
            if Random::int_max(80) < count {
                map.map[i] = EMPTY_DECO;
            }
        } else if map.map[i] == WALL
            && map.map[i - 1] != WALL_DECO
            && map.map[i - w as usize] != WALL_DECO
            && Random::int_max(20) == 0
        {
            map.map[i] = WALL_DECO;
        }
    }
}
