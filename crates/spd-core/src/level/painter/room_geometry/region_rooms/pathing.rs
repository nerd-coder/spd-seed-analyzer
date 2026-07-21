//! Room-local reachability helpers used by structural painters.

use std::collections::VecDeque;

use crate::geom::Point;
use crate::level::terrain::{TerrainMap, CHASM};
use crate::rooms::room::Room;

pub(super) fn room_cell_from_door(room: &Room, door: Point) -> usize {
    let (x, y) = if door.x == room.left {
        (door.x + 1, door.y)
    } else if door.x == room.right {
        (door.x - 1, door.y)
    } else if door.y == room.top {
        (door.x, door.y + 1)
    } else {
        (door.x, door.y - 1)
    };
    ((x - room.left - 1) + (y - room.top - 1) * (room.width() - 2)) as usize
}

pub(super) fn all_floor_reachable(map: &TerrainMap, room: &Room, start: usize) -> bool {
    let width = room.width() - 2;
    let height = room.height() - 2;
    let len = (width * height) as usize;
    if start >= len {
        return false;
    }
    let passable: Vec<bool> = (0..len)
        .map(|cell| {
            let x = room.left + 1 + cell as i32 % width;
            let y = room.top + 1 + cell as i32 / width;
            map.point_to_cell(x, y)
                .is_some_and(|map_cell| map.map[map_cell] != CHASM)
        })
        .collect();
    let mut reached = vec![false; len];
    let mut queue = VecDeque::from([start]);
    reached[start] = true;
    while let Some(cell) = queue.pop_front() {
        let x = cell as i32 % width;
        let y = cell as i32 / width;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= width || ny >= height {
                    continue;
                }
                let next = (nx + ny * width) as usize;
                if passable[next] && !reached[next] {
                    reached[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }
    passable
        .into_iter()
        .zip(reached)
        .all(|(floor, seen)| !floor || seen)
}
