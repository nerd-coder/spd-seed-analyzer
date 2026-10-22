//! Boss-floor heaps and mobs against the Java oracle's final level state.

use serde::Deserialize;

use crate::report::FloorMap;

#[derive(Deserialize)]
struct Fixture {
    floors: Vec<OracleFloor>,
}

#[derive(Deserialize)]
struct OracleFloor {
    depth: i32,
    terrain: Vec<u16>,
    final_mobs: Vec<OracleMob>,
    final_heaps: Vec<OracleHeap>,
}

#[derive(Deserialize)]
struct OracleMob {
    cell: u32,
    #[serde(rename = "class")]
    class_name: String,
}

#[derive(Deserialize)]
struct OracleHeap {
    cell: u32,
    heap_type: String,
    items: Vec<OracleItem>,
}

#[derive(Deserialize)]
struct OracleItem {
    #[serde(rename = "class")]
    class_name: String,
    quantity: i32,
}

type Mobs = Vec<(u32, String)>;
type Heaps = Vec<(u32, String, Vec<(String, i32)>)>;

fn entities(map: &FloorMap) -> (Mobs, Heaps) {
    let mut mobs: Mobs = map
        .mobs
        .iter()
        .map(|mob| (mob.cell, mob.class_name().to_string()))
        .collect();
    let mut heaps: Heaps = map
        .heaps
        .iter()
        .map(|heap| {
            let items = heap
                .items
                .iter()
                .map(|item| (item.class_name().to_string(), item.quantity))
                .collect();
            (heap.cell, heap.heap_type.clone(), items)
        })
        .collect();
    mobs.sort();
    heaps.sort();
    (mobs, heaps)
}

fn oracle_entities(floor: &OracleFloor) -> (Mobs, Heaps) {
    let mut mobs: Mobs = floor
        .final_mobs
        .iter()
        .map(|mob| (mob.cell, mob.class_name.clone()))
        .collect();
    let mut heaps: Heaps = floor
        .final_heaps
        .iter()
        .map(|heap| {
            let items = heap
                .items
                .iter()
                .map(|item| (item.class_name.clone(), item.quantity))
                .collect();
            (heap.cell, heap.heap_type.clone(), items)
        })
        .collect();
    mobs.sort();
    heaps.sort();
    (mobs, heaps)
}

fn public_boss_map(seed: &str, depth: i32) -> FloorMap {
    let report = crate::analyze_seed_with_profile(
        seed,
        depth.try_into().unwrap(),
        &crate::MapProfile::default(),
    )
    .unwrap();
    report.floors[depth as usize - 1]
        .map
        .clone()
        .unwrap_or_else(|| panic!("{seed} depth {depth} map"))
}

macro_rules! fixture {
    ($seed:literal, $file:literal) => {
        (
            $seed,
            include_str!(concat!("../../../../../tools/java-oracle/fixtures/", $file)),
        )
    };
}

#[test]
fn boss_entities_match_java_oracles() {
    for (seed, json) in [
        fixture!("AAA-AAA-AAA", "aaa-aaa-aaa-final-heaps-floor-5.json"),
        fixture!("ABC-DEF-GHI", "abc-def-ghi-final-heaps-floor-5.json"),
        fixture!("GFX-PZH-DCH", "gfx-pzh-dch-final-heaps-floor-5.json"),
        fixture!("HKT-JZN-XQQ", "hkt-jzn-xqq-final-heaps-floor-5.json"),
        fixture!("ZZZ-ZZZ-ZZZ", "zzz-zzz-zzz-final-heaps-floor-5.json"),
        fixture!("AAA-AAA-AAA", "aaa-aaa-aaa-final-heaps-floor-10.json"),
        fixture!("GFX-PZH-DCH", "gfx-pzh-dch-final-heaps-floor-10.json"),
        fixture!("AAA-AAA-AAA", "aaa-aaa-aaa-final-heaps-floor-15.json"),
        fixture!("ABC-DEF-GHI", "abc-def-ghi-final-heaps-floor-15.json"),
        fixture!("GFX-PZH-DCH", "gfx-pzh-dch-final-heaps-floor-15.json"),
        fixture!("ZZZ-ZZZ-ZZZ", "zzz-zzz-zzz-final-heaps-floor-15.json"),
        fixture!("AAA-AAA-AAA", "aaa-aaa-aaa-final-heaps-floor-26.json"),
        fixture!("ABC-DEF-GHI", "abc-def-ghi-final-heaps-floor-26.json"),
        fixture!("GFX-PZH-DCH", "gfx-pzh-dch-final-heaps-floor-26.json"),
    ] {
        let oracle = serde_json::from_str::<Fixture>(json)
            .unwrap()
            .floors
            .remove(0);
        let actual = public_boss_map(seed, oracle.depth);
        assert_eq!(
            entities(&actual),
            oracle_entities(&oracle),
            "{seed} depth {}",
            oracle.depth
        );
        assert!(
            actual.uncertain_cells.is_empty(),
            "{seed} depth {} uncertain {:?}",
            oracle.depth,
            actual.uncertain_cells
        );
        if oracle.depth == 10 {
            assert_eq!(actual.tiles, oracle.terrain, "{seed} cages and torches");
        }
    }
}
