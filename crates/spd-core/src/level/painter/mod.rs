//! RegularPainter water / grass / trap / decorate (post room-paint).
//!
//! Matches SPD `RegularPainter.paint` tail:
//! `nTraps()` on main stream (via [`n_traps`]) → room paint (elsewhere) →
//! optional paintDoors RNG → `pushGenerator(Long)` → water/grass/traps/decorate → pop.

mod decorate;
mod params;

use crate::level::patch;
use crate::level::terrain::{self, TerrainMap, EMPTY, GRASS, HIGH_GRASS, SECRET_TRAP, TRAP, WATER};
use crate::level::Feeling;
use crate::random::Random;
use crate::rooms::room::Room;

pub use params::n_traps;

/// Approximate `paintDoors` main-stream RNG: one `Float` per undirected connection
/// when evaluating hidden-door chance (merge-room path skipped — may desync slightly).
pub fn paint_doors_rng(depth: i32, feeling: Feeling, rooms: &[Room]) {
    let mut hidden_door_chance = if depth > 1 {
        (depth as f32 / 20.0).min(1.0)
    } else {
        0.0
    };
    if feeling == Feeling::Secrets {
        hidden_door_chance = (0.5 + hidden_door_chance) / 2.0;
    }

    let mut seen: Vec<(usize, usize)> = Vec::new();
    for (ri, room) in rooms.iter().enumerate() {
        if room.is_empty() {
            continue;
        }
        for &ni in &room.connected {
            let a = ri.min(ni);
            let b = ri.max(ni);
            if seen.contains(&(a, b)) {
                continue;
            }
            seen.push((a, b));
            // Always REGULAR doors in our builder path → Float vs hidden chance.
            let _ = Random::float() < hidden_door_chance;
            // Graph connectivity checks do not consume RNG.
        }
    }
}

/// Water + grass + traps + region decorate under a separate generator
/// (`Random.pushGenerator(Random.Long())` … `pop`), matching RegularPainter.
pub fn paint_water_grass_traps(
    map: &mut TerrainMap,
    rooms: &[Room],
    depth: i32,
    feeling: Feeling,
    n_traps: i32,
) {
    let seed = Random::long();
    Random::push_generator_seeded(seed);

    let (water_fill, water_smooth) = params::water_params(depth, feeling);
    let (grass_fill, grass_smooth) = params::grass_params(depth, feeling);

    if water_fill > 0.0 {
        paint_water(map, rooms, water_fill, water_smooth);
    }
    if grass_fill > 0.0 {
        paint_grass(map, rooms, grass_fill, grass_smooth);
    }
    if n_traps > 0 {
        paint_traps(map, rooms, depth, feeling, n_traps);
    }

    decorate::decorate(map, rooms, depth);

    Random::pop_generator();
    map.recompute_passable();
}

fn paint_water(map: &mut TerrainMap, rooms: &[Room], fill: f32, smoothness: i32) {
    let lake = patch::generate(map.width, map.height, fill, smoothness, true);
    if rooms.iter().any(|r| !r.is_empty()) {
        for room in rooms.iter().filter(|r| !r.is_empty()) {
            for y in room.top..=room.bottom {
                for x in room.left..=room.right {
                    // Room.canPlaceWater defaults true for all points in room rect.
                    if let Some(i) = map.point_to_cell(x, y) {
                        if lake.get(i).copied().unwrap_or(false) && map.map[i] == EMPTY {
                            map.map[i] = WATER;
                        }
                    }
                }
            }
        }
    } else {
        for (i, cell) in map.map.iter_mut().enumerate() {
            if lake.get(i).copied().unwrap_or(false) && *cell == EMPTY {
                *cell = WATER;
            }
        }
    }
}

fn paint_grass(map: &mut TerrainMap, rooms: &[Room], fill: f32, smoothness: i32) {
    let grass = patch::generate(map.width, map.height, fill, smoothness, true);
    let mut grass_cells: Vec<usize> = Vec::new();

    if rooms.iter().any(|r| !r.is_empty()) {
        for room in rooms.iter().filter(|r| !r.is_empty()) {
            for y in room.top..=room.bottom {
                for x in room.left..=room.right {
                    if let Some(i) = map.point_to_cell(x, y) {
                        if grass.get(i).copied().unwrap_or(false) && map.map[i] == EMPTY {
                            grass_cells.push(i);
                        }
                    }
                }
            }
        }
    } else {
        for (i, cell) in map.map.iter().enumerate() {
            if grass.get(i).copied().unwrap_or(false) && *cell == EMPTY {
                grass_cells.push(i);
            }
        }
    }

    let w = map.width;
    // PathFinder.NEIGHBOURS8 relative to cell (requires interior safety — grass on
    // map edge still indexes; clamp by bounds check).
    let neigh8: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for &i in &grass_cells {
        // No heaps/mobs in our paint path — always height roll.
        let mut count = 1i32;
        let x = (i as i32) % w;
        let y = (i as i32) / w;
        for (dx, dy) in neigh8 {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= w || ny >= map.height {
                continue;
            }
            let ni = (nx + ny * w) as usize;
            if grass.get(ni).copied().unwrap_or(false) {
                count += 1;
            }
        }
        map.map[i] = if Random::float() < count as f32 / 12.0 {
            HIGH_GRASS
        } else {
            GRASS
        };
    }
}

fn paint_traps(
    map: &mut TerrainMap,
    rooms: &[Room],
    depth: i32,
    feeling: Feeling,
    mut n_traps: i32,
) {
    let mut valid: Vec<usize> = Vec::new();
    if rooms.iter().any(|r| !r.is_empty()) {
        for room in rooms.iter().filter(|r| !r.is_empty()) {
            for y in room.top..=room.bottom {
                for x in room.left..=room.right {
                    if let Some(i) = map.point_to_cell(x, y) {
                        if map.map[i] == EMPTY {
                            valid.push(i);
                        }
                    }
                }
            }
        }
    } else {
        for (i, &t) in map.map.iter().enumerate() {
            if t == EMPTY {
                valid.push(i);
            }
        }
    }

    n_traps = n_traps.min(valid.len() as i32 / 5);

    // Passable snapshot for hallway test (matches paintTraps temp use of passable[]).
    let passable: Vec<bool> = map
        .map
        .iter()
        .map(|&t| terrain::is_passable_tile(t))
        .collect();
    let width = map.width as usize;
    // CIRCLE4: N, E, S, W
    let circle4: [i32; 4] = [-(map.width), 1, map.width, -1];

    let mut non_hall: Vec<usize> = Vec::new();
    for &i in &valid {
        let n = passable_at(&passable, width, i, circle4[0]);
        let e = passable_at(&passable, width, i, circle4[1]);
        let s = passable_at(&passable, width, i, circle4[2]);
        let wdir = passable_at(&passable, width, i, circle4[3]);
        if (n || s) && (e || wdir) {
            non_hall.push(i);
        }
    }

    n_traps = n_traps.min(valid.len() as i32 / 5);

    let (classes, chances) = params::trap_table(depth);
    // TrapMechanism absent → reveal chance 0
    let revealed_chance = 0.0f32;
    let mut reveal_inc = 0.0f32;

    let total = if feeling == Feeling::Traps {
        5 * n_traps
    } else {
        n_traps
    };

    for i in 0..total {
        if valid.is_empty() {
            break;
        }
        let ti = Random::chances(chances);
        if ti < 0 {
            break;
        }
        let trap = &classes[ti as usize];

        let pos = if trap.avoids_hallways && !non_hall.is_empty() {
            let idx = Random::int_max(non_hall.len() as i32) as usize;
            non_hall[idx]
        } else {
            let idx = Random::int_max(valid.len() as i32) as usize;
            valid[idx]
        };
        // remove Integer object (value), not index — Java ArrayList.remove(Object)
        valid.retain(|&c| c != pos);
        non_hall.retain(|&c| c != pos);

        reveal_inc += revealed_chance;
        let visible = i >= n_traps || reveal_inc >= 1.0;
        if reveal_inc >= 1.0 {
            reveal_inc -= 1.0;
        }

        map.map[pos] = if visible { TRAP } else { SECRET_TRAP };
        if pos < map.trap_destroys_items.len() {
            map.trap_destroys_items[pos] = trap.destroys_items;
        }
        if pos < map.trap_names.len() {
            map.trap_names[pos] = Some(trap.name);
        }
    }
}

fn passable_at(passable: &[bool], width: usize, cell: usize, delta: i32) -> bool {
    let n = cell as i32 + delta;
    if n < 0 || n as usize >= passable.len() {
        return false;
    }
    // Reject row wrap on E/W steps
    if delta == 1 && cell % width + 1 >= width {
        return false;
    }
    if delta == -1 && cell.is_multiple_of(width) {
        return false;
    }
    passable[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::terrain::{self, GRASS, HIGH_GRASS, SECRET_TRAP, TRAP, WATER};
    use crate::rooms::room::Room;
    use crate::rooms::types::RoomKind;

    fn box_room(id: usize, l: i32, t: i32, r: i32, b: i32) -> Room {
        let mut room = Room::new(id, "EmptyRoom", RoomKind::Standard, 1, 4, 5, 9, 5, 9);
        room.left = l;
        room.top = t;
        room.right = r;
        room.bottom = b;
        room
    }

    #[test]
    fn paint_adds_water_grass_or_traps() {
        Random::push_generator_seeded(42);
        let mut a = box_room(0, 1, 1, 10, 10);
        let mut b = box_room(1, 10, 1, 18, 10);
        a.connected.push(1);
        b.connected.push(0);
        let rooms = vec![a, b];
        let mut map = terrain::paint_minimal(&rooms).expect("map");
        let n = n_traps(3);
        paint_water_grass_traps(&mut map, &rooms, 3, Feeling::None, n);
        Random::pop_generator();

        let water = map.map.iter().filter(|&&t| t == WATER).count();
        let grass = map
            .map
            .iter()
            .filter(|&&t| t == GRASS || t == HIGH_GRASS)
            .count();
        let traps = map
            .map
            .iter()
            .filter(|&&t| t == TRAP || t == SECRET_TRAP)
            .count();
        // With default sewers fill (~30% water, 20% grass) on two rooms we expect some tiles.
        assert!(
            water + grass + traps > 0,
            "expected painter to place water/grass/traps, got w={water} g={grass} t={traps}"
        );
    }
}
