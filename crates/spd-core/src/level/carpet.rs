//! Pinned `tiles.custom.Carpet` stitch and city override indices.

use crate::level::terrain::TerrainMap;

pub(crate) const CLASS: &str = "Carpet";
/// Public city maps use the short texture stem; vault goldens keep the Java asset path.
pub(crate) const TEXTURE: &str = "carpet";
pub(crate) const VAULT_TEXTURE: &str = "environment/custom_tiles/carpet.png";

pub(crate) const SKIP: i16 = -1;
pub(crate) const CITY_STATUE: i16 = 80;
pub(crate) const CITY_PEDESTAL: i16 = 81;
pub(crate) const CITY_ENTRANCE: i16 = 82;
pub(crate) const CITY_STATUE_TR: i16 = 83;
pub(crate) const CITY_STATUE_BR: i16 = 84;
pub(crate) const CITY_STATUE_TL: i16 = 85;
pub(crate) const CITY_STATUE_BL: i16 = 86;
pub(crate) const CITY_PEDESTAL_TR: i16 = 87;
pub(crate) const CITY_PEDESTAL_TL: i16 = 88;

pub(crate) fn stitch(
    tile_w: i32,
    tile_h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
) -> Vec<i16> {
    let region = (16 * ((depth - 1) / 5)) as i16;
    let mut data = vec![0i16; (tile_w * tile_h) as usize];
    let mut i = 0;
    for y in 0..tile_h {
        for x in 0..tile_w {
            data[i] = region;
            if y == 0 {
                data[i] += 1;
            }
            if x == tile_w - 1 {
                data[i] += 2;
            }
            if y == tile_h - 1 {
                data[i] += 4;
            }
            if x == 0 {
                data[i] += 8;
            }
            i += 1;
        }
    }
    for &(x, y, value) in overrides {
        data[(x + tile_w * y) as usize] = value;
    }
    data
}

#[allow(clippy::too_many_arguments)] // Java Carpet.setRect + texture + overrides
pub(crate) fn add(
    map: &mut TerrainMap,
    texture: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
) {
    stamp(map, texture, x, y, w, h, depth, overrides, false);
}

#[allow(clippy::too_many_arguments)] // Java customTiles.add(0, carpet)
pub(crate) fn add_under(
    map: &mut TerrainMap,
    texture: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
) {
    stamp(map, texture, x, y, w, h, depth, overrides, true);
}

#[allow(clippy::too_many_arguments)] // city stamp without repeating the texture stem
pub(crate) fn add_city(
    map: &mut TerrainMap,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
) {
    add(map, TEXTURE, x, y, w, h, depth, overrides);
}

#[allow(clippy::too_many_arguments)] // StatuesEntrance/Exit insert-under variant
pub(crate) fn add_city_under(
    map: &mut TerrainMap,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
) {
    add_under(map, TEXTURE, x, y, w, h, depth, overrides);
}

#[allow(clippy::too_many_arguments)] // mirrors Java Carpet.setRect + override list
fn stamp(
    map: &mut TerrainMap,
    texture: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: i32,
    overrides: &[(i32, i32, i16)],
    under: bool,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let data = stitch(w, h, depth, overrides);
    if under {
        map.record_custom_tile_front(CLASS, texture, (x, y, w, h), data);
    } else {
        map.record_custom_tile(CLASS, texture, (x, y, w, h), data);
    }
}
