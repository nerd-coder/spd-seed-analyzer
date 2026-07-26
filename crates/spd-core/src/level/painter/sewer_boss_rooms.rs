//! Pinned SPD v3.3.8 SewerBoss room painters.
//!
//! Mob and cache heap placement is deliberately omitted from the public map,
//! but their paint-time random calls are retained for downstream parity.

use crate::geom::Point;
use crate::level::terrain::{
    TerrainMap, EMPTY, EMPTY_SP, ENTRANCE, LOCKED_EXIT, WALL, WALL_DECO, WATER,
};
use crate::random::Random;
use crate::rooms::room::Room;

use super::DoorMap;

pub(crate) fn paint(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap) -> bool {
    match room.name.as_str() {
        "SewerBossEntranceRoom" => paint_entrance(map, room, ri, doors),
        "SewerBossExitRoom" => paint_exit(map, room),
        "DiamondGooRoom" => paint_diamond(map, room, ri, doors),
        "WalledGooRoom" => paint_walled(map, room),
        "ThinPillarsGooRoom" => paint_thin_pillars(map, room, ri, doors),
        "ThickPillarsGooRoom" => paint_thick_pillars(map, room, ri, doors),
        "RatKingRoom" => paint_rat_king(map, room, ri, doors),
        _ => return false,
    }
    true
}

fn paint_entrance(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    fill_rect(
        map,
        room.left + 1,
        room.top + 1,
        room.width() - 2,
        1,
        WALL_DECO,
    );
    fill_rect(map, room.left + 1, room.top + 2, room.width() - 2, 1, WATER);

    // Room.random(3); no room-painted mobs exist yet, so the retry never fires.
    let entrance = room.random_margin(3);
    set(map, entrance.x, entrance.y, ENTRANCE);
    for door in door_points(room, ri, doors) {
        if door.y == room.top || door.y == room.top + 1 {
            draw_inside(map, room, door, 1, WATER);
        }
    }
}

fn paint_exit(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);
    let c = room.as_rect().center_room();
    fill_rect(map, c.x - 1, c.y - 1, 3, 2, WALL);
    fill_rect(map, c.x - 1, c.y + 1, 3, 1, EMPTY_SP);
    set(map, c.x, c.y, LOCKED_EXIT);
}

fn paint_diamond(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_diamond(map, room, 1, EMPTY);
    for door in door_points(room, ri, doors) {
        let (dx, dy) = inward(room, door);
        let mut p = door;
        loop {
            set(map, p.x, p.y, EMPTY_SP);
            p.x += dx;
            p.y += dy;
            if terrain_at(map, p.x, p.y) != Some(WALL) {
                break;
            }
        }
    }
    paint_cross_water(map, room);
    reject_later_water(map, room);
    let _boss_position = room.as_rect().center_room();
}

fn paint_walled(map: &mut TerrainMap, room: &Room) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY_SP);
    fill_margin(map, room, 2, EMPTY);
    let pw = (room.width() - 6) / 2;
    let ph = (room.height() - 6) / 2;
    for &(x, y, w, h) in &[
        (room.left + 2, room.top + 2, pw, 1),
        (room.left + 2, room.top + 2, 1, ph),
        (room.left + 2, room.bottom - 2, pw, 1),
        (room.left + 2, room.bottom - 1 - ph, 1, ph),
        (room.right - 1 - pw, room.top + 2, pw, 1),
        (room.right - 2, room.top + 2, 1, ph),
        (room.right - 1 - pw, room.bottom - 2, pw, 1),
        (room.right - 2, room.bottom - 1 - ph, 1, ph),
    ] {
        fill_rect(map, x, y, w, h, WALL);
    }
    paint_cross_water(map, room);
    reject_later_water(map, room);
    let _boss_position = room.as_rect().center_room();
}

fn paint_thin_pillars(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, WATER);
    let pw = (if room.width() == 14 { 4 } else { 2 }) + room.width() % 2;
    let ph = (if room.height() == 14 { 4 } else { 2 }) + room.height() % 2;
    let yo = if room.height() < 12 { 2 } else { 3 };
    fill_rect(
        map,
        room.left + (room.width() - pw) / 2,
        room.top + yo,
        pw,
        1,
        WALL,
    );
    fill_rect(
        map,
        room.left + (room.width() - pw) / 2,
        room.bottom - yo,
        pw,
        1,
        WALL,
    );
    let xo = if room.width() < 12 { 2 } else { 3 };
    fill_rect(
        map,
        room.left + xo,
        room.top + (room.height() - ph) / 2,
        1,
        ph,
        WALL,
    );
    fill_rect(
        map,
        room.right - xo,
        room.top + (room.height() - ph) / 2,
        1,
        ph,
        WALL,
    );
    fill_perimeter_paths(map, room, ri, doors, EMPTY_SP);
    let _boss_position = room.as_rect().center_room();
}

fn paint_thick_pillars(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, WATER);
    let pw = (room.width() - 8) / 2;
    let ph = (room.height() - 8) / 2;
    for &(x, y) in &[
        (room.left + 2, room.top + 2),
        (room.left + 2, room.bottom - 2 - ph),
        (room.right - 2 - pw, room.top + 2),
        (room.right - 2 - pw, room.bottom - 2 - ph),
    ] {
        fill_rect(map, x, y, pw + 1, ph + 1, WALL);
    }
    fill_perimeter_paths(map, room, ri, doors, EMPTY_SP);
    let _boss_position = room.as_rect().center_room();
}

fn paint_rat_king(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY_SP);
    let door_points = door_points(room, ri, doors);
    let Some(&door) = door_points.first() else {
        return;
    };
    let door_cell = map
        .point_to_cell(door.x, door.y)
        .map(|v| v as i32)
        .unwrap_or(-1);
    let width = map.width;
    let mut chest_cells = Vec::new();
    for x in room.left + 1..room.right {
        chest_cells.push((top_cell(map, room, x), door_cell));
        chest_cells.push((bottom_cell(map, room, x), door_cell));
    }
    for y in room.top + 2..room.bottom - 1 {
        chest_cells.push((cell(map, room.left + 1, y), door_cell));
        chest_cells.push((cell(map, room.right - 1, y), door_cell));
    }
    for (pos, door) in chest_cells {
        if pos == door - 1 || pos == door + 1 || pos == door - width || pos == door + width {
            continue;
        }
        let _gold_quantity = Random::int_range_inclusive(10, 25);
    }
    let _king_position = room.random_margin(2);
}

fn top_cell(map: &TerrainMap, room: &Room, x: i32) -> i32 {
    cell(map, x, room.top + 1)
}
fn bottom_cell(map: &TerrainMap, room: &Room, x: i32) -> i32 {
    cell(map, x, room.bottom - 1)
}
fn cell(map: &TerrainMap, x: i32, y: i32) -> i32 {
    (x - map.origin_x) + (y - map.origin_y) * map.width
}

fn paint_cross_water(map: &mut TerrainMap, room: &Room) {
    fill_rect(
        map,
        room.left + room.width() / 2 - 1,
        room.top + room.height() / 2 - 2,
        2 + room.width() % 2,
        4 + room.height() % 2,
        WATER,
    );
    fill_rect(
        map,
        room.left + room.width() / 2 - 2,
        room.top + room.height() / 2 - 1,
        4 + room.width() % 2,
        2 + room.height() % 2,
        WATER,
    );
}

fn reject_later_water(map: &mut TerrainMap, room: &Room) {
    for x in room.left..=room.right {
        for y in room.top..=room.bottom {
            if let Some(i) = map.point_to_cell(x, y) {
                map.water_allowed[i] = false;
            }
        }
    }
}

fn fill_perimeter_paths(map: &mut TerrainMap, room: &Room, ri: usize, doors: &DoorMap, tile: i32) {
    let mut remaining: Vec<Point> = door_points(room, ri, doors)
        .into_iter()
        .map(|p| {
            let (dx, dy) = inward(room, p);
            Point::new(p.x + dx, p.y + dy)
        })
        .collect();
    if remaining.is_empty() {
        return;
    }
    let mut filled = vec![remaining.remove(0)];
    while !remaining.is_empty() {
        let mut best = (i32::MAX, 0, 0);
        for (fi, &from) in filled.iter().enumerate() {
            for (ti, &to) in remaining.iter().enumerate() {
                let distance = perimeter_distance(room, from, to);
                if distance < best.0 {
                    best = (distance, fi, ti);
                }
            }
        }
        let to = remaining.remove(best.2);
        fill_between(map, room, filled[best.1], to, tile);
        filled.push(to);
    }
}

fn perimeter_distance(room: &Room, a: Point, b: Point) -> i32 {
    if ((a.x == room.left + 1 || a.x == room.right - 1) && a.y == b.y)
        || ((a.y == room.top + 1 || a.y == room.bottom - 1) && a.x == b.x)
    {
        return space_between(a.x, b.x).max(space_between(a.y, b.y));
    }
    (space_between(room.left, a.x) + space_between(room.left, b.x))
        .min(space_between(room.right, a.x) + space_between(room.right, b.x))
        + (space_between(room.top, a.y) + space_between(room.top, b.y))
            .min(space_between(room.bottom, a.y) + space_between(room.bottom, b.y))
        - 1
}

fn space_between(a: i32, b: i32) -> i32 {
    (a - b).abs() - 1
}

fn fill_between(map: &mut TerrainMap, room: &Room, from: Point, to: Point, tile: i32) {
    if ((from.x == room.left + 1 || from.x == room.right - 1) && from.x == to.x)
        || ((from.y == room.top + 1 || from.y == room.bottom - 1) && from.y == to.y)
    {
        fill_rect(
            map,
            from.x.min(to.x),
            from.y.min(to.y),
            space_between(from.x, to.x) + 2,
            space_between(from.y, to.y) + 2,
            tile,
        );
        return;
    }
    for corner in [
        Point::new(room.left + 1, room.top + 1),
        Point::new(room.right - 1, room.top + 1),
        Point::new(room.right - 1, room.bottom - 1),
        Point::new(room.left + 1, room.bottom - 1),
    ] {
        if (corner.x == from.x || corner.y == from.y) && (corner.x == to.x || corner.y == to.y) {
            draw_line(map, from, corner, tile);
            draw_line(map, corner, to, tile);
            return;
        }
    }
    let side = if from.y == room.top + 1 || from.y == room.bottom - 1 {
        if space_between(room.left, from.x) + space_between(room.left, to.x)
            <= space_between(room.right, from.x) + space_between(room.right, to.x)
        {
            Point::new(room.left + 1, room.top + room.height() / 2)
        } else {
            Point::new(room.right - 1, room.top + room.height() / 2)
        }
    } else if space_between(room.top, from.y) + space_between(room.top, to.y)
        <= space_between(room.bottom, from.y) + space_between(room.bottom, to.y)
    {
        Point::new(room.left + room.width() / 2, room.top + 1)
    } else {
        Point::new(room.left + room.width() / 2, room.bottom - 1)
    };
    fill_between(map, room, from, side, tile);
    fill_between(map, room, side, to, tile);
}

fn draw_line(map: &mut TerrainMap, from: Point, to: Point, tile: i32) {
    let (dx, dy) = ((to.x - from.x).signum(), (to.y - from.y).signum());
    let mut p = from;
    loop {
        set(map, p.x, p.y, tile);
        if p == to {
            break;
        }
        p.x += dx;
        p.y += dy;
    }
}

fn door_points(room: &Room, ri: usize, doors: &DoorMap) -> Vec<Point> {
    room.connected
        .iter()
        .filter_map(|&ni| doors.get(ri, ni))
        .map(|d| Point::new(d.x, d.y))
        .collect()
}
fn inward(room: &Room, p: Point) -> (i32, i32) {
    if p.x == room.left {
        (1, 0)
    } else if p.y == room.top {
        (0, 1)
    } else if p.x == room.right {
        (-1, 0)
    } else {
        (0, -1)
    }
}
fn draw_inside(map: &mut TerrainMap, room: &Room, p: Point, n: i32, tile: i32) {
    let (dx, dy) = inward(room, p);
    for i in 1..=n {
        set(map, p.x + dx * i, p.y + dy * i, tile);
    }
}
fn fill_room(map: &mut TerrainMap, room: &Room, tile: i32) {
    fill_rect(map, room.left, room.top, room.width(), room.height(), tile);
}
fn fill_margin(map: &mut TerrainMap, room: &Room, m: i32, tile: i32) {
    fill_rect(
        map,
        room.left + m,
        room.top + m,
        room.width() - 2 * m,
        room.height() - 2 * m,
        tile,
    );
}
fn fill_rect(map: &mut TerrainMap, x: i32, y: i32, w: i32, h: i32, tile: i32) {
    for px in x..x + w {
        for py in y..y + h {
            set(map, px, py, tile);
        }
    }
}
fn fill_diamond(map: &mut TerrainMap, room: &Room, m: i32, tile: i32) {
    let (x, y, w, h) = (
        room.left + m,
        room.top + m,
        room.width() - 2 * m,
        room.height() - 2 * m,
    );
    let mut dw = (w - (h - 2 - h % 2)).max(if w % 2 == 0 { 2 } else { 3 });
    for i in 0..=h {
        fill_rect(map, x + (w - dw) / 2, y + i, dw, h - 2 * i, tile);
        dw += 2;
        if dw > w {
            break;
        }
    }
}
fn set(map: &mut TerrainMap, x: i32, y: i32, tile: i32) {
    if let Some(i) = map.point_to_cell(x, y) {
        map.map[i] = tile;
    }
}
fn terrain_at(map: &TerrainMap, x: i32, y: i32) -> Option<i32> {
    map.point_to_cell(x, y).map(|i| map.map[i])
}
