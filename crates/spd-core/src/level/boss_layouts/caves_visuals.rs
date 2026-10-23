//! Pinned `CavesBossLevel` custom tilemaps, as created before the fight.

use crate::level::terrain;
use crate::report::MapCustomTile;

const WIDTH: usize = 33;
const TEXTURE: &str = "caves_boss";
/// `CityEntrance` / `EntranceOverhang` rect: `(0, 0, width, 11)`.
const ENTRY_HEIGHT: usize = 11;
/// The 5-wide entryway column starts at `tileW/2 - 2`.
const ENTRY_LEFT: usize = WIDTH / 2 - 2;
/// `ArenaVisuals.setRect(0, 12, width, 27)`.
const ARENA_TOP: usize = 12;
const ARENA_HEIGHT: usize = 27;

#[rustfmt::skip]
const CITY_ENTRANCE: [i16; 55] = [
    -1,  7,  7,  7, -1,
    -1,  1,  2,  3, -1,
     8,  1,  2,  3, 12,
    16,  9, 10, 11, 20,
    16, 16, 22, 20, 20,
    16, 17, 18, 19, 20,
    16, 16, 18, 20, 20,
    16, 17, 18, 19, 20,
    16, 16, 18, 20, 20,
    16, 17, 18, 19, 20,
    24, 25, 26, 27, 28,
];

#[rustfmt::skip]
const ENTRANCE_TERRAIN: [i16; 55] = [
    -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1,
     8, -1, -1, -1, 12,
    30, -1, -1, -1, 31,
    -1, -1, -1, -1, -1,
    -1, 17, -1, 19, -1,
    -1, -1, -1, -1, -1,
    -1, 17, -1, 19, -1,
    -1, -1, -1, -1, -1,
    -1, 17, -1, 19, -1,
    -1, -1, -1, -1, -1,
];

#[rustfmt::skip]
const ENTRANCE_OVERHANG: [i16; 55] = [
     0,  7,  7,  7,  4,
     0, 15, 15, 15,  4,
    -1, 23, 23, 23, -1,
    -1, -1, -1, -1, -1,
    -1,  6, -1, 14, -1,
    -1, -1, -1, -1, -1,
    -1,  6, -1, 14, -1,
    -1, -1, -1, -1, -1,
    -1,  6, -1, 14, -1,
    -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1,
];

/// `(custom_tiles, custom_terrain, custom_walls)` in Java insertion order.
pub(super) fn layers(
    map: &[u16],
    pylons: &[(i32, i32)],
) -> (Vec<MapCustomTile>, Vec<MapCustomTile>, Vec<MapCustomTile>) {
    (
        vec![
            layer("CityEntrance", 0, 0, WIDTH, ENTRY_HEIGHT, city_entrance()),
            layer(
                "ArenaVisuals",
                0,
                ARENA_TOP,
                WIDTH,
                ARENA_HEIGHT,
                arena(map, pylons),
            ),
        ],
        vec![layer(
            "EntranceTerrain",
            ENTRY_LEFT,
            0,
            5,
            ENTRY_HEIGHT,
            ENTRANCE_TERRAIN.to_vec(),
        )],
        vec![layer(
            "EntranceOverhang",
            0,
            0,
            WIDTH,
            ENTRY_HEIGHT,
            with_entryway(&ENTRANCE_OVERHANG, |_, _| -1),
        )],
    )
}

fn layer(
    class_name: &str,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    static_data: Vec<i16>,
) -> MapCustomTile {
    MapCustomTile {
        class_name: class_name.into(),
        texture: TEXTURE.into(),
        x: x as u32,
        y: y as u32,
        width: width as u32,
        height: height as u32,
        static_data,
    }
}

/// Copies `entryway` into the centre columns and fills the rest per cell.
fn with_entryway(entryway: &[i16; 55], other: impl Fn(usize, usize) -> i16) -> Vec<i16> {
    let mut data = vec![0; WIDTH * ENTRY_HEIGHT];
    for (i, value) in data.iter_mut().enumerate() {
        let (x, y) = (i % WIDTH, i / WIDTH);
        *value = if (ENTRY_LEFT..ENTRY_LEFT + 5).contains(&x) {
            entryway[y * 5 + x - ENTRY_LEFT]
        } else {
            other(x, y)
        };
    }
    data
}

/// `CityEntrance.create`: rows 2 and 3 cover the walls beside the stairs,
/// except the metal structures in columns 9 and 23.
fn city_entrance() -> Vec<i16> {
    with_entryway(&CITY_ENTRANCE, |x, y| match y {
        2 => 13,
        3 if x == 9 || x == 23 => -1,
        3 => 21,
        _ => -1,
    })
}

/// `ArenaVisuals.updateState` while unlocked: pylon pads, wires, and gate.
fn arena(map: &[u16], pylons: &[(i32, i32)]) -> Vec<i16> {
    let mut data = vec![0i16; WIDTH * ARENA_HEIGHT];
    let mut j = WIDTH * ARENA_TOP;
    let mut i = 0;
    while i < data.len() {
        let (x, y) = ((j % WIDTH) as i32, (j / WIDTH) as i32);
        let tile = i32::from(map[j]);
        if tile == terrain::EMPTY_SP {
            for &(px, py) in pylons {
                if (px, py) == (x, y) {
                    data[i] = -1;
                } else if (px - x).abs().max((py - y).abs()) == 1 {
                    data[i] = (54 + (x + 8 * y) - (px + 8 * py)) as i16;
                }
            }
        } else if tile == terrain::INACTIVE_TRAP {
            data[i] = 37;
        } else if (14..19).contains(&x) && y == 13 {
            let mut visual = if terrain::is_solid_tile(tile) { 40 } else { 32 };
            for _ in 0..4 {
                data[i] = visual;
                i += 1;
                visual += 1;
            }
            data[i] = visual;
            j += 4;
        } else {
            data[i] = -1;
        }
        j += 1;
        i += 1;
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn city_entrance_keeps_metal_columns_clear() {
        let data = city_entrance();
        assert_eq!(data[2 * WIDTH], 13);
        assert_eq!(data[3 * WIDTH + 9], -1);
        assert_eq!(data[3 * WIDTH + 10], 21);
        assert_eq!(&data[ENTRY_LEFT..ENTRY_LEFT + 5], &[-1, 7, 7, 7, -1]);
    }
}
