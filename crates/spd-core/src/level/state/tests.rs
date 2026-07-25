use super::*;
use crate::items::model::{ItemCategory, ItemProvenance, RoomLootRole, ShopStockRole};
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
        public_forced_items: vec![item("PotionOfStrength", "forced")],
        placed_items: vec![
            item("PotionOfHealing", "chest:heap"),
            item("PotionOfMindVision", "mimic:mimic"),
            item("PotionOfHaste", "golden_mimic:golden_mimic"),
        ],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        quest_public_labels: vec![],
        runtime_sensitive_map: false,
        room_public_facts: vec![],
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
            constrained_equipment_cells: vec![],
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
    assert!(!json.contains("constrained_equipment_cells"));

    let mut consumed_internally = floor.clone();
    consumed_internally.forced_items.clear();
    let guarded = consumed_internally.to_floor_report();
    assert!(guarded
        .items
        .iter()
        .any(|item| item.class_name.as_deref() == Some("PotionOfStrength")));
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
            ItemProvenance::None | ItemProvenance::Quest(_) | ItemProvenance::Room(_) => None,
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

    if let Some(map) = &report.map {
        let map_json = serde_json::to_string(map).expect("serialize public map");
        assert!(!map_json.contains("for_sale"));
    }

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

#[test]
fn room_reward_projection_hides_all_concrete_fields_and_deduplicates_counts() {
    let mut floor = LevelState {
        depth: 7,
        feeling: Feeling::None,
        builder: None,
        rooms: vec!["ArmoryRoom".into()],
        room_bounds: vec![],
        build_ok: true,
        forced_items: vec![],
        public_forced_items: vec![],
        placed_items: vec![],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        quest_public_labels: vec![],
        runtime_sensitive_map: false,
        room_public_facts: vec![
            super::super::room_public::RoomPublicFact::new("ArmoryRoom", 7)
                .expect("Armory contract"),
        ],
        complete: true,
        map: None,
        pre_items_rng_probe: vec![],
        pre_mobs_rng_probe: vec![],
        pre_paint_rng_probe: vec![],
    };
    for class_name in ["Sword", "MailArmor", "Kunai"] {
        let mut reward = GeneratedItem::new(class_name, ItemCategory::Weapon);
        reward.level = 3;
        reward.cursed = true;
        reward.enchantment = Some("Corrupting".into());
        reward.source = Some("ArmoryRoom".into());
        reward.provenance = ItemProvenance::Room(RoomLootRole::RuntimeSensitive);
        floor.placed_items.push(reward);
    }

    let report = floor.to_floor_report();
    assert_eq!(report.items.len(), 1);
    let entry = &report.items[0];
    assert_eq!(entry.prediction, ItemPredictionKind::Constrained);
    assert!(entry.class_name.is_none());
    assert_eq!(entry.category, "other");
    assert_eq!(entry.level, None);
    assert_eq!(entry.cursed, None);
    let json = serde_json::to_string(&report).expect("serialize report");
    for secret in ["Sword", "MailArmor", "Kunai", "Corrupting"] {
        assert!(!json.contains(secret), "leaked {secret}: {json}");
    }
}

#[test]
fn standard_center_room_sampled_classes_do_not_leak_to_report_or_searchable_entries() {
    let mut found = None;
    for seed in 0..128 {
        let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(seed));
        for depth in 1..=9 {
            dungeon.depth = depth;
            let state = crate::level::create_level_partial(&mut dungeon);
            let sampled: Vec<_> = state
                .placed_items
                .iter()
                .filter(|item| {
                    matches!(
                        item.source
                            .as_deref()
                            .and_then(|source| source.split(':').next()),
                        Some(
                            "StudyRoom"
                                | "RitualRoom"
                                | "RingRoom"
                                | "SuspiciousChestRoom"
                                | "GrassyGraveRoom"
                        )
                    )
                })
                .map(|item| item.class_name.clone())
                .collect();
            if !sampled.is_empty() {
                found = Some((state.to_floor_report(), sampled));
                break;
            }
        }
        if found.is_some() {
            break;
        }
    }
    let (report, sampled) = found.expect("standard center-room loot fixture");
    let json = serde_json::to_string(&report).expect("serialize public report");
    assert!(json.contains("reward"));
    assert!(
        report.map.is_none(),
        "divergent callback suppresses sampled map"
    );
    for class_name in sampled {
        assert!(report.items.iter().all(|entry| {
            entry.class_name.as_deref() != Some(class_name.as_str())
                || entry.source.as_deref() == Some("forced")
        }));
    }
}

#[test]
fn quest_report_json_hides_constrained_classes_titles_and_persisted_wands() {
    use crate::items::model::QuestRewardRole;

    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    let mut saw_persisted_wands = false;
    let mut checked_unique_constrained_class = false;
    for depth in 1..=19 {
        dungeon.depth = depth;
        let state = crate::level::create_level_partial(&mut dungeon);
        let report = state.to_floor_report();
        let json = serde_json::to_string(&report).expect("serialize report");

        for item in &state.placed_items {
            if matches!(
                item.provenance,
                ItemProvenance::Quest(
                    QuestRewardRole::GhostWeapon { .. }
                        | QuestRewardRole::WandmakerWand
                        | QuestRewardRole::BlacksmithRoomWeapon { .. }
                        | QuestRewardRole::BlacksmithRoomMissile { .. }
                        | QuestRewardRole::ImpRing
                )
            ) {
                let class_is_exact_elsewhere = report
                    .items
                    .iter()
                    .any(|entry| entry.class_name.as_deref() == Some(&item.class_name))
                    || report.map.as_ref().is_some_and(|map| {
                        map.heaps.iter().any(|heap| {
                            heap.items
                                .iter()
                                .any(|entry| entry.class_name == item.class_name)
                        })
                    });
                if !class_is_exact_elsewhere {
                    assert!(
                        !json.contains(&item.class_name),
                        "leaked {}",
                        item.class_name
                    );
                    checked_unique_constrained_class = true;
                }
                if let Some(enchantment) = &item.enchantment {
                    assert!(report
                        .items
                        .iter()
                        .filter(|entry| entry.source == item.source)
                        .all(|entry| !entry.name.contains(enchantment)));
                }
            }
            if item.provenance == ItemProvenance::Quest(QuestRewardRole::WandmakerPersisted) {
                saw_persisted_wands = true;
            }
        }
        for summary in &state.quests {
            if let Some((prefix, titles)) = summary.split_once(" — ") {
                assert!(report.quests.iter().all(|quest| !quest.contains(titles)));
                assert!(report.quests.iter().all(|quest| {
                    !quest.starts_with(prefix) || quest.starts_with(&format!("{prefix} — "))
                }));
            }
        }
        if saw_persisted_wands && state.quests.is_empty() {
            assert!(report
                .items
                .iter()
                .all(|item| item.source.as_deref() != Some("Wandmaker.Quest")));
        }
    }
    assert!(saw_persisted_wands);
    assert!(checked_unique_constrained_class);
}
