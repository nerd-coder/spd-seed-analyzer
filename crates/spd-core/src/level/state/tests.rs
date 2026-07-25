use super::*;
use crate::items::model::{ItemCategory, ItemProvenance, ShopStockRole};
use crate::report::{MapHeap, MapHeapItem, MapMarker, MapMarkerKind, MapMob};

fn item(class_name: &str, source: &str) -> GeneratedItem {
    let mut item = GeneratedItem::new(class_name, ItemCategory::Potion);
    item.source = Some(source.into());
    item
}

#[test]
fn public_projection_omits_runtime_sensitive_main_loot_and_map_cells() {
    let runtime_cells = vec![7, 9];
    let floor = LevelState {
        depth: 3,
        feeling: Feeling::None,
        builder: None,
        rooms: vec![],
        room_bounds: vec![],
        build_ok: true,
        forced_items: vec![item("PotionOfStrength", "forced")],
        placed_items: vec![
            item("PotionOfHealing", "chest:heap"),
            item("PotionOfMindVision", "mimic:mimic"),
            item("PotionOfHaste", "golden_mimic:golden_mimic"),
        ],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        complete: true,
        map: Some(FloorMap {
            width: 4,
            height: 4,
            tileset: "sewers".into(),
            tiles: vec![0; 16],
            tile_variance: vec![0; 16],
            discoverable: vec![true; 16],
            markers: vec![
                MapMarker {
                    cell: 7,
                    kind: MapMarkerKind::Item,
                    label: "Potion of Healing".into(),
                },
                MapMarker {
                    cell: 9,
                    kind: MapMarkerKind::Mob,
                    label: "Mimic".into(),
                },
                MapMarker {
                    cell: 12,
                    kind: MapMarkerKind::Item,
                    label: "Potion of Strength".into(),
                },
                MapMarker {
                    cell: 14,
                    kind: MapMarkerKind::Item,
                    label: "Shop stock".into(),
                },
            ],
            heaps: vec![
                MapHeap {
                    cell: 7,
                    heap_type: "chest".into(),
                    items: vec![MapHeapItem {
                        class_name: "PotionOfHealing".into(),
                        quantity: 1,
                        level: 0,
                        cursed: false,
                    }],
                },
                MapHeap {
                    cell: 12,
                    heap_type: "heap".into(),
                    items: vec![MapHeapItem {
                        class_name: "PotionOfStrength".into(),
                        quantity: 1,
                        level: 0,
                        cursed: false,
                    }],
                },
                MapHeap {
                    cell: 14,
                    heap_type: "for_sale".into(),
                    items: vec![MapHeapItem {
                        class_name: "LeatherArmor".into(),
                        quantity: 1,
                        level: 0,
                        cursed: false,
                    }],
                },
            ],
            mobs: vec![MapMob {
                cell: 9,
                class_name: "Mimic".into(),
            }],
            transitions: vec![],
            traps: vec![],
            plants: vec![],
            blobs: vec![],
            runtime_sensitive_loot_cells: runtime_cells,
        }),
        pre_items_rng_probe: vec![],
        pre_mobs_rng_probe: vec![],
        pre_paint_rng_probe: vec![],
    };

    let report = floor.to_floor_report();
    assert_eq!(report.items.len(), 1);
    assert_eq!(
        report.items[0].class_name.as_deref(),
        Some("PotionOfStrength")
    );
    let map = report.map.expect("map");
    assert_eq!(
        map.heaps.iter().map(|heap| heap.cell).collect::<Vec<_>>(),
        [12]
    );
    assert!(map.mobs.is_empty());
    assert_eq!(
        map.markers
            .iter()
            .map(|marker| marker.cell)
            .collect::<Vec<_>>(),
        [12]
    );
    assert!(map.runtime_sensitive_loot_cells.is_empty());

    let json = serde_json::to_string(&map).expect("serialize public map");
    assert!(!json.contains("PotionOfHealing"));
    assert!(!json.contains("Mimic"));
    assert!(!json.contains("LeatherArmor"));
    assert!(!json.contains("for_sale"));
    assert!(!json.contains("runtime_sensitive_loot_cells"));
}

#[test]
fn main_loot_classification_uses_source_provenance() {
    assert!(is_runtime_sensitive_main_loot(&item("Anything", "heap")));
    assert!(is_runtime_sensitive_main_loot(&item(
        "Anything",
        "locked_chest:heap"
    )));
    assert!(is_runtime_sensitive_main_loot(&item(
        "Anything",
        "mimic:mimic"
    )));
    assert!(!is_runtime_sensitive_main_loot(&item("Mimic", "forced")));
}

#[test]
fn artifact_or_ring_shop_fallback_never_promises_a_level() {
    let mut rare = GeneratedItem::new("RingOfForce", ItemCategory::Ring);
    rare.level = 2;
    rare.provenance = ItemProvenance::Shop(ShopStockRole::DeckRareArtifactOrRing);
    assert_eq!(
        reported_level(&rare, true, Some(ShopStockRole::DeckRareArtifactOrRing)),
        None
    );
    assert_eq!(
        reported_level(&rare, true, Some(ShopStockRole::DeckRareRing)),
        Some(0)
    );
}

#[test]
fn real_shop_public_serialization_redacts_internal_roles_but_keeps_fixed_stock_exact() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    let mut floor_six = None;
    for depth in 1..=6 {
        dungeon.depth = depth;
        floor_six = Some(crate::level::create_level_partial(&mut dungeon));
    }
    let state = floor_six.expect("floor 6 state");
    let internal_shop: Vec<_> = state
        .placed_items
        .iter()
        .filter_map(|item| match item.provenance {
            ItemProvenance::Shop(role) => Some((role, item.class_name.clone())),
            ItemProvenance::None => None,
        })
        .collect();
    assert!(!internal_shop.is_empty());

    let report = state.to_floor_report();
    for (role, concrete_class) in &internal_shop {
        let public_name = match role {
            ShopStockRole::DeckWeapon { .. } => Some("weapon stock"),
            ShopStockRole::DeckMissile { .. } => Some("missile weapon stock"),
            ShopStockRole::ChooseBag => Some("inventory-dependent bag stock"),
            ShopStockRole::DeckRareWand => Some("wand stock"),
            ShopStockRole::DeckRareRing => Some("ring stock"),
            ShopStockRole::DeckRareArtifactOrRing => Some("artifact or ring stock"),
            ShopStockRole::Fixed => None,
        };
        let Some(public_name) = public_name else {
            continue;
        };
        let entry = report
            .items
            .iter()
            .find(|entry| entry.name == public_name)
            .expect("constrained shop entry");
        assert_eq!(entry.prediction, ItemPredictionKind::Constrained);
        assert!(entry.class_name.is_none());
        let entry_json = serde_json::to_string(entry).expect("serialize constrained entry");
        assert!(
            !entry_json.contains(concrete_class),
            "internal {concrete_class} leaked through {public_name}"
        );
    }

    let armor = report
        .items
        .iter()
        .find(|entry| {
            entry.source.as_deref() == Some("ShopRoom")
                && entry.class_name.as_deref() == Some("LeatherArmor")
        })
        .expect("fixed shop armor remains public");
    assert_eq!(armor.prediction, ItemPredictionKind::Exact);
    assert!(serde_json::to_string(armor)
        .expect("serialize fixed entry")
        .contains("LeatherArmor"));

    let map_json = serde_json::to_string(report.map.as_ref().expect("floor-6 map"))
        .expect("serialize public map");
    assert!(!map_json.contains("for_sale"));

    let search = crate::search_seeds(&crate::SeedSearchRequest {
        start_seed: 0,
        candidate_count: 1,
        floors: 6,
        constraints: vec![crate::ItemConstraint {
            class_name: "LeatherArmor".into(),
            min_level: None,
            min_depth: 6,
            max_depth: 6,
        }],
        match_mode: crate::MatchMode::All,
        max_matches: 1,
    })
    .expect("search fixed shop armor");
    assert_eq!(search.matches.len(), 1);
    assert_eq!(search.matches[0].evidence[0].class_name, "LeatherArmor");
}
