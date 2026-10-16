//! Cells a player can only reach by finding a `SECRET_DOOR`.

use std::collections::VecDeque;

use crate::level::terrain::{ENTRANCE, ENTRANCE_SP, EXIT, SECRET_DOOR, WALL, WALL_DECO};

fn is_wall(tile: i32) -> bool {
    matches!(tile, WALL | WALL_DECO)
}

/// Sorted cells behind secret doors, including the doors themselves.
///
/// Connectivity is orthogonal and treats every non-wall tile as open, so
/// locked doors, chasms, and furniture never make a room look hidden. A cell
/// counts only when the arrival flood fill misses it and a flood from some
/// secret door reaches it.
pub(crate) fn hidden_cells(tiles: &[i32], width: i32) -> Vec<u32> {
    let width = width.max(1) as usize;
    let Some(start) = tiles
        .iter()
        .position(|&tile| matches!(tile, ENTRANCE | ENTRANCE_SP))
        .or_else(|| tiles.iter().position(|&tile| tile == EXIT))
    else {
        return Vec::new();
    };

    let mut reached = vec![false; tiles.len()];
    flood(tiles, width, &mut reached, [start], |tile| {
        !is_wall(tile) && tile != SECRET_DOOR
    });

    let doors: Vec<usize> = (0..tiles.len())
        .filter(|&cell| tiles[cell] == SECRET_DOOR)
        .collect();
    let mut hidden = vec![false; tiles.len()];
    for &door in &doors {
        hidden[door] = true;
    }
    let mut seen = reached.clone();
    flood(tiles, width, &mut seen, doors.iter().copied(), |tile| {
        !is_wall(tile)
    });
    for cell in 0..tiles.len() {
        if seen[cell] && !reached[cell] {
            hidden[cell] = true;
        }
    }

    (0..tiles.len())
        .filter(|&cell| hidden[cell])
        .map(|cell| cell as u32)
        .collect()
}

fn flood(
    tiles: &[i32],
    width: usize,
    seen: &mut [bool],
    starts: impl IntoIterator<Item = usize>,
    open: impl Fn(i32) -> bool,
) {
    let mut queue = VecDeque::new();
    for start in starts {
        seen[start] = true;
        queue.push_back(start);
    }
    while let Some(cell) = queue.pop_front() {
        let x = cell % width;
        let neighbours = [
            (cell >= width).then(|| cell - width),
            (cell + width < tiles.len()).then(|| cell + width),
            (x > 0).then(|| cell - 1),
            (x + 1 < width).then(|| cell + 1),
        ];
        for next in neighbours.into_iter().flatten() {
            if !seen[next] && open(tiles[next]) {
                seen[next] = true;
                queue.push_back(next);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::terrain::{DOOR, EMPTY};

    #[test]
    fn secret_room_behind_secret_door_is_hidden() {
        const W: i32 = WALL;
        const E: i32 = EMPTY;
        // Arrival room on the left, secret room on the right.
        #[rustfmt::skip]
        let tiles = [
            W, W, W, W, W, W, W,
            W, ENTRANCE, E, SECRET_DOOR, E, E, W,
            W, E, E, W, E, E, W,
            W, W, W, W, W, W, W,
        ];
        assert_eq!(hidden_cells(&tiles, 7), vec![10, 11, 12, 18, 19]);
    }

    #[test]
    fn room_with_a_regular_door_is_not_hidden() {
        const W: i32 = WALL;
        const E: i32 = EMPTY;
        #[rustfmt::skip]
        let tiles = [
            W, W, W, W, W, W, W,
            W, ENTRANCE, E, SECRET_DOOR, E, E, W,
            W, E, E, DOOR, E, E, W,
            W, W, W, W, W, W, W,
        ];
        assert_eq!(hidden_cells(&tiles, 7), vec![10]);
    }
}
