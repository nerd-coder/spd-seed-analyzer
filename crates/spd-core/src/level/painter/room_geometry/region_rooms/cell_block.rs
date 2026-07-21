//! Prison `CellBlockRoom` and its entrance/exit variants.

use crate::level::terrain::{TerrainMap, EMPTY, EMPTY_SP, ENTRANCE_SP, EXIT, REGION_DECO, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::{fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    fill_margin(map, room, 3, WALL);

    // `internal` is an EmptyRoom in SPD, so width/height are inclusive
    // (Room.width()/height()), unlike a bare watabou Rect.
    let internal = RoomRect {
        left: room.left + 3,
        top: room.top + 3,
        right: room.right - 3,
        bottom: room.bottom - 3,
    };
    let mut rows = (internal.width() - 1) / 3;
    let mut cols = (internal.height() - 1) / 3;
    if internal.height() == 11 {
        cols -= 1;
    }
    if internal.width() == 11 {
        rows -= 1;
    }

    let w = (internal.width() - 2 - (rows - 1)) / rows;
    let h = (internal.height() - 2 - (cols - 1)) / cols;
    let w_spacing = if rows * w + (rows + 1) == internal.width() {
        1
    } else {
        2
    };
    let h_spacing = if cols * h + (cols + 1) == internal.height() {
        1
    } else {
        2
    };

    let mut top_bottom = rows > cols || (rows == cols && Random::int_max(2) == 0);
    if rows == 1 || cols == 1 {
        top_bottom = !top_bottom;
    }
    let top_bottom_doors = (rows != 1 || cols != 1).then_some(top_bottom);

    let mut open_rooms = rows * cols;
    if open_rooms == 9 {
        open_rooms -= 1;
    }
    let guarantee_open_room = matches!(room.kind, RoomKind::Entrance | RoomKind::Exit);

    for x_index in 0..rows {
        for y_index in 0..cols {
            // The center of a 3×3 block is always the central wall block.
            if rows == 3 && cols == 3 && x_index == 1 && y_index == 1 {
                continue;
            }
            let left = internal.left + 1 + x_index * (w + w_spacing);
            let top = internal.top + 1 + y_index * (h + h_spacing);
            let deco = Random::int_max(w * h) == 0 && (!guarantee_open_room || open_rooms > 1);
            if deco {
                fill_size(map, left, top, w, h, REGION_DECO);
                open_rooms -= 1;
            } else {
                fill_size(map, left, top, w, h, EMPTY_SP);
            }

            match top_bottom_doors {
                None => match Random::int_max(4) {
                    0 => set(
                        map,
                        internal.left,
                        internal.top + internal.height() / 2,
                        crate::level::terrain::DOOR,
                    ),
                    1 => set(
                        map,
                        internal.left + internal.width() / 2,
                        internal.top,
                        crate::level::terrain::DOOR,
                    ),
                    2 => set(
                        map,
                        internal.right,
                        internal.top + internal.height() / 2,
                        crate::level::terrain::DOOR,
                    ),
                    _ => set(
                        map,
                        internal.left + internal.width() / 2,
                        internal.bottom,
                        crate::level::terrain::DOOR,
                    ),
                },
                Some(true) if y_index == 0 => {
                    set(map, left + w / 2, top - 1, crate::level::terrain::DOOR)
                }
                Some(true) if y_index == cols - 1 => {
                    set(map, left + w / 2 - 1, top + h, crate::level::terrain::DOOR)
                }
                Some(true) if x_index == 0 => {
                    set(map, left - 1, top + h / 2 - 1, crate::level::terrain::DOOR)
                }
                Some(true) if x_index == rows - 1 => {
                    set(map, left + w, top + h / 2, crate::level::terrain::DOOR)
                }
                Some(false) if x_index == 0 => {
                    set(map, left - 1, top + h / 2 - 1, crate::level::terrain::DOOR)
                }
                Some(false) if x_index == rows - 1 => {
                    set(map, left + w, top + h / 2, crate::level::terrain::DOOR)
                }
                Some(false) if y_index == 0 => {
                    set(map, left + w / 2, top - 1, crate::level::terrain::DOOR)
                }
                Some(false) if y_index == cols - 1 => {
                    set(map, left + w / 2 - 1, top + h, crate::level::terrain::DOOR)
                }
                _ => {}
            }
        }
    }
    paint_transition(map, room);
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE_SP,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    for _ in 0..10_000 {
        let point = room.random_margin(3);
        if terrain_at(map, point.x, point.y) != Some(EMPTY_SP) {
            continue;
        }
        let valid = (-1..=1).all(|dy| {
            (-1..=1).all(|dx| {
                if dx == 0 && dy == 0 {
                    true
                } else {
                    terrain_at(map, point.x + dx, point.y + dy) != Some(crate::level::terrain::DOOR)
                }
            })
        });
        if !valid {
            continue;
        }
        set(map, point.x, point.y, terrain);
        if room.kind == RoomKind::Exit {
            if let Some(cell) = map.point_to_cell(point.x, point.y) {
                map.character_allowed[cell] = false;
            }
        }
        return;
    }
}

fn fill_size(map: &mut TerrainMap, x: i32, y: i32, width: i32, height: i32, terrain: i32) {
    fill_rect(map, x, y, x + width - 1, y + height - 1, terrain);
}

#[derive(Clone, Copy)]
struct RoomRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl RoomRect {
    fn width(self) -> i32 {
        self.right - self.left + 1
    }

    fn height(self) -> i32 {
        self.bottom - self.top + 1
    }
}
