//! Terrain map using SPD `Terrain` IDs for asset-aligned rendering.

use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

// Match `com.shatteredpixel.shatteredpixeldungeon.levels.Terrain`
pub const CHASM: i32 = 0;
pub const EMPTY: i32 = 1;
pub const GRASS: i32 = 2;
pub const WALL: i32 = 4;
pub const DOOR: i32 = 5;
pub const OPEN_DOOR: i32 = 6;
pub const ENTRANCE: i32 = 7;
pub const EXIT: i32 = 8;
pub const LOCKED_DOOR: i32 = 10;
pub const EMPTY_SP: i32 = 14;
pub const WATER: i32 = 29;

#[derive(Debug, Clone)]
pub struct TerrainMap {
    pub width: i32,
    pub height: i32,
    /// Absolute dungeon coords of map[0]
    pub origin_x: i32,
    pub origin_y: i32,
    /// Row-major terrain IDs (`Terrain.*`)
    pub map: Vec<i32>,
    pub passable: Vec<bool>,
}

impl TerrainMap {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn point_to_cell(&self, x: i32, y: i32) -> Option<usize> {
        let lx = x - self.origin_x;
        let ly = y - self.origin_y;
        if lx < 0 || ly < 0 || lx >= self.width || ly >= self.height {
            return None;
        }
        Some((lx + ly * self.width) as usize)
    }
}

/// Region tileset key for frontend asset lookup.
pub fn tileset_for_depth(depth: i32) -> &'static str {
    match depth {
        1..=5 => "sewers",
        6..=10 => "prison",
        11..=15 => "caves",
        16..=20 => "city",
        _ => "halls",
    }
}

/// Paint rooms as walls with empty interiors and door tiles on connections.
/// Uses SPD terrain IDs so the client can render with original tilesheets.
pub fn paint_minimal(rooms: &[Room]) -> Option<TerrainMap> {
    let placed: Vec<&Room> = rooms.iter().filter(|r| !r.is_empty()).collect();
    if placed.is_empty() {
        return None;
    }
    let min_l = placed.iter().map(|r| r.left).min()?;
    let min_t = placed.iter().map(|r| r.top).min()?;
    let max_r = placed.iter().map(|r| r.right).max()?;
    let max_b = placed.iter().map(|r| r.bottom).max()?;

    let origin_x = min_l - 1;
    let origin_y = min_t - 1;
    let width = max_r - min_l + 3;
    let height = max_b - min_t + 3;
    let len = (width * height) as usize;
    // Outside rooms: solid wall (void is wall-like for minimap)
    let mut map = vec![WALL; len];

    let idx = |x: i32, y: i32| ((x - origin_x) + (y - origin_y) * width) as usize;

    for r in &placed {
        for x in r.left..=r.right {
            for y in r.top..=r.bottom {
                let border = x == r.left || x == r.right || y == r.top || y == r.bottom;
                // Connection tunnels often paint as empty corridors; treat thin rooms as empty fill
                let is_connection = r.kind == RoomKind::Connection;
                map[idx(x, y)] = if border && !is_connection {
                    WALL
                } else if border && is_connection {
                    // keep wall shell for tunnels too
                    WALL
                } else {
                    EMPTY
                };
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
                let y = if ib - it >= 2 { (it + ib) / 2 } else { it };
                if (r.left..=r.right).contains(&il) && (r.top..=r.bottom).contains(&y) {
                    map[idx(il, y)] = DOOR;
                }
            } else if it == ib {
                let x = if ir - il >= 2 { (il + ir) / 2 } else { il };
                if (r.left..=r.right).contains(&x) && (r.top..=r.bottom).contains(&it) {
                    map[idx(x, it)] = DOOR;
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
        // Special rooms: empty_sp floor for visual distinction
        if matches!(r.kind, RoomKind::Special | RoomKind::Shop) {
            for x in (r.left + 1)..r.right {
                for y in (r.top + 1)..r.bottom {
                    if map[idx(x, y)] == EMPTY {
                        map[idx(x, y)] = EMPTY_SP;
                    }
                }
            }
        }
    }

    let passable: Vec<bool> = map
        .iter()
        .map(|&t| {
            matches!(
                t,
                EMPTY | GRASS | DOOR | OPEN_DOOR | ENTRANCE | EXIT | EMPTY_SP | WATER
            )
        })
        .collect();

    Some(TerrainMap {
        width,
        height,
        origin_x,
        origin_y,
        map,
        passable,
    })
}
