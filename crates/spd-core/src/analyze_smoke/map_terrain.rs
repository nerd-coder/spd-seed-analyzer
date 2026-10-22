use super::*;
use crate::level::{FURROWED_GRASS, GRASS, HIGH_GRASS};

#[test]
fn created_loot_flattens_the_tall_grass_under_it() {
    let report = analyze_seed("GVF-RLC-LDK", 1).expect("analyze");
    let map = report.floors[0].map.as_ref().expect("floor-one map");
    for cell in [1014, 1053] {
        assert!(map.heaps.iter().any(|heap| heap.cell == cell));
        assert_eq!(i32::from(map.tiles[cell as usize]), GRASS, "cell {cell}");
    }
    for heap in &map.heaps {
        let tile = i32::from(map.tiles[heap.cell as usize]);
        assert!(
            !matches!(tile, HIGH_GRASS | FURROWED_GRASS),
            "heap {} on tall grass",
            heap.cell
        );
    }
}
