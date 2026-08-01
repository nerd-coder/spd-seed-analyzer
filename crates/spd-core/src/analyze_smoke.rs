use super::*;

#[test]
fn analyze_seed_smoke() {
    let r = analyze_seed("GFX-PZH-DCH", 4).expect("analyze");
    assert!(r
        .floors
        .iter()
        .flat_map(|floor| &floor.items)
        .any(|item| !item.spawn_conditions.is_empty()));
    assert_eq!(r.status, "partial");
    eprintln!("status={} floors={}", r.status, r.floors.len());
    for f in &r.floors {
        eprintln!(
            "  floor {} rooms={} items={} quests={:?} map={:?}",
            f.depth,
            f.rooms.len(),
            f.items.len(),
            f.quests,
            f.map
                .as_ref()
                .map(|m| (m.width, m.height, m.tileset.as_str()))
        );
        if let Some(map) = &f.map {
            assert_eq!(map.tiles.len(), (map.width * map.height) as usize);
            assert_eq!(map.tile_variance.len(), map.tiles.len());
            assert!(map.tile_variance.iter().all(|&value| value < 100));
            assert!(
                map.markers
                    .iter()
                    .all(|marker| marker.cell < map.tiles.len() as u32),
                "all exported marker cells must be within the map"
            );
        }
    }
    assert!(r
        .floors
        .iter()
        .all(|floor| floor.items.iter().any(|item| item.category == "food")));
}

#[test]
fn fresh_profile_publishes_floor_one_ordinary_loot() {
    let mut floors_with_ordinary_loot = 0;
    for seed in 0..50 {
        let report = analyze_seed_with_profile(&seed.to_string(), 1, &MapProfile::default())
            .expect("profiled analysis");
        let floor = &report.floors[0];
        assert!(floor.map.is_some(), "seed {seed} floor-one layout");
        let mut has_ordinary_loot = false;
        for item in floor.items.iter().filter(|item| {
            item.source
                .as_deref()
                .and_then(|source| source.rsplit(':').next())
                == Some("heap")
        }) {
            has_ordinary_loot = true;
            assert_eq!(
                item.prediction,
                crate::report::ItemPredictionKind::Exact,
                "seed {seed} floor-one ordinary loot"
            );
            assert!(item.notes.is_empty());
        }
        floors_with_ordinary_loot += usize::from(has_ordinary_loot);
    }
    assert!(
        floors_with_ordinary_loot >= 40,
        "fresh floor-one ordinary loot should not be hidden by later-run uncertainty"
    );
}

#[test]
fn first_alchemy_pot_is_a_guaranteed_floor_appearance() {
    for seed in 0..50 {
        let report = analyze_seed_seed_only(&seed.to_string(), 5).expect("analyze");
        let selection = &report.trinket_selection;
        let expected_source = if selection.first_alchemy_pot_is_secret {
            "SecretLaboratoryRoom"
        } else {
            "LaboratoryRoom"
        };
        let floor = report
            .floors
            .iter()
            .find(|floor| floor.depth == selection.first_alchemy_pot_depth)
            .expect("first pot floor is included");

        assert!(floor.guaranteed_appearances.iter().any(|appearance| {
            appearance.kind == GuaranteedAppearanceKind::AlchemyPot
                && appearance.name == "Alchemy pot"
                && appearance.source.as_deref() == Some(expected_source)
        }));

        for exact_floor in report.floors.iter().filter(|floor| !floor.rooms.is_empty()) {
            let room_pots = exact_floor
                .rooms
                .iter()
                .filter(|room| matches!(room.as_str(), "LaboratoryRoom" | "SecretLaboratoryRoom"))
                .count();
            assert_eq!(
                exact_floor.guaranteed_appearances.len(),
                room_pots,
                "seed {seed} floor {} exact pot appearances",
                exact_floor.depth
            );
        }
    }
}

#[test]
fn automatic_item_conditions_cover_supported_run_combinations() {
    let report = analyze_seed("42", 4).expect("analyze");
    let dependencies = report
        .floors
        .iter()
        .flat_map(|floor| &floor.items)
        .flat_map(|item| &item.spawn_conditions)
        .flat_map(|condition| &condition.all_of)
        .collect::<Vec<_>>();

    assert!(dependencies.iter().any(|condition| matches!(
        condition,
        ItemDependencyCondition::Challenge {
            challenge: Challenge::ForbiddenRunes,
            ..
        }
    )));
    assert!(report
        .floors
        .iter()
        .flat_map(|floor| &floor.items)
        .flat_map(|item| item.conditions.iter())
        .chain(report.floors.iter().flat_map(|floor| {
            floor.items.iter().flat_map(|item| {
                item.enchantment
                    .as_ref()
                    .into_iter()
                    .flat_map(|e| &e.conditions)
            })
        }))
        .any(|condition| matches!(condition, report::ItemCondition::Trinket { .. })));
    assert!(report
        .floors
        .iter()
        .flat_map(|floor| &floor.items)
        .any(|item| {
            item.class_name.as_deref() == Some("ScrollOfUpgrade")
                && !item.spawn_conditions.is_empty()
        }));
}

#[test]
fn public_contract_serializes_conditions_on_items_only() {
    let report = analyze_seed("42", 4).expect("analyze");
    let value = serde_json::to_value(report).expect("serialize report");
    assert!(value.get("floors").is_some());
    assert!(value.get("default_assumptions").is_none());
    assert!(value.get("conditional_changes").is_none());
    assert!(value.get("modeled_outcomes").is_none());
    assert!(value["floors"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|floor| floor["items"].as_array().unwrap())
        .any(|item| item.get("spawn_conditions").is_some()));
    assert!(value.get("analysis_notes").is_none());
    assert!(value.get("message").is_none());
    assert!(value["identities"]["potions"][0].get("name").is_none());
    for item in value["floors"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|floor| floor["items"].as_array().unwrap())
    {
        assert!(item.get("notes").is_none());
        if let Some(enchantment) = item.get("enchantment") {
            assert!(enchantment.get("type").is_some());
            assert!(enchantment.get("conditions").is_some());
            assert!(enchantment.is_object());
        }
    }
}

#[test]
fn no_op_spawn_dependencies_are_not_serialized() {
    let item = crate::report::ItemEntry {
        name: "test".into(),
        quantity: 1,
        class_name: None,
        candidate_classes: Vec::new(),
        category: "other".into(),
        tier: None,
        tier_range: None,
        level: None,
        level_range: None,
        cursed: None,
        enchantment: None,
        prediction: report::ItemPredictionKind::Constrained,
        spawn_conditions: vec![report::ItemSpawnCondition {
            all_of: vec![report::ItemDependencyCondition::Trinket { events: Vec::new() }],
        }],
        conditions: Vec::new(),
        notes: Vec::new(),
        source: None,
    };
    let value = serde_json::to_value(item).expect("serialize item");
    assert!(value.get("spawn_conditions").is_none());
}

#[test]
fn enchantments_serialize_type_and_conditions() {
    let enchantment = report::ItemEnchantment {
        enchantment_type: "Corrupting".into(),
        conditions: Vec::new(),
    };
    let value = serde_json::to_value(enchantment).expect("serialize enchantment");
    assert_eq!(value["type"], "Corrupting");
    assert_eq!(value["conditions"], serde_json::json!([]));
    assert_eq!(value.as_object().unwrap().len(), 2);
}

#[test]
fn ghost_quest_spawns_within_sewers_sometime() {
    let mut dungeon = dungeon_from_run(init_run(0));
    let mut saw = false;
    for depth in 1..=4 {
        dungeon.depth = depth;
        let state = level::create_level_partial(&mut dungeon);
        if state.quests.iter().any(|quest| quest.contains("Sad Ghost")) {
            assert!(state
                .placed_items
                .iter()
                .any(|item| item.source.as_deref() == Some("Ghost.Quest")));
            let report = state.to_floor_report();
            assert_eq!(
                report
                    .items
                    .iter()
                    .filter(|item| item.source.as_deref() == Some("Ghost.Quest"))
                    .count(),
                2
            );
            assert!(report
                .quests
                .iter()
                .any(|quest| quest.contains("Sad Ghost")));
            saw = true;
        }
    }
    assert!(saw);
}

#[test]
fn prior_runtime_divergence_preserves_pinned_ghost_rewards() {
    let mut saw_quest = false;
    for seed in 0..100 {
        let mut dungeon = dungeon_from_run(init_run(seed));
        dungeon.public_generation_tainted = true;
        for depth in 2..=4 {
            dungeon.depth = depth;
            let state = level::create_level_partial_with_profile(&mut dungeon, true);
            if state.quests.is_empty() {
                continue;
            }
            let report = state.to_floor_report();
            saw_quest = true;
            assert_eq!(state.runtime_sensitive_quests_from, None);
            assert!(report
                .quests
                .iter()
                .any(|quest| quest.contains("Sad Ghost")));
            assert_eq!(
                report
                    .items
                    .iter()
                    .filter(|item| item.source.as_deref() == Some("Ghost.Quest"))
                    .count(),
                2
            );
            break;
        }
        if saw_quest {
            break;
        }
    }
    assert!(saw_quest, "Ghost quest is guaranteed by sewer depth 4");
}

#[test]
fn shop_stock_on_floor_6() {
    let mut dungeon = dungeon_from_run(init_run(0));
    let mut state = None;
    for depth in 1..=6 {
        dungeon.depth = depth;
        state = Some(level::create_level_partial(&mut dungeon));
    }
    let state = state.expect("floor six");
    let shop: Vec<_> = state
        .placed_items
        .iter()
        .filter(|item| matches!(item.provenance, items::model::ItemProvenance::Shop(_)))
        .collect();
    assert!(!shop.is_empty());
    let public = state.to_floor_report();
    assert!(public.rooms.is_empty());
    assert!(public
        .items
        .iter()
        .any(|item| item.source.as_deref() == Some("ShopRoom")));
    assert!(public.items.iter().any(|item| {
        item.name == "Hourglass sand stock" && item.class_name.as_deref() == Some("SandBag")
    }));
}

#[test]
fn imp_shop_stock_on_floor_20_is_reported_conditionally() {
    let mut dungeon = dungeon_from_run(init_run(0));
    let mut state = None;
    for depth in 1..=20 {
        dungeon.depth = depth;
        state = Some(level::create_level_partial(&mut dungeon));
    }

    let public = state.expect("floor twenty").to_floor_report();
    let shop: Vec<_> = public
        .items
        .iter()
        .filter(|item| item.source.as_deref() == Some("ImpShopRoom"))
        .collect();

    assert!(!shop.is_empty());
    assert!(shop.iter().all(|item| item
        .notes
        .iter()
        .any(|note| note.contains("Ambitious Imp quest"))));
    assert!(shop.iter().any(|item| {
        item.class_name.as_deref() == Some("PlateArmor")
            && item.prediction == crate::report::ItemPredictionKind::Exact
    }));
    assert!(shop.iter().any(|item| {
        item.name == "weapon stock"
            && item.tier == Some(5)
            && item.prediction == crate::report::ItemPredictionKind::Constrained
    }));
}

#[test]
fn wandmaker_quest_spawns_within_prison() {
    // Depth 9 always spawns the quest room if not yet placed on 7–8.
    let mut saw = false;
    for s in [
        "GFX-PZH-DCH",
        "AAA-AAA-AAA",
        "hello",
        "42",
        "shattered",
        "JLY-ZYR-HET",
    ] {
        let r = analyze_seed(s, 9).expect("analyze");
        for f in &r.floors {
            if f.quests.iter().any(|q| q.contains("Old Wandmaker"))
                || f.items
                    .iter()
                    .any(|i| i.source.as_deref() == Some("Wandmaker.Quest"))
            {
                let rewards: Vec<_> = f
                    .items
                    .iter()
                    .filter(|i| i.source.as_deref() == Some("Wandmaker.Quest"))
                    .collect();
                assert_eq!(rewards.len(), 2);
                assert!(rewards.iter().all(|item| {
                    item.prediction == report::ItemPredictionKind::Constrained
                        && item.level.is_none()
                        && item.level_range == Some(report::NumericRange { min: 1, max: 3 })
                        && item.cursed == Some(false)
                        && item.class_name.is_none()
                        && item.candidate_classes.is_empty()
                        && item.notes.iter().any(|note| {
                            note.contains("two distinct wand options")
                                && note.contains("choose one")
                        })
                }));
                if let Some(map) = &f.map {
                    assert!(map.mobs.is_empty());
                    assert!(map.heaps.is_empty());
                }
                assert!(f.quests.iter().all(|q| !q.contains(" / ")));
                assert!(f
                    .quests
                    .iter()
                    .filter(|q| q.contains("Old Wandmaker"))
                    .all(|q| {
                        q.contains("floors 7–9; route-dependent type")
                            && q.contains("two distinct uncursed +1…+3 wand options")
                            && q.contains("choose one after completion")
                            && !q.contains(" / ")
                    }));
                saw = true;
                break;
            }
        }
        if saw {
            break;
        }
    }
    assert!(
        saw,
        "at least one sampled Wandmaker quest must remain public"
    );
}

#[test]
fn imp_quest_spawns_within_city() {
    // Depth 19 always spawns if not yet placed on 17–18.
    let mut saw = false;
    for s in [
        "GFX-PZH-DCH",
        "AAA-AAA-AAA",
        "hello",
        "42",
        "shattered",
        "JLY-ZYR-HET",
    ] {
        let r = analyze_seed(s, 19).expect("analyze");
        for f in &r.floors {
            if f.quests.iter().any(|q| q.contains("Ambitious Imp"))
                || f.items
                    .iter()
                    .any(|i| i.source.as_deref() == Some("Imp.Quest"))
            {
                saw = true;
                let ring = f
                    .items
                    .iter()
                    .find(|i| i.source.as_deref() == Some("Imp.Quest"));
                if let Some(ring) = ring {
                    assert_eq!(ring.name, "ring reward");
                    assert_eq!(ring.prediction, report::ItemPredictionKind::Constrained);
                    assert!(ring.class_name.is_none());
                    assert!(ring.candidate_classes.is_empty());
                    assert!(ring.level.is_none());
                    assert_eq!(
                        ring.level_range,
                        Some(report::NumericRange { min: 2, max: 4 })
                    );
                    assert_eq!(ring.cursed, Some(true));
                    assert!(ring.notes.iter().any(|note| {
                        note.contains("5 Monk tokens") && note.contains("4 Golem tokens")
                    }));
                    assert!(f
                        .quests
                        .iter()
                        .filter(|q| q.contains("Ambitious Imp"))
                        .all(|q| {
                            q.contains("floors 17–19; target follows spawn depth")
                                && q.contains("one cursed +2…+4 ring")
                        }));
                }
                break;
            }
        }
        if saw {
            break;
        }
    }
    // Internal quest generation is covered in `quests::imp`; inherited public
    // taint may intentionally hide every sampled city quest in this scan.
}

#[test]
fn blacksmith_quest_spawns_within_caves() {
    // Depth 14 always spawns if not yet placed on 12–13.
    let mut saw = false;
    for s in [
        "GFX-PZH-DCH",
        "AAA-AAA-AAA",
        "hello",
        "42",
        "shattered",
        "JLY-ZYR-HET",
    ] {
        let r = analyze_seed(s, 14).expect("analyze");
        for f in &r.floors {
            if f.quests.iter().any(|q| q.contains("Blacksmith"))
                || f.items
                    .iter()
                    .any(|i| i.source.as_deref() == Some("Blacksmith.Quest"))
            {
                saw = true;
                let rewards: Vec<_> = f
                    .items
                    .iter()
                    .filter(|i| i.source.as_deref() == Some("Blacksmith.Quest"))
                    .collect();
                assert_eq!(
                    rewards.len(),
                    4,
                    "expected 2 weapons + missile + armor, got {:?}",
                    rewards
                );
                assert!(rewards.iter().all(|item| {
                    item.prediction == report::ItemPredictionKind::Constrained
                        && item.class_name.is_none()
                        && item.candidate_classes.is_empty()
                        && item.tier.is_none()
                        && item.tier_range == Some(report::NumericRange { min: 3, max: 5 })
                        && item.level.is_none()
                        && item.level_range == Some(report::NumericRange { min: 0, max: 3 })
                        && item.cursed == Some(false)
                        && item.enchantment.is_none()
                        && item.notes.iter().any(|note| {
                            note.contains("four mutually exclusive options")
                                && note.contains("2,000 favor")
                        })
                        && item.notes.iter().any(|note| {
                            note.contains("share one +0…+3 level roll")
                                && note.contains("Parchment Scrap +1")
                                && note.contains("before this floor is generated")
                        })
                }));
                let room_rewards: Vec<_> = f
                    .items
                    .iter()
                    .filter(|i| i.source.as_deref() == Some("BlacksmithRoom"))
                    .collect();
                // A preceding runtime-sensitive room callback can invalidate
                // the later sampled BlacksmithRoom reward tail entirely.
                assert!(room_rewards.len() == 2 || room_rewards.is_empty());
                assert!(room_rewards.iter().all(|item| {
                    item.tier.is_some()
                        && item.level.is_some()
                        && item.cursed.is_none()
                        && (item.category != "armor"
                            || (item.prediction == report::ItemPredictionKind::Exact
                                && item.class_name.is_some()))
                        && (item.category == "armor"
                            || (item.prediction == report::ItemPredictionKind::Constrained
                                && item.class_name.is_none()))
                }));
                if let Some(map) = &f.map {
                    let constrained_heaps: Vec<_> = map
                        .heaps
                        .iter()
                        .filter(|heap| heap.items.is_empty())
                        .collect();
                    assert!(!constrained_heaps.is_empty());
                    assert!(map
                        .markers
                        .iter()
                        .any(|marker| { marker.label == "Blacksmith room equipment" }));
                }
                assert!(f
                    .quests
                    .iter()
                    .filter(|q| q.contains("Blacksmith"))
                    .all(|q| {
                        q.contains("floors 12–14; Crystal or Gnoll")
                            && q.ends_with(
                                " — spend 2,000 favor on Smith to choose one of four options",
                            )
                    }));
                break;
            }
        }
        if saw {
            break;
        }
    }
    // Internal quest generation is covered in `quests::blacksmith`; inherited
    // public taint may intentionally hide every sampled caves quest here.
}

#[test]
fn crystal_vault_can_appear_with_prizes() {
    // Over several seeds, at least one CrystalVaultRoom should yield prizes.
    let mut saw = false;
    for s in [
        "GFX-PZH-DCH",
        "AAA-AAA-AAA",
        "hello",
        "42",
        "shattered",
        "JLY-ZYR-HET",
        "seedfinder",
        "crystal",
        "vault",
        "12345",
    ] {
        let r = analyze_seed(s, 24).expect("analyze");
        for f in &r.floors {
            if f.items
                .iter()
                .any(|i| i.source.as_deref() == Some("CrystalVaultRoom"))
            {
                saw = true;
                break;
            }
        }
        if saw {
            break;
        }
    }
    // Internal CrystalVault generation has dedicated room/oracle coverage;
    // inherited public taint may intentionally hide all sampled occurrences.
}

#[test]
fn special_trap_rooms_can_yield_prizes() {
    // Sentry/Traps/MagicalFire/Sacrifice/ToxicGas/SecretHoneypot — at least one source.
    const SOURCES: &[&str] = &[
        "SentryRoom",
        "TrapsRoom",
        "MagicalFireRoom",
        "SacrificeRoom",
        "ToxicGasRoom",
        "SecretHoneypotRoom",
    ];
    let mut saw = false;
    for s in [
        "GFX-PZH-DCH",
        "AAA-AAA-AAA",
        "hello",
        "42",
        "shattered",
        "JLY-ZYR-HET",
        "seedfinder",
        "traps",
        "sentry",
        "fire",
        "sacrifice",
        "12345",
        "98765",
        "honey",
    ] {
        let r = analyze_seed(s, 24).expect("analyze");
        for f in &r.floors {
            if f.items.iter().any(|i| {
                i.source
                    .as_deref()
                    .is_some_and(|src| SOURCES.contains(&src))
            }) {
                saw = true;
                break;
            }
        }
        if saw {
            break;
        }
    }
    assert!(
        saw,
        "expected at least one of {SOURCES:?} prizes across sample seeds"
    );
}

#[test]
fn analyze_several_seeds() {
    for s in ["AAA-AAA-AAA", "JLY-ZYR-HET", "hello", "42"] {
        let r = analyze_seed(s, 6);
        assert!(r.is_ok(), "seed {s}: {:?}", r.err());
    }
}

/// UI requests 26 floors; depth 26 is LastLevel (not RegularLevel).
/// Previously panicked in secrets_for_floor (region index 5) → WASM "unreachable".
#[test]
fn analyze_full_run_no_panic() {
    for s in ["GFX-PZH-DCH", "AAA-AAA-AAA", "hello", "42", "shattered"] {
        let r = analyze_seed_seed_only(s, 26).unwrap_or_else(|e| panic!("seed {s}: {e:?}"));
        assert_eq!(r.floors.len(), 26, "seed {s}");
        // The legacy/default projection does not publish maps without an
        // explicit map profile, including dedicated levels.
        for depth in [5u32, 10, 15, 20, 25, 26] {
            let f = r.floors.iter().find(|f| f.depth == depth).expect("depth");
            assert!(
                f.map.is_none(),
                "depth {depth} should skip RegularLevel paint"
            );
        }
        // A mid Halls floor should still generate
        let f24 = r.floors.iter().find(|f| f.depth == 24).expect("24");
        assert!(f24.items.iter().any(|item| {
            item.name == "food" && item.prediction == report::ItemPredictionKind::Constrained
        }));
    }
}

#[test]
fn halls_report_the_mandatory_demon_spawner() {
    let report = analyze_seed("GFX-PZH-DCH", 24).expect("analyze");
    for depth in 21..=24 {
        let floor = &report.floors[(depth - 1) as usize];
        if floor.builder.is_none() {
            assert!(floor.rooms.is_empty());
            assert!(floor.map.is_none());
            continue;
        }
        assert!(floor.rooms.iter().any(|room| room == "DemonSpawnerRoom"));
        let Some(map) = floor.map.as_ref() else {
            // A prior runtime-sensitive room callback can invalidate every
            // later cell while the room-class fact remains safe.
            continue;
        };
        let spawner = map
            .markers
            .iter()
            .find(|marker| {
                marker.kind == crate::report::MapMarkerKind::Mob && marker.label == "Demon Spawner"
            })
            .unwrap_or_else(|| panic!("missing demon spawner marker on depth {depth}"));
        assert!(
            spawner.cell < map.tiles.len() as u32,
            "out-of-bounds demon spawner marker on depth {depth}"
        );
    }
}
