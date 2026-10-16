//! Compact painter-complete floor layout.

use serde::Serialize;

use crate::report::FloorMap;

#[derive(Debug, Clone, Serialize)]
pub struct ExactMap {
    pub width: u32,
    pub height: u32,
    pub tileset: String,
    /// Row-major SPD `Terrain` ids, run-length encoded as `id` or `id*count`,
    /// comma separated.
    pub tiles: String,
    /// `[cell, trap class, visible]`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub traps: Vec<(u32, &'static str, bool)>,
    /// `[cell, plant class]`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plants: Vec<(u32, &'static str)>,
    /// `[cell, transition type, destination depth]`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transitions: Vec<(u32, String, i32)>,
}

impl ExactMap {
    pub fn from_floor_map(map: &FloorMap) -> Self {
        Self {
            width: map.width,
            height: map.height,
            tileset: map.tileset.clone(),
            tiles: encode_rle(&map.tiles),
            traps: map
                .traps
                .iter()
                .map(|trap| (trap.cell, trap.class_name(), trap.visible))
                .collect(),
            plants: map
                .plants
                .iter()
                .map(|plant| (plant.cell, plant.class_name()))
                .collect(),
            transitions: map
                .transitions
                .iter()
                .map(|t| (t.cell, t.transition_type.clone(), t.dest_depth))
                .collect(),
        }
    }
}

pub fn encode_rle(tiles: &[u16]) -> String {
    let mut out = String::new();
    let mut iter = tiles.iter().peekable();
    while let Some(&tile) = iter.next() {
        let mut count = 1;
        while iter.next_if_eq(&&tile).is_some() {
            count += 1;
        }
        if !out.is_empty() {
            out.push(',');
        }
        out.push_str(&tile.to_string());
        if count > 1 {
            out.push('*');
            out.push_str(&count.to_string());
        }
    }
    out
}

pub fn decode_rle(encoded: &str) -> Vec<u16> {
    encoded
        .split(',')
        .filter(|run| !run.is_empty())
        .flat_map(|run| {
            let (tile, count) = run.split_once('*').unwrap_or((run, "1"));
            let tile: u16 = tile.parse().expect("terrain id");
            let count: usize = count.parse().expect("run length");
            std::iter::repeat_n(tile, count)
        })
        .collect()
}
