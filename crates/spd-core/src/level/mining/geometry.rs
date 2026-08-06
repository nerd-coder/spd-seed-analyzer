//! Small painter geometry helpers shared by mining room callbacks.

use crate::geom::Point;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

pub(super) fn occupy_mob(map: &mut TerrainMap, point: Point, class_name: &'static str) {
    let cell = map.point_to_cell(point.x, point.y).unwrap();
    map.mob_occupied[cell] = true;
    map.known_mobs[cell] = Some(class_name);
}

pub(super) fn point_distance(a: Point, b: Point) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    f64::from(dx * dx + dy * dy).sqrt() as f32
}

pub(super) fn neighbour8(width: i32) -> [isize; 8] {
    let w = width as isize;
    [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1]
}

pub(super) fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(cell) = map.point_to_cell(x, y) {
        map.map[cell] = terrain;
    }
}

pub(super) fn fill_room(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill_rect(
        map,
        room.left,
        room.top,
        room.right,
        room.bottom,
        margin,
        terrain,
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn fill_rect(
    map: &mut TerrainMap,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    margin: i32,
    terrain: i32,
) {
    for x in left + margin..=right - margin {
        for y in top + margin..=bottom - margin {
            set(map, x, y, terrain);
        }
    }
}

pub(super) fn fill_ellipse(map: &mut TerrainMap, room: &Room, margin: i32, terrain: i32) {
    fill_ellipse_rect(
        map,
        room.left + margin,
        room.top + margin,
        room.right - margin,
        room.bottom - margin,
        terrain,
    );
}

pub(super) fn fill_ellipse_rect(
    map: &mut TerrainMap,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    terrain: i32,
) {
    let width = right - left + 1;
    let height = bottom - top + 1;
    let rh = height as f64 / 2.0;
    let rw = width as f64 / 2.0;
    for row in 0..height {
        let y = -rh + 0.5 + row as f64;
        let mut row_width = 2.0 * ((rw * rw) * (1.0 - y * y / (rh * rh))).max(0.0).sqrt();
        row_width = if width % 2 == 0 {
            (row_width / 2.0).round() * 2.0
        } else {
            (row_width / 2.0).floor() * 2.0 + 1.0
        };
        let row_width = row_width as i32;
        let start = left + (width - row_width) / 2;
        for x in start..start + row_width {
            set(map, x, top + row, terrain);
        }
    }
}

pub(super) fn fill_raw_rect(
    map: &mut TerrainMap,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    margin: i32,
    terrain: i32,
) {
    for x in left + margin..right - margin {
        for y in top + margin..bottom - margin {
            set(map, x, y, terrain);
        }
    }
}

pub(super) fn fill_ellipse_raw_rect(
    map: &mut TerrainMap,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    terrain: i32,
) {
    let width = right - left;
    let height = bottom - top;
    let radius_h = height as f64 / 2.0;
    let radius_w = width as f64 / 2.0;
    for row in 0..height {
        let y = -radius_h + 0.5 + row as f64;
        let mut row_width = 2.0
            * ((radius_w * radius_w) * (1.0 - y * y / (radius_h * radius_h)))
                .max(0.0)
                .sqrt();
        row_width = if width % 2 == 0 {
            (row_width / 2.0).round() * 2.0
        } else {
            (row_width / 2.0).floor() * 2.0 + 1.0
        };
        let row_width = row_width as i32;
        let start = left + (width - row_width) / 2;
        for x in start..start + row_width {
            set(map, x, top + row, terrain);
        }
    }
}
