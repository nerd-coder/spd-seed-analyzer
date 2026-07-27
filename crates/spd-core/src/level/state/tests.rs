use super::*;
use crate::items::model::{ItemCategory, ItemProvenance, RoomLootRole, ShopStockRole};
use crate::report::{MapHeap, MapHeapItem, MapMarker, MapMarkerKind, MapMob};

#[path = "tests/forced_queue.rs"]
mod forced_queue;
#[path = "tests/quantities.rs"]
mod quantities;
#[path = "tests/sacrifice.rs"]
mod sacrifice;

fn item(class_name: &str, source: &str) -> GeneratedItem {
    let mut item = GeneratedItem::new(class_name, ItemCategory::Potion);
    item.source = Some(source.into());
    item
}

#[test]
fn public_projection_omits_the_whole_regular_map_but_keeps_independent_contracts() {
    let runtime_cells = vec![7, 9];
    let floor = LevelState {
        depth: 3,
        feeling: Feeling::None,
        builder: None,
        rooms: vec![],
        room_bounds: vec![],
        pre_shuffle_room_bounds: vec![],
        build_ok: true,
        forced_items: vec![item("PotionOfStrength", "forced")],
        initial_forced_items: vec![item("PotionOfStrength", "forced")],
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
        runtime_sensitive_layout: false,
        runtime_sensitive_feeling: false,
        room_public_facts: vec![
            super::super::room_public::RoomPublicFact::new("SacrificeRoom", 3)
                .expect("Sacrifice contract"),
        ],
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
                MapMarker {
                    cell: 15,
                    kind: MapMarkerKind::Item,
                    label: "Guide Page".into(),
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
                        source: None,
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
                        source: None,
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
                        source: None,
                    }],
                },
                MapHeap {
                    cell: 15,
                    heap_type: "heap".into(),
                    items: vec![MapHeapItem {
                        class_name: "GuidePage".into(),
                        quantity: 1,
                        level: 0,
                        cursed: false,
                        source: None,
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
            custom_tiles: vec![],
            custom_walls: vec![],
            runtime_sensitive_loot_cells: runtime_cells,
            constrained_equipment_cells: vec![],
        }),
        layout_map: None,
        pre_items_rng_probe: vec![],
        pre_mobs_rng_probe: vec![],
        pre_paint_rng_probe: vec![],
        room_paint_rng_checkpoints: vec![],
        post_doors_rng_probe: vec![],
    };

    let report = floor.to_floor_report();
    assert!(report.items.iter().all(|item| item.class_name.is_none()));
    assert!(report
        .items
        .iter()
        .any(|item| { item.name == "food" && item.prediction == ItemPredictionKind::Constrained }));
    assert!(report.map.is_none());
    assert!(report.items.iter().any(|item| {
        item.source.as_deref() == Some("SacrificeRoom")
            && item.prediction == ItemPredictionKind::Constrained
    }));
    let json = serde_json::to_string(&report).expect("serialize public report");
    for unsafe_map_fact in [
        "GuidePage",
        "Guide Page",
        "PotionOfHealing",
        "Mimic",
        "LeatherArmor",
        "for_sale",
    ] {
        assert!(!json.contains(unsafe_map_fact), "leaked {unsafe_map_fact}");
    }

    let mut consumed_internally = floor.clone();
    consumed_internally.forced_items.clear();
    consumed_internally.initial_forced_items.clear();
    let guarded = consumed_internally.to_floor_report();
    assert_eq!(
        serde_json::to_value(report.items).expect("original forced contracts"),
        serde_json::to_value(guarded.items).expect("consumed forced contracts")
    );
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
fn real_shop_remains_constrained_after_inherited_generation_taint() {
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
            ItemProvenance::None
            | ItemProvenance::Quest(_)
            | ItemProvenance::Room(_)
            | ItemProvenance::Forced(_) => None,
        })
        .collect();
    assert!(!internal_shop.is_empty());

    let report = state.to_floor_report();
    assert!(report
        .items
        .iter()
        .any(|entry| entry.source.as_deref() == Some("ShopRoom")));
    assert!(report.items.iter().all(|entry| {
        matches!(
            entry.source.as_deref(),
            Some("guaranteed floor spawn" | "ShopRoom")
        )
    }));
    assert!(report.map.is_none());

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
    assert_eq!(search.matches.len(), 1, "fixed shop armor is searchable");
}

#[test]
fn room_reward_projection_hides_all_concrete_fields_and_deduplicates_counts() {
    let mut floor = LevelState {
        depth: 7,
        feeling: Feeling::None,
        builder: None,
        rooms: vec!["ArmoryRoom".into()],
        room_bounds: vec![],
        pre_shuffle_room_bounds: vec![],
        build_ok: true,
        forced_items: vec![],
        initial_forced_items: vec![],
        placed_items: vec![],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        quest_public_labels: vec![],
        runtime_sensitive_map: false,
        runtime_sensitive_layout: false,
        runtime_sensitive_feeling: false,
        room_public_facts: vec![
            super::super::room_public::RoomPublicFact::new("ArmoryRoom", 7)
                .expect("Armory contract"),
        ],
        complete: true,
        map: None,
        layout_map: None,
        pre_items_rng_probe: vec![],
        pre_mobs_rng_probe: vec![],
        pre_paint_rng_probe: vec![],
        room_paint_rng_checkpoints: vec![],
        post_doors_rng_probe: vec![],
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
    let entries: Vec<_> = report
        .items
        .iter()
        .filter(|item| item.source.as_deref() == Some("ArmoryRoom"))
        .collect();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].prediction, ItemPredictionKind::Constrained);
    assert!(entries[0].class_name.is_none());
    assert_eq!(entries[0].category, "bomb / weapon / armor / missile");
    assert_eq!(
        entries[0].level_range,
        Some(crate::report::NumericRange { min: 0, max: 2 })
    );
    assert_eq!(entries[0].cursed, None);
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
    if report.rooms.is_empty() {
        assert!(report
            .items
            .iter()
            .all(|item| { item.source.as_deref() == Some("guaranteed floor spawn") }));
    } else {
        assert!(json.contains("reward"));
    }
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
                        | QuestRewardRole::ImpRing { .. }
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
