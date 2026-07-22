//! Entrance FOV and bounded walk-distance helpers used by `createMobs`.

use std::collections::VecDeque;

use crate::level::terrain::TerrainMap;

pub(super) fn shadow_cast(source: usize, map: &TerrainMap) -> Vec<bool> {
    let mut fov = vec![false; map.len()];
    fov[source] = true;
    let x = source as i32 % map.width;
    let y = source as i32 / map.width;
    for (mx, my, mxy) in [
        (1, -1, false),
        (-1, 1, true),
        (1, 1, true),
        (1, 1, false),
        (-1, 1, false),
        (1, -1, true),
        (-1, -1, true),
        (-1, -1, false),
    ] {
        scan_octant(map, &mut fov, 1, x, y, 0.0, 1.0, mx, my, mxy, 8);
    }
    fov
}

#[allow(clippy::too_many_arguments)] // direct port of ShadowCaster.scanOctant
fn scan_octant(
    map: &TerrainMap,
    fov: &mut [bool],
    mut row: i32,
    x: i32,
    y: i32,
    mut left_slope: f64,
    right_slope: f64,
    mx: i32,
    my: i32,
    mxy: bool,
    distance: i32,
) {
    if row > distance || right_slope < left_slope {
        return;
    }
    let mut blocking = false;
    let rounded = |r: i32| -> i32 {
        let ratio = r as f64 / (distance as f64 + 0.5);
        (distance as f64 * (1.0 - ratio * ratio).sqrt())
            .round()
            .min(r as f64) as i32
    };
    while row <= distance {
        if right_slope < left_slope {
            return;
        }
        let start = if left_slope == 0.0 {
            0
        } else {
            ((row as f64 - 0.5) * left_slope + 0.499).floor() as i32
        };
        let end = if right_slope == 1.0 {
            rounded(row)
        } else {
            rounded(row).min(((row as f64 + 0.5) * right_slope - 0.499).ceil() as i32)
        };
        let mut cell = if mxy {
            x + y * map.width + mx * start * map.width + my * row
        } else {
            x + y * map.width + mx * start + my * row * map.width
        };
        for col in start..=end {
            if col == end
                && blocking
                && ((row as f64 - 0.5) * right_slope - 0.499).ceil() as i32 != end
            {
                break;
            }
            if cell < 0 || cell as usize >= map.len() {
                break;
            }
            let index = cell as usize;
            fov[index] = true;
            if map.is_los_blocking(index) {
                if !blocking {
                    blocking = true;
                    if col != start {
                        scan_octant(
                            map,
                            fov,
                            row + 1,
                            x,
                            y,
                            left_slope,
                            (col as f64 - 0.5) / (row as f64 + 0.5),
                            mx,
                            my,
                            mxy,
                            distance,
                        );
                    }
                }
            } else if blocking {
                blocking = false;
                left_slope = (col as f64 - 0.5) / (row as f64 - 0.5);
            }
            cell += if mxy { mx * map.width } else { mx };
        }
        if blocking {
            return;
        }
        row += 1;
    }
}

pub(super) fn distance_limited(
    map: &TerrainMap,
    destination: usize,
    passable: &[bool],
    limit: i32,
) -> Vec<i32> {
    let mut distance = vec![i32::MAX; map.len()];
    let mut queue = VecDeque::new();
    distance[destination] = 0;
    queue.push_back(destination);
    while let Some(cell) = queue.pop_front() {
        let next = distance[cell] + 1;
        if next > limit {
            continue;
        }
        let x = cell as i32 % map.width;
        let y = cell as i32 / map.width;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= map.width || ny >= map.height {
                    continue;
                }
                let next_cell = (nx + ny * map.width) as usize;
                if passable[next_cell] && distance[next_cell] > next {
                    distance[next_cell] = next;
                    queue.push_back(next_cell);
                }
            }
        }
    }
    distance
}
