use super::*;
use crate::report::MapCustomTile;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    floors: Vec<OracleFloor>,
}
#[derive(Deserialize)]
struct OracleFloor {
    depth: i32,
    width: u32,
    height: u32,
    terrain: Vec<u16>,
    discoverable: Vec<bool>,
    tile_variance: Vec<u8>,
    #[serde(default)]
    custom_tiles: Vec<OracleCustomTile>,
    #[serde(default)]
    custom_terrain: Vec<OracleCustomTile>,
    #[serde(default)]
    custom_walls: Vec<OracleCustomTile>,
    transitions: Vec<MapTransition>,
    #[serde(default)]
    pre_items_rng: Vec<i32>,
}

#[derive(Deserialize)]
struct OracleCustomTile {
    #[serde(rename = "class")]
    class_name: String,
    #[serde(default)]
    texture: Option<String>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    static_data: Vec<i16>,
}

impl From<&OracleCustomTile> for MapCustomTile {
    fn from(tile: &OracleCustomTile) -> Self {
        Self {
            class_name: tile.class_name.clone(),
            texture: tile
                .texture
                .clone()
                .unwrap_or_else(|| "halls_special".into()),
            x: tile.x,
            y: tile.y,
            width: tile.width,
            height: tile.height,
            static_data: tile.static_data.clone(),
        }
    }
}

trait LayerRect {
    fn class_name(&self) -> &str;
    fn rect(&self) -> (u32, u32, u32, u32);
}

impl LayerRect for MapCustomTile {
    fn class_name(&self) -> &str {
        &self.class_name
    }
    fn rect(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl LayerRect for OracleCustomTile {
    fn class_name(&self) -> &str {
        &self.class_name
    }
    fn rect(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }
}

fn rat_king_layer_rects<T: LayerRect>(tiles: &[T]) -> Vec<(String, u32, u32, u32, u32)> {
    let mut out: Vec<_> = tiles
        .iter()
        .filter(|tile| {
            matches!(
                tile.class_name(),
                "Carpet" | "RatKingRoomDeco" | "RatKingStatues" | "StatueOverhang"
            )
        })
        .map(|tile| {
            let (x, y, width, height) = tile.rect();
            (tile.class_name().to_string(), x, y, width, height)
        })
        .collect();
    out.sort();
    out
}

#[test]
fn fixed_structures_match_normalized_java_oracles() {
    for fixture in [
        include_str!(
            "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-10.json"
        ),
        include_str!(
            "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-20.json"
        ),
        include_str!(
            "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-26.json"
        ),
    ] {
        let mut oracle: OracleFloor = serde_json::from_str::<Fixture>(fixture)
            .unwrap()
            .floors
            .remove(0);
        for tile in &mut oracle.terrain {
            *tile = match i32::from(*tile) {
                terrain::EMPTY_DECO | terrain::REGION_DECO | terrain::REGION_DECO_ALT => {
                    terrain::EMPTY as u16
                }
                terrain::WALL_DECO => terrain::WALL as u16,
                tile => tile as u16,
            };
        }
        let actual = fixed_layout(oracle.depth).unwrap();
        assert_eq!((actual.width, actual.height), (oracle.width, oracle.height));
        assert_eq!(actual.tiles, oracle.terrain);
        assert_eq!(actual.discoverable, oracle.discoverable);
        assert_eq!(actual.transitions, oracle.transitions);
        assert!(actual.markers.is_empty() && actual.heaps.is_empty() && actual.mobs.is_empty());
    }
}

#[test]
fn sewer_boss_structures_match_raw_java_oracles() {
    for (seed, json) in [
        (
            "AAA-AAA-AAA",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-5.json"
            ),
        ),
        (
            "ABC-DEF-GHI",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-5.json"
            ),
        ),
        (
            "GFX-PZH-DCH",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-5.json"
            ),
        ),
        (
            "HKT-JZN-XQQ",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/hkt-jzn-xqq-final-heaps-floor-5.json"
            ),
        ),
        (
            "ZZZ-ZZZ-ZZZ",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-5.json"
            ),
        ),
    ] {
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        let oracle = &fixture.floors[0];
        let report =
            crate::analyze_seed_with_profile(seed, 5, &crate::MapProfile::default()).unwrap();
        let actual = report.floors[4].map.as_ref().expect("depth-5 layout");
        assert_eq!(
            (actual.width, actual.height),
            (oracle.width, oracle.height),
            "{seed}"
        );
        assert_eq!(
            sewer::last_pre_items_rng(),
            oracle.pre_items_rng,
            "{seed} pre-items RNG"
        );
        if actual.tiles != oracle.terrain {
            let diffs = actual
                .tiles
                .iter()
                .zip(&oracle.terrain)
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .take(20)
                .map(|(cell, (a, b))| {
                    (
                        cell,
                        cell % actual.width as usize,
                        cell / actual.width as usize,
                        *a,
                        *b,
                    )
                })
                .collect::<Vec<_>>();
            panic!("{seed} terrain first diffs: {diffs:?}");
        }
        assert_eq!(
            actual.discoverable, oracle.discoverable,
            "{seed} discoverable"
        );
        assert_eq!(actual.transitions, oracle.transitions, "{seed} transitions");
        assert_eq!(
            rat_king_layer_rects(&actual.custom_tiles),
            rat_king_layer_rects(&oracle.custom_tiles),
            "{seed} RatKing custom tiles"
        );
        assert_eq!(
            rat_king_layer_rects(&actual.custom_terrain),
            rat_king_layer_rects(&oracle.custom_terrain),
            "{seed} RatKing custom terrain"
        );
        assert_eq!(
            rat_king_layer_rects(&actual.custom_walls),
            rat_king_layer_rects(&oracle.custom_walls),
            "{seed} RatKing custom walls"
        );
    }
}

#[test]
fn caves_boss_structures_match_raw_java_oracles() {
    for (seed, json) in [
        (
            "AAA-AAA-AAA",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-15.json"
            ),
        ),
        (
            "ABC-DEF-GHI",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-15.json"
            ),
        ),
        (
            "GFX-PZH-DCH",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-15.json"
            ),
        ),
        (
            "ZZZ-ZZZ-ZZZ",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-15.json"
            ),
        ),
    ] {
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        let oracle = &fixture.floors[0];
        let report =
            crate::analyze_seed_with_profile(seed, 15, &crate::MapProfile::default()).unwrap();
        let actual = report.floors[14].map.as_ref().expect("depth-15 layout");
        assert_eq!(
            (actual.width, actual.height),
            (oracle.width, oracle.height),
            "{seed}"
        );
        if actual.tiles != oracle.terrain {
            let width = actual.width as usize;
            let diffs = actual
                .tiles
                .iter()
                .zip(&oracle.terrain)
                .enumerate()
                .filter(|(_, (actual, oracle))| actual != oracle)
                .take(20)
                .map(|(cell, (actual, oracle))| {
                    (cell, cell % width, cell / width, *actual, *oracle)
                })
                .collect::<Vec<_>>();
            panic!("{seed} terrain first diffs: {diffs:?}");
        }
        assert_eq!(
            caves::last_pre_items_rng(),
            oracle.pre_items_rng,
            "{seed} pre-items RNG"
        );
        assert_eq!(
            actual.discoverable, oracle.discoverable,
            "{seed} discoverable"
        );
        assert_eq!(actual.transitions, oracle.transitions, "{seed} transitions");
    }
}

#[test]
fn halls_boss_structures_match_exact_java_oracles() {
    for (seed, json) in [
        (
            "AAA-AAA-AAA",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-25.json"
            ),
        ),
        (
            "ABC-DEF-GHI",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-25.json"
            ),
        ),
        (
            "GFX-PZH-DCH",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/gfx-pzh-dch-final-heaps-floor-25.json"
            ),
        ),
        (
            "ZZZ-ZZZ-ZZZ",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-25.json"
            ),
        ),
    ] {
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        let oracle = &fixture.floors[0];
        let report =
            crate::analyze_seed_with_profile(seed, 25, &crate::MapProfile::default()).unwrap();
        let actual = report.floors[24].map.as_ref().expect("depth-25 layout");
        assert_eq!(
            (actual.width, actual.height),
            (oracle.width, oracle.height),
            "{seed}"
        );
        let exit = actual
            .transitions
            .iter()
            .find(|transition| transition.transition_type == "REGULAR_EXIT")
            .expect("halls exit")
            .cell as usize;
        assert_eq!(
            actual.tiles[exit],
            terrain::EXIT as u16,
            "{seed} opened exit"
        );
        let mut sealed = actual.tiles.clone();
        sealed[exit] = terrain::WALL_DECO as u16;
        assert_eq!(sealed, oracle.terrain, "{seed} raw terrain");
        assert_eq!(
            actual.tile_variance, oracle.tile_variance,
            "{seed} tile variance"
        );
        assert_eq!(
            halls::last_pre_items_rng(),
            oracle.pre_items_rng,
            "{seed} pre-items RNG"
        );
        assert_eq!(
            actual.discoverable, oracle.discoverable,
            "{seed} discoverable"
        );
        assert_eq!(actual.transitions, oracle.transitions, "{seed} transitions");
        assert!(actual.markers.is_empty() && actual.heaps.is_empty() && actual.mobs.is_empty());
    }
}

#[test]
fn halls_boss_custom_layers_match_java_for_preserved_runs() {
    for (seed, json) in [
        (
            "AAA-AAA-AAA",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-25.json"
            ),
        ),
        (
            "ABC-DEF-GHI",
            include_str!(
                "../../../../../tools/java-oracle/fixtures/abc-def-ghi-final-heaps-floor-25.json"
            ),
        ),
    ] {
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        let oracle = &fixture.floors[0];
        let report =
            crate::analyze_seed_with_profile(seed, 25, &crate::MapProfile::default()).unwrap();
        let actual = report.floors[24].map.as_ref().expect("depth-25 layout");
        let mut tiles: Vec<_> = oracle
            .custom_tiles
            .iter()
            .map(MapCustomTile::from)
            .collect();
        let mut walls: Vec<_> = oracle
            .custom_walls
            .iter()
            .map(MapCustomTile::from)
            .collect();
        halls::open_center_piece(&mut tiles, &mut walls);
        assert_eq!(actual.custom_tiles, tiles, "{seed} custom tiles");
        assert_eq!(actual.custom_walls, walls, "{seed} custom walls");
    }
}

fn layer_rects<T: LayerRect>(tiles: &[T]) -> Vec<(String, u32, u32, u32, u32)> {
    let mut out: Vec<_> = tiles
        .iter()
        .map(|tile| {
            let (x, y, width, height) = tile.rect();
            (tile.class_name().to_string(), x, y, width, height)
        })
        .collect();
    out.sort();
    out
}

#[test]
fn boss_custom_layer_rects_match_java_oracles() {
    let fixtures = [
        (
            "AAA-AAA-AAA",
            5,
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-5.json"
            ),
        ),
        (
            "HKT-JZN-XQQ",
            5,
            include_str!(
                "../../../../../tools/java-oracle/fixtures/hkt-jzn-xqq-final-heaps-floor-5.json"
            ),
        ),
        (
            "AAA-AAA-AAA",
            15,
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-15.json"
            ),
        ),
        (
            "ZZZ-ZZZ-ZZZ",
            15,
            include_str!(
                "../../../../../tools/java-oracle/fixtures/zzz-zzz-zzz-final-heaps-floor-15.json"
            ),
        ),
        (
            "AAA-AAA-AAA",
            26,
            include_str!(
                "../../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-final-heaps-floor-26.json"
            ),
        ),
    ];
    for (seed, depth, json) in fixtures {
        let oracle = serde_json::from_str::<Fixture>(json)
            .unwrap()
            .floors
            .remove(0);
        let report =
            crate::analyze_seed_with_profile(seed, depth, &crate::MapProfile::default()).unwrap();
        let actual = report.floors[depth as usize - 1].map.as_ref().unwrap();
        for (kind, actual, oracle) in [
            ("tiles", &actual.custom_tiles, &oracle.custom_tiles),
            ("terrain", &actual.custom_terrain, &oracle.custom_terrain),
            ("walls", &actual.custom_walls, &oracle.custom_walls),
        ] {
            assert_eq!(
                layer_rects(actual),
                layer_rects(oracle),
                "{seed} {depth} {kind}"
            );
            for layer in actual {
                assert_eq!(
                    layer.static_data.len(),
                    (layer.width * layer.height) as usize,
                    "{seed} {depth} {}",
                    layer.class_name
                );
            }
        }
    }
}
