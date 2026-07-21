//! Minefield room geometry and explosive traps.

use crate::level::terrain::{TerrainMap, EMBERS, EMPTY, SECRET_TRAP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

use super::{fill_margin, fill_room};

const NEIGHBOURS_8: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub(super) fn paint_minefield(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    let mut mines = (room.square() as f64).sqrt().round() as i32;
    mines += match room.size_factor {
        1 => -3,
        2 => 3,
        3 => 9,
        _ => 0,
    };

    let mut trap_cells = Vec::with_capacity(mines.max(0) as usize);
    for _ in 0..mines {
        let (x, y, pos) = loop {
            let x = Random::int_range_inclusive(room.left + 1, room.right - 1);
            let y = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
            let Some(pos) = map.point_to_cell(x, y) else {
                continue;
            };
            if !trap_cells.contains(&pos) {
                break (x, y, pos);
            }
        };

        for _ in 0..8 {
            let (dx, dy) = NEIGHBOURS_8[Random::int_max(8) as usize];
            let Some(neighbour) = map.point_to_cell(x + dx, y + dy) else {
                continue;
            };
            if !trap_cells.contains(&neighbour) && map.map[neighbour] == EMPTY {
                map.map[neighbour] = EMBERS;
            }
        }

        // TrapMechanism is absent in analyzer runs, so reveal chance is zero.
        map.map[pos] = SECRET_TRAP;
        map.trap_destroys_items[pos] = true;
        map.trap_names[pos] = Some("ExplosiveTrap");
        trap_cells.push(pos);
    }
}
