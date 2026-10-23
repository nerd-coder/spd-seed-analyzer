//! Pinned `GooBossRoom.GooNest` and `SewerBossExitRoom` custom tilemaps.

use crate::geom::Point;
use crate::level::terrain::TerrainMap;
use crate::rooms::room::Room;

const SEWER_BOSS_TEX: &str = "sewer_boss";

/// `GooBossRoom.setupGooNest`.
pub(super) fn goo_nest(map: &mut TerrainMap, room: &Room) {
    let width = 4 + room.width() % 2;
    let height = 4 + room.height() % 2;
    let rect = (
        room.left + room.width() / 2 - 2,
        room.top + room.height() / 2 - 2,
        width,
        height,
    );
    map.record_custom_tile(
        "GooNest",
        SEWER_BOSS_TEX,
        rect,
        goo_nest_data(width, height),
    );
}

fn goo_nest_data(width: i32, height: i32) -> Vec<i16> {
    let mut data = vec![0; (width * height) as usize];
    let (right, bottom) = (width - 1, height - 1);
    for y in 0..height {
        for x in 0..width {
            data[(x + width * y) as usize] = if (x == 0 || x == right) && (y == 0 || y == bottom) {
                -1
            } else if (x, y) == (1, 0) || (x, y) == (0, 1) {
                0
            } else if (x, y) == (right - 1, 0) || (x, y) == (right, 1) {
                1
            } else if (x, y) == (1, bottom) || (x, y) == (0, bottom - 1) {
                2
            } else if (x, y) == (right - 1, bottom) || (x, y) == (right, bottom - 1) {
                3
            } else if x == 0 {
                4
            } else if y == 0 {
                5
            } else if x == right {
                6
            } else if y == bottom {
                7
            } else {
                8
            };
        }
    }
    data
}

/// `SewerBossExitRoom.paint`: `SewerExit` below and `SewerExitOverhang` above.
pub(super) fn sewer_exit(map: &mut TerrainMap, center: Point) {
    map.record_custom_tile(
        "SewerExit",
        SEWER_BOSS_TEX,
        (center.x - 1, center.y, 3, 3),
        vec![21, -1, 22, 23, 23, 23, 24, 24, 24],
    );
    map.record_custom_wall(
        "SewerExitOverhang",
        SEWER_BOSS_TEX,
        (center.x - 1, center.y - 2, 3, 2),
        vec![16, 17, 18, 19, -1, 20],
    );
}

#[cfg(test)]
mod tests {
    use super::goo_nest_data;

    #[test]
    fn goo_nest_matches_java_odd_room_layout() {
        assert_eq!(
            goo_nest_data(5, 4),
            vec![-1, 0, 5, 1, -1, 0, 8, 8, 8, 1, 2, 8, 8, 8, 3, -1, 2, 7, 3, -1]
        );
    }
}
