//! Terrain map using SPD `Terrain` IDs for asset-aligned rendering.

use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

// Match `com.shatteredpixel.shatteredpixeldungeon.levels.Terrain`
#[allow(dead_code)] // reserved for chasm feeling / caves merge paint
pub const CHASM: i32 = 0;
pub const EMPTY: i32 = 1;
pub const GRASS: i32 = 2;
pub const WALL: i32 = 4;
pub const DOOR: i32 = 5;
pub const OPEN_DOOR: i32 = 6;
pub const ENTRANCE: i32 = 7;
pub const EXIT: i32 = 8;
pub const LOCKED_DOOR: i32 = 10;
/// SPD `Terrain.SECRET_DOOR` — hidden wall door.
pub const SECRET_DOOR: i32 = 16;
pub const WALL_DECO: i32 = 12;
pub const EMPTY_SP: i32 = 14;
pub const HIGH_GRASS: i32 = 15;
pub const SECRET_TRAP: i32 = 17;
pub const TRAP: i32 = 18;
pub const EMPTY_DECO: i32 = 20;
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
    /// Parallel to `map`: trap destroys dropped items (randomDropCell filter).
    pub trap_destroys_items: Vec<bool>,
    /// Optional trap class name for debugging / future UI.
    pub trap_names: Vec<Option<&'static str>>,
}

impl TerrainMap {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn point_to_cell(&self, x: i32, y: i32) -> Option<usize> {
        let lx = x - self.origin_x;
        let ly = y - self.origin_y;
        if lx < 0 || ly < 0 || lx >= self.width || ly >= self.height {
            return None;
        }
        Some((lx + ly * self.width) as usize)
    }

    /// Approximate SPD `Terrain.SOLID` flag for painted tiles.
    pub fn is_solid(&self, cell: usize) -> bool {
        matches!(
            self.map[cell],
            WALL | WALL_DECO | DOOR | LOCKED_DOOR | SECRET_DOOR
        )
    }

    pub fn recompute_passable(&mut self) {
        self.passable = self.map.iter().copied().map(is_passable_tile).collect();
    }

    /// Approximate SPD `openSpace`: not solid, and some diagonal corner pair is open.
    pub fn is_open_space(&self, cell: usize) -> bool {
        if self.is_solid(cell) {
            return false;
        }
        let w = self.width as usize;
        let x = cell % w;
        let y = cell / w;
        // CIRCLE8: N, NE, E, SE, S, SW, W, NW — diagonals at odd indices
        let deltas: [(i32, i32); 8] = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        for j in (1..8).step_by(2) {
            let (dx, dy) = deltas[j];
            if self.neighbor_solid(x, y, dx, dy) {
                continue;
            }
            let (dx1, dy1) = deltas[(j + 1) % 8];
            let (dx2, dy2) = deltas[(j + 2) % 8];
            if !self.neighbor_solid(x, y, dx1, dy1) && !self.neighbor_solid(x, y, dx2, dy2) {
                return true;
            }
        }
        false
    }

    fn neighbor_solid(&self, x: usize, y: usize, dx: i32, dy: i32) -> bool {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= self.width || ny >= self.height {
            return true;
        }
        let cell = (nx + ny * self.width) as usize;
        self.is_solid(cell)
    }
}

/// SPD `Terrain.flags[t] & PASSABLE` approximation for our tile subset.
pub fn is_passable_tile(t: i32) -> bool {
    matches!(
        t,
        EMPTY
            | GRASS
            | HIGH_GRASS
            | DOOR
            | OPEN_DOOR
            | ENTRANCE
            | EXIT
            | EMPTY_SP
            | EMPTY_DECO
            | WATER
            | SECRET_TRAP // SECRET_TRAP = EMPTY | SECRET
    )
    // TRAP is AVOID (not passable)
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

    let passable: Vec<bool> = map.iter().copied().map(is_passable_tile).collect();
    let trap_destroys_items = vec![false; len];
    let trap_names = vec![None; len];

    Some(TerrainMap {
        width,
        height,
        origin_x,
        origin_y,
        map,
        passable,
        trap_destroys_items,
        trap_names,
    })
}
