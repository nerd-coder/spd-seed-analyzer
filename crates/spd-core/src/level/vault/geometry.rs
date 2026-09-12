//! Painter primitives matching SPD `Painter` plus vault carpets and flame traps.

use crate::geom::Point;
use crate::level::painter::DoorMap;
use crate::level::terrain::{TerrainMap, INACTIVE_TRAP, WALL, WALL_DECO};
use crate::rooms::room::Room;

pub(super) const CITY_QUEST_TEX: &str = "environment/custom_tiles/city_quest.png";
pub(super) use crate::level::carpet::{CITY_PEDESTAL, CITY_PEDESTAL_TL, CITY_PEDESTAL_TR};

pub(super) fn cell(map: &TerrainMap, x: i32, y: i32) -> usize {
    map.point_to_cell(x, y).expect("vault cell")
}

pub(super) fn set(map: &mut TerrainMap, x: i32, y: i32, terrain: i32) {
    if let Some(i) = map.point_to_cell(x, y) {
        map.map[i] = terrain;
    }
}

pub(super) fn set_cell(map: &mut TerrainMap, cell: usize, terrain: i32) {
    map.map[cell] = terrain;
}

pub(super) fn terrain_at(map: &TerrainMap, x: i32, y: i32) -> i32 {
    map.point_to_cell(x, y).map(|i| map.map[i]).unwrap_or(WALL)
}

pub(super) fn center(room: &Room) -> Point {
    Point::new((room.left + room.right) / 2, (room.top + room.bottom) / 2)
}

pub(super) fn wide(room: &Room) -> bool {
    painter_w(room) > painter_h(room)
}

/// SPD `Rect.width()` / `Painter.fill` size (`right - left`), not cell count.
pub(super) fn painter_w(room: &Room) -> i32 {
    room.right - room.left
}

pub(super) fn painter_h(room: &Room) -> i32 {
    room.bottom - room.top
}

pub(super) fn occupy_heap(map: &mut TerrainMap, cell: usize) {
    map.heap_occupied[cell] = true;
}

pub(super) fn occupy_mob(map: &mut TerrainMap, cell: usize) {
    map.mob_occupied[cell] = true;
}

pub(super) fn chebyshev(map: &TerrainMap, a: usize, b: usize) -> i32 {
    let w = map.width;
    let ax = a as i32 % w;
    let ay = a as i32 / w;
    let bx = b as i32 % w;
    let by = b as i32 / w;
    (ax - bx).abs().max((ay - by).abs())
}

pub(super) fn true_distance(map: &TerrainMap, a: usize, b: usize) -> f32 {
    let w = map.width;
    let dx = (a as i32 % w - b as i32 % w) as f32;
    let dy = (a as i32 / w - b as i32 / w) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub(super) fn point_distance(a: Point, b: Point) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

pub(super) fn gate(min: i32, value: i32, max: i32) -> i32 {
    value.clamp(min, max)
}

pub(super) fn neighbours4(width: i32) -> [i32; 4] {
    [-width, -1, 1, width]
}

pub(super) fn neighbours8(width: i32) -> [i32; 8] {
    [
        -width - 1,
        -width,
        -width + 1,
        -1,
        1,
        width - 1,
        width,
        width + 1,
    ]
}

/// `Painter.fill(level, x, y, w, h, value)` — half-open width/height.
pub(super) fn fill_wh(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, terrain: i32) {
    if w <= 0 || h <= 0 {
        return;
    }
    for py in y..y + h {
        for px in x..x + w {
            set(map, px, py, terrain);
        }
    }
}

pub(super) fn fill_room(map: &mut TerrainMap, room: &Room, terrain: i32) {
    // Room.width() is Rect.width()+1 (inclusive right/bottom).
    fill_wh(
        map,
        room.left,
        room.top,
        room.width(),
        room.height(),
        terrain,
    );
}

pub(super) fn fill_margin(map: &mut TerrainMap, room: &Room, m: i32, terrain: i32) {
    fill_wh(
        map,
        room.left + m,
        room.top + m,
        room.width() - 2 * m,
        room.height() - 2 * m,
        terrain,
    );
}

pub(super) fn fill_insets(
    map: &mut TerrainMap,
    room: &Room,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    terrain: i32,
) {
    fill_wh(
        map,
        room.left + left,
        room.top + top,
        room.width() - (left + right),
        room.height() - (top + bottom),
        terrain,
    );
}

/// `Painter.fill(level, roomRect)` for an `EmptyRoom.set(l,t,r,b)` — inclusive width.
pub(super) fn fill_rect(
    map: &mut TerrainMap,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    terrain: i32,
) {
    fill_wh(map, left, top, right - left + 1, bottom - top + 1, terrain);
}

pub(super) fn draw_inside(map: &mut TerrainMap, room: &Room, from: Point, n: i32, terrain: i32) {
    let (dx, dy) = if from.x == room.left {
        (1, 0)
    } else if from.x == room.right {
        (-1, 0)
    } else if from.y == room.top {
        (0, 1)
    } else if from.y == room.bottom {
        (0, -1)
    } else {
        return;
    };
    let mut x = from.x + dx;
    let mut y = from.y + dy;
    for _ in 0..n {
        set(map, x, y, terrain);
        x += dx;
        y += dy;
    }
}

/// SPD `Painter.drawLine` float stepper (not axis-aligned-only).
pub(super) fn draw_line(map: &mut TerrainMap, from: Point, to: Point, terrain: i32) {
    let mut x = from.x as f32;
    let mut y = from.y as f32;
    let mut dx = (to.x - from.x) as f32;
    let mut dy = (to.y - from.y) as f32;
    let moving_by_x = dx.abs() >= dy.abs();
    if moving_by_x {
        if dx != 0.0 {
            dy /= dx.abs();
            dx /= dx.abs();
        }
    } else if dy != 0.0 {
        dx /= dy.abs();
        dy /= dy.abs();
    }
    set(map, x.round() as i32, y.round() as i32, terrain);
    while (moving_by_x && to.x as f32 != x) || (!moving_by_x && to.y as f32 != y) {
        x += dx;
        y += dy;
        set(map, x.round() as i32, y.round() as i32, terrain);
    }
}

pub(super) fn fill_ellipse_margin(map: &mut TerrainMap, room: &Room, m: i32, terrain: i32) {
    fill_ellipse_wh(
        map,
        room.left + m,
        room.top + m,
        room.width() - 2 * m,
        room.height() - 2 * m,
        terrain,
    );
}

pub(super) fn fill_ellipse_wh(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, terrain: i32) {
    let rad_h = h as f64 / 2.0;
    let rad_w = w as f64 / 2.0;
    for i in 0..h {
        let row_y = -rad_h + 0.5 + i as f64;
        let mut row_w = 2.0
            * ((rad_w * rad_w) * (1.0 - (row_y * row_y) / (rad_h * rad_h)))
                .max(0.0)
                .sqrt();
        row_w = if w % 2 == 0 {
            (row_w / 2.0).round() * 2.0
        } else {
            (row_w / 2.0).floor() * 2.0 + 1.0
        };
        let row_w = row_w as i32;
        let cell_x = x + (w - row_w) / 2;
        fill_wh(map, cell_x, y + i, row_w, 1, terrain);
    }
}

pub(super) fn fill_diamond_wh(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, terrain: i32) {
    let mut diamond_width = w - (h - 2 - h % 2);
    diamond_width = diamond_width.max(if w % 2 == 0 { 2 } else { 3 });
    for i in 0..=h {
        fill_wh(
            map,
            x + (w - diamond_width) / 2,
            y + i,
            diamond_width,
            h - 2 * i,
            terrain,
        );
        diamond_width += 2;
        if diamond_width > w {
            break;
        }
    }
}

pub(super) fn rect_points(left: i32, top: i32, right: i32, bottom: i32) -> Vec<Point> {
    let mut points = Vec::new();
    for x in left..=right {
        for y in top..=bottom {
            points.push(Point::new(x, y));
        }
    }
    points
}

pub(super) fn door_points(rooms: &[Room], ri: usize, doors: &DoorMap) -> Vec<Point> {
    rooms[ri]
        .connected
        .iter()
        .filter_map(|&other| doors.get(ri, other).map(|d| Point::new(d.x, d.y)))
        .collect()
}

/// `VaultLevel.VaultFlameTrap.setupTrap` — inactive trap plus cooldown blob.
pub(super) fn setup_flame_trap(
    map: &mut TerrainMap,
    cell: usize,
    initial_cd: i32,
    after_cd: i32,
    triggers: i32,
) {
    map.map[cell] = INACTIVE_TRAP;
    map.trap_names[cell] = Some("VaultFlameTrap");
    // Pack layout-time cooldowns: cur | after<<8 | triggers<<16. `Blob.seed(0,0)`
    // leaves `cur[]` empty, so FloorVisualFacts would drop the blob otherwise.
    let value = initial_cd as u32 + ((after_cd as u32) << 8) + ((triggers as u32) << 16);
    map.record_blob_cell_set("VaultFlameTraps", false, cell, value);
}

pub(super) fn map_simple_image(tile_w: i32, tile_h: i32, tx: i32, ty: i32, tex_w: i32) -> Vec<i16> {
    let tex_tile_width = tex_w / 16;
    let mut data = Vec::with_capacity((tile_w * tile_h) as usize);
    let mut x = tx;
    let mut y = ty;
    for _ in 0..tile_w * tile_h {
        data.push((x + tex_tile_width * y) as i16);
        x += 1;
        if x - tx == tile_w {
            x = tx;
            y += 1;
        }
    }
    data
}

pub(super) fn add_carpet(
    map: &mut TerrainMap,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
) {
    crate::level::carpet::add(
        map,
        crate::level::carpet::VAULT_TEXTURE,
        x,
        y,
        w,
        h,
        depth,
        overrides,
    );
}

pub(super) fn vault_treasure_map(
    map: &TerrainMap,
    tile_x: i32,
    tile_y: i32,
    tile_w: i32,
    tile_h: i32,
    variance: &[u8],
) -> Vec<i16> {
    let mut data = vec![-1i16; (tile_w * tile_h) as usize];
    let w = map.width;
    for (i, slot) in data.iter_mut().enumerate() {
        let lx = i as i32 % tile_w;
        let ly = i as i32 / tile_w;
        if ly == 0 {
            if lx == 0 {
                *slot = 5 * 16 + 4;
            } else if lx == tile_w - 1 {
                *slot = 5 * 16 + 3;
            }
            continue;
        }
        let cell = cell(map, tile_x + lx, tile_y + ly);
        if map.map[cell] == crate::level::terrain::PEDESTAL {
            *slot = 7 * 16 + 3;
        } else {
            let above = (cell as i32 - w) as usize;
            if map.map[above] == WALL || map.map[above] == WALL_DECO {
                let mut tile = 6 * 16 + 2;
                if terrain_at(map, tile_x + lx + 1, tile_y + ly) == WALL {
                    tile += 1;
                } else if terrain_at(map, tile_x + lx - 1, tile_y + ly) == WALL {
                    tile += 2;
                }
                *slot = tile;
            } else if terrain_at(map, tile_x + lx + 1, tile_y + ly) == WALL {
                *slot = 6 * 16;
            } else if terrain_at(map, tile_x + lx - 1, tile_y + ly) == WALL {
                *slot = 6 * 16 + 1;
            } else {
                let var = variance.get(cell).copied().unwrap_or(0) as i16;
                *slot = 7 * 16 + var / 34;
            }
        }
    }
    data
}
