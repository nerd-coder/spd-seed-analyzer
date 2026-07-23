//! Terrain map using SPD `Terrain` IDs for asset-aligned rendering.

use crate::items::model::GeneratedItem;
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
pub const EMBERS: i32 = 9;
pub const LOCKED_DOOR: i32 = 10;
pub const PEDESTAL: i32 = 11;
/// SPD `Terrain.SECRET_DOOR` — hidden wall door.
pub const SECRET_DOOR: i32 = 16;
pub const WALL_DECO: i32 = 12;
pub const EMPTY_SP: i32 = 14;
pub const HIGH_GRASS: i32 = 15;
pub const SECRET_TRAP: i32 = 17;
pub const TRAP: i32 = 18;
pub const INACTIVE_TRAP: i32 = 19;
pub const EMPTY_DECO: i32 = 20;
pub const WELL: i32 = 24;
pub const STATUE: i32 = 25;
pub const STATUE_SP: i32 = 26;
pub const BOOKSHELF: i32 = 27;
pub const ALCHEMY: i32 = 28;
pub const WATER: i32 = 29;
pub const FURROWED_GRASS: i32 = 30;
pub const CRYSTAL_DOOR: i32 = 31;
pub const CUSTOM_DECO_EMPTY: i32 = 32;
pub const REGION_DECO: i32 = 33;
pub const REGION_DECO_ALT: i32 = 34;
pub const ENTRANCE_SP: i32 = 37;

#[derive(Debug, Clone)]
pub(crate) struct KnownHeap {
    pub heap_type: &'static str,
    pub items: Vec<GeneratedItem>,
}

#[derive(Debug, Clone)]
pub(crate) struct KnownBlob {
    pub class_name: &'static str,
    pub always_visible: bool,
    pub cells: Vec<(usize, u32)>,
}

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
    /// Room-specific `canPlaceWater` mask used by the post-paint lake pass.
    pub water_allowed: Vec<bool>,
    /// Room-specific `canPlaceGrass` mask used by the post-paint grass pass.
    pub grass_allowed: Vec<bool>,
    /// Room-specific `canPlaceTrap` mask used by the post-paint trap pass.
    pub trap_allowed: Vec<bool>,
    /// Room-specific `canPlaceItem` mask used by `randomDropCell`.
    pub item_allowed: Vec<bool>,
    /// Room-specific `canPlaceCharacter` mask used by mob/NPC placement.
    pub character_allowed: Vec<bool>,
    /// Cells occupied by mobs placed during room paint.
    pub mob_occupied: Vec<bool>,
    /// Cells occupied by plants; Java rejects these separately during mob placement.
    pub plant_occupied: Vec<bool>,
    /// Exact known mob identity for room-painted and ambient cells.
    pub known_mobs: Vec<Option<&'static str>>,
    /// Cells occupied by heaps placed during room paint.
    pub heap_occupied: Vec<bool>,
    /// Exact room-painted heap facts retained for the structured map report.
    pub(crate) known_heaps: Vec<Option<KnownHeap>>,
    /// Active room-painted blobs retained for the structured map report.
    pub(crate) known_blobs: Vec<KnownBlob>,
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

    pub(crate) fn record_heap(
        &mut self,
        cell: usize,
        heap_type: &'static str,
        item: GeneratedItem,
    ) {
        self.heap_occupied[cell] = true;
        if let Some(heap) = &mut self.known_heaps[cell] {
            heap.items.push(item);
        } else {
            self.known_heaps[cell] = Some(KnownHeap {
                heap_type,
                items: vec![item],
            });
        }
    }

    pub(crate) fn record_blob_cell(
        &mut self,
        class_name: &'static str,
        always_visible: bool,
        cell: usize,
        value: u32,
    ) {
        assert!(cell < self.len(), "blob cell must be inside the map");
        if let Some(blob) = self
            .known_blobs
            .iter_mut()
            .find(|blob| blob.class_name == class_name)
        {
            debug_assert_eq!(blob.always_visible, always_visible);
            if let Some((_, concentration)) = blob
                .cells
                .iter_mut()
                .find(|(known_cell, _)| *known_cell == cell)
            {
                // `Blob.seed` adds to the cell's current concentration and
                // volume when a class is seeded into the same cell again.
                *concentration += value;
            } else {
                blob.cells.push((cell, value));
            }
        } else {
            self.known_blobs.push(KnownBlob {
                class_name,
                always_visible,
                cells: vec![(cell, value)],
            });
        }
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
            WALL | WALL_DECO
                | CRYSTAL_DOOR
                | STATUE
                | STATUE_SP
                | BOOKSHELF
                | DOOR
                | LOCKED_DOOR
                | SECRET_DOOR
                | REGION_DECO
                | REGION_DECO_ALT
        )
    }

    pub fn is_los_blocking(&self, cell: usize) -> bool {
        matches!(
            self.map[cell],
            WALL | DOOR | LOCKED_DOOR | SECRET_DOOR | WALL_DECO | HIGH_GRASS | BOOKSHELF
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
            | EMBERS
            | DOOR
            | OPEN_DOOR
            | ENTRANCE
            | ENTRANCE_SP
            | EXIT
            | PEDESTAL
            | EMPTY_SP
            | EMPTY_DECO
            | INACTIVE_TRAP
            | WATER
            | SECRET_TRAP // SECRET_TRAP = EMPTY | SECRET
            | CUSTOM_DECO_EMPTY
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

/// Pinned `RegularPainter.paint` bounds normalization. Java mutates every room
/// into level-local coordinates before any room painter computes centers or
/// random points; preserving that shift matters because integer division is
/// not translation-invariant for negative odd coordinates.
pub fn shift_rooms_for_painter(rooms: &mut [Room], chasm_feeling: bool) {
    let Some(left_most) = rooms.iter().map(|room| room.left).min() else {
        return;
    };
    let Some(top_most) = rooms.iter().map(|room| room.top).min() else {
        return;
    };
    let padding = if chasm_feeling { 2 } else { 1 };
    let dx = padding - left_most;
    let dy = padding - top_most;
    for room in rooms {
        room.shift(dx, dy);
    }
}

/// Paint the initial wall/interior terrain before room and connection painters.
/// Uses SPD terrain IDs so the client can render with original tilesheets.
pub fn paint_minimal(rooms: &[Room]) -> Option<TerrainMap> {
    paint_minimal_with_chasm(rooms, false)
}

/// RegularPainter's level bounds/default terrain. Chasm-feeling floors use two
/// cells of padding and initialize the outside of rooms to chasm.
pub fn paint_minimal_with_chasm(rooms: &[Room], chasm_feeling: bool) -> Option<TerrainMap> {
    let placed: Vec<&Room> = rooms.iter().filter(|r| !r.is_empty()).collect();
    if placed.is_empty() {
        return None;
    }
    let min_l = placed.iter().map(|r| r.left).min()?;
    let min_t = placed.iter().map(|r| r.top).min()?;
    let max_r = placed.iter().map(|r| r.right).max()?;
    let max_b = placed.iter().map(|r| r.bottom).max()?;

    let padding = if chasm_feeling { 2 } else { 1 };
    let origin_x = min_l - padding;
    let origin_y = min_t - padding;
    let width = max_r - min_l + 1 + 2 * padding;
    let height = max_b - min_t + 1 + 2 * padding;
    let len = (width * height) as usize;
    let mut map = vec![if chasm_feeling { CHASM } else { WALL }; len];

    let idx = |x: i32, y: i32| ((x - origin_x) + (y - origin_y) * width) as usize;

    for r in &placed {
        for x in r.left..=r.right {
            for y in r.top..=r.bottom {
                let border = x == r.left || x == r.right || y == r.top || y == r.bottom;
                // Connection tunnels often paint as empty corridors; treat thin rooms as empty fill
                let is_connection = r.kind == RoomKind::Connection;
                map[idx(x, y)] = if is_connection {
                    // Java starts the whole level as wall; connection painters carve
                    // only their tunnel path instead of receiving a pre-cleared room.
                    WALL
                } else if border {
                    WALL
                } else {
                    EMPTY
                };
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
    let water_allowed = vec![true; len];
    let grass_allowed = vec![true; len];
    let trap_allowed = vec![true; len];
    let item_allowed = vec![true; len];
    let character_allowed = vec![true; len];
    let mob_occupied = vec![false; len];
    let plant_occupied = vec![false; len];
    let known_mobs = vec![None; len];
    let heap_occupied = vec![false; len];
    let known_heaps = vec![None; len];
    let known_blobs = Vec::new();
    let trap_destroys_items = vec![false; len];
    let trap_names = vec![None; len];

    Some(TerrainMap {
        width,
        height,
        origin_x,
        origin_y,
        map,
        passable,
        water_allowed,
        grass_allowed,
        trap_allowed,
        item_allowed,
        character_allowed,
        mob_occupied,
        plant_occupied,
        known_mobs,
        heap_occupied,
        known_heaps,
        known_blobs,
        trap_destroys_items,
        trap_names,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chasm_feeling_adds_two_cell_chasm_padding() {
        let mut room = Room::new(0, "EmptyRoom", RoomKind::Standard, 1, 4, 4, 10, 4, 10);
        room.left = 5;
        room.top = 7;
        room.right = 11;
        room.bottom = 13;

        let normal = paint_minimal(std::slice::from_ref(&room)).expect("normal map");
        let chasm = paint_minimal_with_chasm(&[room], true).expect("chasm map");
        assert_eq!(chasm.width, normal.width + 2);
        assert_eq!(chasm.height, normal.height + 2);
        assert_eq!(chasm.origin_x, normal.origin_x - 1);
        assert_eq!(chasm.origin_y, normal.origin_y - 1);
        assert_eq!(chasm.map[0], CHASM);
    }
}
