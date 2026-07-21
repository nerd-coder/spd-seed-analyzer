//! Pinned SPD `Maze.generate` and `SecretMazeRoom.paint` geometry.

use std::collections::VecDeque;

use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, EMPTY, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

const FILLED: bool = true;
const EMPTY_CELL: bool = false;
const MAZE_FAILURE_LIMIT: i32 = 2_500;
const FILLED_PICK_LIMIT: usize = 10_000;

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    let width = room.width();
    let height = room.height();
    let mut maze = vec![EMPTY_CELL; (width * height) as usize];

    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                maze[index(width, x, y)] = FILLED;
            }
        }
    }
    for &other in &room.connected {
        if let Some(door) = doors.get(room_index, other) {
            let x = door.x - room.left;
            let y = door.y - room.top;
            if (0..width).contains(&x) && (0..height).contains(&y) {
                maze[index(width, x, y)] = EMPTY_CELL;
            }
        }
    }

    generate(&mut maze, width, height);
    for x in 0..width {
        for y in 0..height {
            if let Some(cell) = map.point_to_cell(room.left + x, room.top + y) {
                map.map[cell] = if maze[index(width, x, y)] {
                    WALL
                } else {
                    EMPTY
                };
            }
        }
    }

    // SecretMazeRoom finds the farthest reachable cell for its chest. This is
    // RNG-free but retained so the painter follows the complete Java path.
    if let Some(entrance) = room.connected.first().and_then(|&other| {
        doors
            .get(room_index, other)
            .map(|door| (door.x - room.left, door.y - room.top))
    }) {
        let passable: Vec<bool> = maze.iter().map(|&filled| !filled).collect();
        let distances = distance_map(
            width,
            height,
            index(width, entrance.0, entrance.1),
            &passable,
        );
        let mut best_distance = 0;
        let mut best_cell = 0;
        for (cell, &distance) in distances.iter().enumerate() {
            if distance != i32::MAX && distance > best_distance {
                best_distance = distance;
                best_cell = cell;
            }
        }
        if best_distance > 0 {
            let prize_cell = map
                .point_to_cell(
                    room.left + best_cell as i32 % width,
                    room.top + best_cell as i32 / width,
                )
                .expect("maze prize inside map");
            map.heap_occupied[prize_cell] = true;
        }
    }
}

fn generate(maze: &mut [bool], width: i32, height: i32) {
    let mut failures = 0;
    while failures < MAZE_FAILURE_LIMIT {
        let Some((mut x, mut y)) = random_filled(maze, width, height) else {
            return;
        };
        let Some(direction) = decide_direction(maze, width, height, x, y) else {
            failures += 1;
            continue;
        };

        failures = 0;
        let mut moves = 0;
        loop {
            x += direction.0;
            y += direction.1;
            maze[index(width, x, y)] = FILLED;
            moves += 1;

            // Java evaluates Random.Int(moves) before checkValidMove.
            if Random::int_max(moves) != 0
                || !check_valid_move(maze, width, height, x, y, direction)
            {
                break;
            }
        }
    }
}

fn random_filled(maze: &[bool], width: i32, height: i32) -> Option<(i32, i32)> {
    // Boundary walls guarantee success for valid SecretMazeRoom dimensions.
    // The cap only protects WASM from malformed callers and is not reached by
    // builder-produced rooms.
    for _ in 0..FILLED_PICK_LIMIT {
        let x = Random::int_max(width);
        let y = Random::int_max(height);
        if maze[index(width, x, y)] == FILLED {
            return Some((x, y));
        }
    }
    None
}

fn decide_direction(maze: &[bool], width: i32, height: i32, x: i32, y: i32) -> Option<(i32, i32)> {
    for (bound, direction) in [(4, (0, -1)), (3, (1, 0)), (2, (0, 1))] {
        if Random::int_max(bound) == 0 && check_valid_move(maze, width, height, x, y, direction) {
            return Some(direction);
        }
    }
    check_valid_move(maze, width, height, x, y, (-1, 0)).then_some((-1, 0))
}

fn check_valid_move(
    maze: &[bool],
    width: i32,
    height: i32,
    mut x: i32,
    mut y: i32,
    direction: (i32, i32),
) -> bool {
    let side_x = 1 - direction.0.abs();
    let side_y = 1 - direction.1.abs();

    x += direction.0;
    y += direction.1;
    if x <= 0 || x >= width - 1 || y <= 0 || y >= height - 1 {
        return false;
    }
    if maze[index(width, x, y)]
        || maze[index(width, x + side_x, y + side_y)]
        || maze[index(width, x - side_x, y - side_y)]
    {
        return false;
    }

    x += direction.0;
    y += direction.1;
    if x <= 0 || x >= width - 1 || y <= 0 || y >= height - 1 {
        return false;
    }
    !maze[index(width, x, y)]
        && !maze[index(width, x + side_x, y + side_y)]
        && !maze[index(width, x - side_x, y - side_y)]
}

pub(super) fn distance_map(
    width: i32,
    height: i32,
    destination: usize,
    passable: &[bool],
) -> Vec<i32> {
    let mut distances = vec![i32::MAX; passable.len()];
    let mut queue = VecDeque::with_capacity(passable.len());
    distances[destination] = 0;
    queue.push_back(destination);

    while let Some(cell) = queue.pop_front() {
        let x = cell as i32 % width;
        let y = cell as i32 / width;
        let next_distance = distances[cell] + 1;
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
                let next = index(width, nx, ny);
                if passable[next] && distances[next] > next_distance {
                    distances[next] = next_distance;
                    queue.push_back(next);
                }
            }
        }
    }
    distances
}

fn index(width: i32, x: i32, y: i32) -> usize {
    (x + y * width) as usize
}
