//! Pinned `CityBossLevel` custom ground and wall tilemaps.

use crate::level::terrain;
use crate::report::MapCustomTile;

const WIDTH: usize = 15;
const HEIGHT: usize = 48;

pub(super) fn layers(map: &[u16]) -> (Vec<MapCustomTile>, Vec<MapCustomTile>) {
    (
        vec![layer("CustomGroundVisuals", ground(map))],
        vec![layer("CustomWallVisuals", walls(map))],
    )
}

fn layer(class_name: &str, static_data: Vec<i16>) -> MapCustomTile {
    MapCustomTile {
        class_name: class_name.into(),
        texture: "city_boss".into(),
        x: 0,
        y: 0,
        width: WIDTH as u32,
        height: HEIGHT as u32,
        static_data,
    }
}

fn ground(map: &[u16]) -> Vec<i16> {
    let mut data = vec![0; WIDTH * HEIGHT];
    let mut stairs_top = None;
    let mut i = WIDTH;
    while i < WIDTH * 22 {
        if map[i] == terrain::EXIT as u16 && stairs_top.is_none() {
            stairs_top = Some(i);
        }
        if map[i] == terrain::WALL as u16 && map[i - WIDTH] == terrain::CHASM as u16 {
            data[i] = 110;
            i += 1;
            data[i] = 111;
        } else if map[i] == terrain::WALL as u16 && map[i - WIDTH] == terrain::WALL as u16 {
            data[i] = 118;
            i += 1;
            data[i] = 119;
        } else if i > WIDTH
            && map[i] == terrain::CHASM as u16
            && map[i - WIDTH] == terrain::WALL as u16
        {
            data[i] = 126;
            i += 1;
            data[i] = 127;
        } else if map[i] == terrain::PEDESTAL as u16 {
            data[i] = 101;
        } else if map[i] == terrain::STATUE as u16 {
            data[i] = 125;
        } else if matches!(
            map[i] as i32,
            terrain::EMPTY
                | terrain::EMPTY_DECO
                | terrain::EMBERS
                | terrain::GRASS
                | terrain::HIGH_GRASS
                | terrain::FURROWED_GRASS
        ) {
            if i / WIDTH == 21 {
                for visual in 88..=94 {
                    data[i] = visual;
                    if visual != 94 {
                        i += 1;
                    }
                }
            } else if map[i - 1] == terrain::CHASM as u16 {
                data[i] = 97;
            } else if map[i + 1] == terrain::CHASM as u16 {
                data[i] = 99;
            } else if map[i] == terrain::EMPTY_DECO as u16 {
                data[i] = 100;
            } else {
                data[i] = 98;
            }
        } else {
            data[i] = -1;
        }
        i += 1;
    }
    let mut stairs_top = stairs_top.expect("city boss exit");
    for row in 0..7 {
        for column in 0..7 {
            data[stairs_top + column] = ((row + 4) * 8 + column) as i16;
        }
        stairs_top += WIDTH;
    }
    i = WIDTH * 22;
    while i < WIDTH * HEIGHT {
        if map[i] == terrain::PEDESTAL as u16 {
            data[i] = 108;
        } else if map[i] == terrain::STATUE as u16 && i % WIDTH > 7 {
            data[i] = 124;
        } else if map[i] == terrain::EMPTY_SP as u16 {
            if map[i + 1] == terrain::EMPTY_SP as u16 && map[i + WIDTH] == terrain::EMPTY_SP as u16
            {
                data[i] = 105;
                data[i + 1] = 106;
                data[i + 2] = 107;
                i += 2;
            } else if map[i + 1] == terrain::CUSTOM_DECO as u16 {
                data[i] = 113;
                data[i + 1] = 114;
                data[i + 2] = 115;
                i += 2;
            } else if map[i + 1] == terrain::EMPTY_SP as u16
                && map[i - WIDTH] == terrain::EMPTY_SP as u16
            {
                data[i] = 121;
                data[i + 1] = 122;
                data[i + 2] = 123;
                i += 2;
            } else if map[i - WIDTH] != terrain::EMPTY_SP as u16 {
                data[i] = 104;
            } else if map[i + WIDTH] != terrain::EMPTY_SP as u16 {
                data[i] = 120;
            } else {
                data[i] = 112;
            }
        } else {
            data[i] = -1;
        }
        i += 1;
    }
    data
}

fn walls(map: &[u16]) -> Vec<i16> {
    let mut data = vec![0; WIDTH * HEIGHT];
    let mut shadow_top = None;
    let mut i = WIDTH;
    while i < WIDTH * 21 {
        if map[i] == terrain::EXIT as u16 && shadow_top.is_none() {
            shadow_top = Some(i - WIDTH * 4);
        }
        if map[i] == terrain::CHASM as u16 && map[i + WIDTH] == terrain::WALL as u16 {
            data[i] = 102;
            i += 1;
            data[i] = 103;
        } else if map[i] == terrain::WALL as u16 && map[i - WIDTH] == terrain::CHASM as u16 {
            data[i] = 110;
            i += 1;
            data[i] = 111;
        } else if map[i + WIDTH] == terrain::STATUE as u16 {
            data[i] = 117;
        } else {
            data[i] = -1;
        }
        i += 1;
    }
    let mut shadow_top = shadow_top.expect("city boss exit shadow");
    for row in 0..8 {
        let (left, middle, right) = if row < 4 {
            (row * 8, row * 8 + 1, row * 8 + 2)
        } else {
            let row = row - 4;
            (row * 8 + 3, row * 8 + 4, row * 8 + 5)
        };
        data[shadow_top] = left as i16;
        data[shadow_top + 1..shadow_top + 7].fill(middle as i16);
        data[shadow_top + 7] = right as i16;
        shadow_top += WIDTH;
    }
    for i in WIDTH * 21..WIDTH * HEIGHT {
        if map[i] == terrain::STATUE as u16 && i % WIDTH > 7 {
            data[i - WIDTH] = 116;
        } else if map[i] == terrain::CUSTOM_DECO as u16 {
            data[i - WIDTH] = 109;
        }
        data[i] = -1;
    }
    data
}
