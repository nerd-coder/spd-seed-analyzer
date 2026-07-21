//! Prison `PillarsRoom` and its entrance/exit variants.

use crate::level::terrain::{TerrainMap, ENTRANCE, EXIT, WALL};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

use super::{fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, crate::level::terrain::EMPTY);

    let min_dim = room.width().min(room.height());
    if min_dim == 7 || (room.size_factor == 1 && Random::int_max(2) == 0) {
        paint_two_pillars(map, room, min_dim);
    } else {
        paint_four_pillars(map, room, min_dim);
    }
    paint_transition(map, room);
}

fn paint_two_pillars(map: &mut TerrainMap, room: &Room, min_dim: i32) {
    let pillar_inset = if min_dim >= 11 { 2 } else { 1 };
    let pillar_size = (min_dim - 3) / 2 - pillar_inset;
    let (mut pillar_x, mut pillar_y) = if Random::int_max(2) == 0 {
        (
            Random::int_range_inclusive(
                room.left + 1 + pillar_inset,
                room.right - pillar_size - pillar_inset,
            ),
            room.top + 1 + pillar_inset,
        )
    } else {
        (
            room.left + 1 + pillar_inset,
            Random::int_range_inclusive(
                room.top + 1 + pillar_inset,
                room.bottom - pillar_size - pillar_inset,
            ),
        )
    };

    fill_size(map, pillar_x, pillar_y, pillar_size, WALL);
    pillar_x = room.right - (pillar_x - room.left + pillar_size - 1);
    pillar_y = room.bottom - (pillar_y - room.top + pillar_size - 1);
    fill_size(map, pillar_x, pillar_y, pillar_size, WALL);
}

fn paint_four_pillars(map: &mut TerrainMap, room: &Room, min_dim: i32) {
    let pillar_inset = if min_dim >= 12 { 2 } else { 1 };
    let pillar_size = (min_dim - 6) / (pillar_inset + 1);
    let x_spaces = (room.width() - 2 * pillar_inset - pillar_size - 2) as f32;
    let y_spaces = (room.height() - 2 * pillar_inset - pillar_size - 2) as f32;
    let min_spaces = x_spaces.min(y_spaces);
    let percent_skew = java_round(Random::float() * min_spaces) as f32 / min_spaces;
    let x_skew = java_round(percent_skew * x_spaces);
    let y_skew = java_round(percent_skew * y_spaces);

    fill_size(
        map,
        room.left + 1 + pillar_inset + x_skew,
        room.top + 1 + pillar_inset,
        pillar_size,
        WALL,
    );
    fill_size(
        map,
        room.right - pillar_size - pillar_inset,
        room.top + 1 + pillar_inset + y_skew,
        pillar_size,
        WALL,
    );
    fill_size(
        map,
        room.right - pillar_size - pillar_inset - x_skew,
        room.bottom - pillar_size - pillar_inset,
        pillar_size,
        WALL,
    );
    fill_size(
        map,
        room.left + 1 + pillar_inset,
        room.bottom - pillar_size - pillar_inset - y_skew,
        pillar_size,
        WALL,
    );
}

fn paint_transition(map: &mut TerrainMap, room: &Room) {
    let terrain = match room.kind {
        RoomKind::Entrance => ENTRANCE,
        RoomKind::Exit => EXIT,
        _ => return,
    };
    for _ in 0..10_000 {
        let point = room.random_margin(2);
        // Pinned code deliberately ignores the north neighbour.
        let valid = [(0, 0), (-1, 0), (1, 0), (0, 1)]
            .into_iter()
            .all(|(dx, dy)| terrain_at(map, point.x + dx, point.y + dy) != Some(WALL));
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

fn fill_size(map: &mut TerrainMap, x: i32, y: i32, size: i32, terrain: i32) {
    fill_rect(map, x, y, x + size - 1, y + size - 1, terrain);
}

fn java_round(value: f32) -> i32 {
    (value + 0.5).floor() as i32
}
