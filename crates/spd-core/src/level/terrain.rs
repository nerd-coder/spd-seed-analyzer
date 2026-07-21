//! Minimal terrain map for headless loot placement.

use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

pub const WALL: i32 = 0;
pub const EMPTY: i32 = 1;
pub const DOOR: i32 = 2;
pub const ENTRANCE: i32 = 3;
pub const EXIT: i32 = 4;

#[derive(Debug, Clone)]
pub struct TerrainMap {
    pub width: i32,
    pub height: i32,
    pub map: Vec<i32>,
    pub passable: Vec<bool>,
}

impl TerrainMap {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn point_to_cell(&self, x: i32, y: i32) -> i32 {
        x + y * self.width
    }

    pub fn cell_to_xy(&self, cell: i32) -> (i32, i32) {
        (cell % self.width, cell / self.width)
    }
}

/// Paint rooms as walls with empty interiors and door tiles on connections.
pub fn paint_minimal(rooms: &[Room]) -> Option<TerrainMap> {
    let placed: Vec<&Room> = rooms.iter().filter(|r| !r.is_empty()).collect();
    if placed.is_empty() {
        return None;
    }
    let min_l = placed.iter().map(|r| r.left).min()?;
    let min_t = placed.iter().map(|r| r.top).min()?;
    let max_r = placed.iter().map(|r| r.right).max()?;
    let max_b = placed.iter().map(|r| r.bottom).max()?;

    // pad 1
    let origin_x = min_l - 1;
    let origin_y = min_t - 1;
    let width = max_r - min_l + 3;
    let height = max_b - min_t + 3;
    let len = (width * height) as usize;
    let mut map = vec![WALL; len];

    let idx = |x: i32, y: i32| ((x - origin_x) + (y - origin_y) * width) as usize;

    for r in &placed {
        for x in r.left..=r.right {
            for y in r.top..=r.bottom {
                let border = x == r.left || x == r.right || y == r.top || y == r.bottom;
                map[idx(x, y)] = if border { WALL } else { EMPTY };
            }
        }
        // doors along connections: midpoint of shared edge
        for &oid in &r.connected {
            let o = rooms.iter().find(|x| x.id == oid)?;
            if o.is_empty() {
                continue;
            }
            let il = r.left.max(o.left);
            let ir = r.right.min(o.right);
            let it = r.top.max(o.top);
            let ib = r.bottom.min(o.bottom);
            if il == ir {
                // vertical edge
                let y = (it + ib) / 2;
                if y > it && y < ib {
                    map[idx(il, y)] = DOOR;
                } else if it < ib {
                    map[idx(il, it + 1)] = DOOR;
                }
            } else if it == ib {
                let x = (il + ir) / 2;
                if x > il && x < ir {
                    map[idx(x, it)] = DOOR;
                } else if il < ir {
                    map[idx(il + 1, it)] = DOOR;
                }
            }
        }
        if r.kind == RoomKind::Entrance {
            let cx = (r.left + r.right) / 2;
            let cy = (r.top + r.bottom) / 2;
            map[idx(cx, cy)] = ENTRANCE;
        }
        if r.kind == RoomKind::Exit {
            let cx = (r.left + r.right) / 2;
            let cy = (r.top + r.bottom) / 2;
            map[idx(cx, cy)] = EXIT;
        }
    }

    let passable: Vec<bool> = map
        .iter()
        .map(|&t| t == EMPTY || t == DOOR || t == ENTRANCE || t == EXIT)
        .collect();

    // Shift room coords? keep absolute; map uses origin offset for indexing only
    // Store width/height and origin in map for cell conversion
    Some(TerrainMap {
        width,
        height,
        map,
        passable,
    })
}
