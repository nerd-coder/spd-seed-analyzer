//! Halls `SkullsRoom`, `RitualRoom`, and ritual transition variants.

use crate::geom::Point;
use crate::level::terrain::{
    TerrainMap, EMBERS, EMPTY, ENTRANCE, EXIT, PEDESTAL, REGION_DECO, STATUE, WALL,
};
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::super::super::DoorMap;
use super::super::patch_rooms;
use super::{center, door_points, draw_inside, fill_margin, fill_rect, fill_room, set};

pub(super) fn paint(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) -> Option<Point> {
    if room.name == "SkullsRoom" {
        paint_skulls(map, room, room_index, doors);
        None
    } else {
        paint_ritual(map, room, room_index, doors)
    }
}

fn paint_skulls(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_ellipse(map, room, 2, EMPTY);

    for door in door_points(room, room_index, doors) {
        let distance = if door.x == room.left || door.x == room.right {
            room.width() / 2
        } else {
            room.height() / 2
        };
        draw_inside(map, room, door, distance, EMPTY);
    }

    fill_ellipse(map, room, 4, STATUE);
    fill_ellipse(map, room, 6, WALL);
}

fn paint_ritual(
    map: &mut TerrainMap,
    room: &Room,
    room_index: usize,
    doors: &DoorMap,
) -> Option<Point> {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    // RitualRoom evaluates center before setupPatch, which evaluates center
    // again for every path-validation attempt.
    let center = center(room);
    let scale = (room.width() * room.height()).min(18 * 18);
    let fill = 0.30 + scale as f32 / 1024.0;
    let mut patch =
        patch_rooms::setup_patch(room, room_index, doors, fill, 0, !room.connected.is_empty());
    patch_rooms::clean_diagonal_edges(&mut patch, room.width() - 2);
    patch_rooms::fill_patch(map, room, &patch, REGION_DECO);

    fill_rect(
        map,
        center.x - 3,
        center.y - 3,
        center.x + 3,
        center.y + 3,
        EMPTY,
    );
    for (dx, dy) in [
        (-2, -1),
        (-1, -2),
        (2, -1),
        (1, -2),
        (-2, 1),
        (-1, 2),
        (2, 1),
        (1, 2),
    ] {
        set(map, center.x + dx, center.y + dy, STATUE);
    }
    fill_rect(
        map,
        center.x - 1,
        center.y - 1,
        center.x + 1,
        center.y + 1,
        EMBERS,
    );
    set(map, center.x, center.y, PEDESTAL);

    match room.kind {
        RoomKind::Entrance => set(map, center.x, center.y, ENTRANCE),
        RoomKind::Exit => {
            set(map, center.x, center.y, EXIT);
            if let Some(cell) = map.point_to_cell(center.x, center.y) {
                map.character_allowed[cell] = false;
            }
        }
        _ => return Some(center),
    }
    None
}

/// Pinned `Painter.fillEllipse`, including even/odd row-width rounding.
fn fill_ellipse(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    let x = room.left + margin;
    let y = room.top + margin;
    let width = room.width() - margin * 2;
    let height = room.height() - margin * 2;
    if width <= 0 || height <= 0 {
        return;
    }
    let radius_h = height as f64 / 2.0;
    let radius_w = width as f64 / 2.0;
    for row in 0..height {
        let row_y = -radius_h + 0.5 + row as f64;
        let mut row_width =
            2.0 * ((radius_w * radius_w) * (1.0 - row_y * row_y / (radius_h * radius_h))).sqrt();
        row_width = if width % 2 == 0 {
            (row_width / 2.0).round() * 2.0
        } else {
            (row_width / 2.0).floor() * 2.0 + 1.0
        };
        let row_width = row_width as i32;
        let left = x + (width - row_width) / 2;
        fill_rect(map, left, y + row, left + row_width - 1, y + row, terrain);
    }
}
