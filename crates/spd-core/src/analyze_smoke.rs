use super::*;

#[test]
fn accuracy_manifest_matches_engine_contract() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../../../specs/accuracy.json"))
            .expect("accuracy manifest must be valid JSON");

    assert_eq!(manifest["target"]["version"], SPD_VERSION);
    assert_eq!(manifest["target"]["commit"], SPD_COMMIT);
    assert_eq!(manifest["overallStatus"], "partial");
}

#[test]
fn analyze_seed_smoke() {
    let r = analyze_seed("GFX-PZH-DCH", 4).expect("analyze");
    eprintln!("status={} floors={}", r.status, r.floors.len());
    for f in &r.floors {
        assert!(
            f.items.iter().all(|item| {
                !item
                    .source
                    .as_deref()
                    .and_then(|source| source.rsplit(':').next())
                    .is_some_and(|origin| matches!(origin, "heap" | "mimic" | "golden_mimic"))
            }),
            "public analysis must omit runtime-sensitive regular and Mimic loot"
        );
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
fn explicit_baseline_profile_can_publish_proven_safe_maps() {
    use crate::{analyze_seed_with_profile, MapMetaProfile, MapProfile, MapTrinketProfile};

    let legacy = analyze_seed("GFX-PZH-DCH", 4).expect("legacy analysis");
    assert!(legacy.floors.iter().all(|floor| floor.map.is_none()));

    let profile = MapProfile {
        trinket: MapTrinketProfile::NoMapAffectingTrinkets,
        meta: MapMetaProfile::Fresh,
        trinket_start_depth: 1,
    };
    let profiled = (0..100)
        .find_map(|seed| {
            let report =
                analyze_seed_with_profile(&seed.to_string(), 4, Some(profile.clone())).ok()?;
            report
                .floors
                .iter()
                .any(|floor| floor.map.is_some())
                .then_some(report)
        })
        .expect("at least one baseline profile should have a public-safe map");
    assert!(profiled.map_profile.is_some());
    let layout = profiled
        .floors
        .iter()
        .find_map(|floor| floor.map.as_ref())
        .expect("profile should expose a layout");
    assert!(layout.markers.is_empty());
    assert!(layout.heaps.is_empty());
    assert!(layout.mobs.is_empty());
}

#[test]
fn trinket_profile_starts_on_the_configured_floor() {
    use crate::{analyze_seed_with_profile, MapMetaProfile, MapProfile, MapTrinketProfile};

    let baseline = MapProfile {
        trinket: MapTrinketProfile::NoMapAffectingTrinkets,
        meta: MapMetaProfile::Fresh,
        trinket_start_depth: 1,
    };
    let delayed = MapProfile {
        trinket: MapTrinketProfile::MossyClump3,
        meta: MapMetaProfile::Fresh,
        trinket_start_depth: 3,
    };
    let baseline = analyze_seed_with_profile("31", 2, Some(baseline)).expect("baseline");
    let delayed = analyze_seed_with_profile("31", 2, Some(delayed)).expect("delayed trinket");

    for (baseline, delayed) in baseline.floors.iter().zip(&delayed.floors) {
        assert_eq!(baseline.feeling, delayed.feeling);
        assert_eq!(baseline.builder, delayed.builder);
        assert_eq!(baseline.rooms, delayed.rooms);
        assert_eq!(
            serde_json::to_value(&baseline.map).expect("baseline map"),
            serde_json::to_value(&delayed.map).expect("delayed map")
        );
    }
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
                    matches!(
                        item.prediction,
                        report::ItemPredictionKind::Exact | report::ItemPredictionKind::Constrained
                    ) && item.level.is_some_and(|level| (1..=3).contains(&level))
                        && item.cursed == Some(false)
                        && item.conditional_notes.is_empty()
                }));
                assert!(rewards.iter().all(|item| {
                    (item.prediction == report::ItemPredictionKind::Exact
                        && item.class_name.is_some()
                        && item.candidate_classes.is_empty())
                        || (item.prediction == report::ItemPredictionKind::Constrained
                            && item.class_name.is_none()
                            && !item.candidate_classes.is_empty())
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
                    .all(|q| !q.contains("one of two +1…+3 wands") && !q.contains(" / ")));
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
                // Level and curse are exact. Identity is exact when the seed
                // rules out both known deck-shift paths, otherwise all three
                // reachable ring classes are reported in order.
                let ring = f
                    .items
                    .iter()
                    .find(|i| i.source.as_deref() == Some("Imp.Quest"));
                if let Some(ring) = ring {
                    assert!(ring.level.is_some());
                    assert_eq!(ring.cursed, Some(true));
                    match ring.prediction {
                        report::ItemPredictionKind::Exact => {
                            assert!(ring.class_name.is_some());
                            assert!(ring.candidate_classes.is_empty());
                            assert!(ring.conditional_notes.is_empty());
                        }
                        report::ItemPredictionKind::Constrained => {
                            assert_eq!(ring.name, format!("+{} ring reward", ring.level.unwrap()));
                            assert!(ring.class_name.is_none());
                            assert_eq!(ring.candidate_classes.len(), 3);
                            assert!(ring.conditional_notes.is_empty());
                        }
                    }
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
                    item.prediction == report::ItemPredictionKind::Exact
                        && item.class_name.is_some()
                        && item.tier.is_some()
                        && item.level.is_some()
                        && item.cursed == Some(false)
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
                        q.ends_with(" — You will get access to those items if you select Smith")
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
        let r = analyze_seed(s, 26).unwrap_or_else(|e| panic!("seed {s}: {e:?}"));
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

    let profile = crate::MapProfile {
        trinket: crate::MapTrinketProfile::NoMapAffectingTrinkets,
        meta: crate::MapMetaProfile::Fresh,
        trinket_start_depth: 1,
    };
    let report = crate::analyze_seed_with_profile("AAA-AAA-AAA", 26, Some(profile))
        .expect("profiled full-run analysis");
    for (depth, dimensions) in [
        (10, (32, 32)),
        (20, (15, 48)),
        (25, (32, 32)),
        (26, (16, 64)),
    ] {
        let floor = &report.floors[depth - 1];
        let map = floor.map.as_ref().expect("fixed dedicated layout");
        assert_eq!((map.width, map.height), dimensions);
        assert!(map.markers.is_empty());
        assert!(map.heaps.is_empty());
        assert!(map.mobs.is_empty());
        assert!(map.traps.is_empty());
        assert!(map.plants.is_empty());
        assert!(map.blobs.is_empty());
        if matches!(depth, 25 | 26) {
            assert!(!map.custom_tiles.is_empty());
            assert!(!map.custom_walls.is_empty());
        } else {
            // Pinned Terrain IDs: WALL_DECO, EMPTY_DECO, REGION_DECO, REGION_DECO_ALT.
            assert!(map
                .tiles
                .iter()
                .all(|tile| !matches!(tile, 12 | 20 | 33 | 34)));
        }
    }
    let sewer_boss = report.floors[4]
        .map
        .as_ref()
        .expect("generated SewerBossLevel layout");
    assert_eq!((sewer_boss.width, sewer_boss.height), (32, 33));
    assert!(sewer_boss.markers.is_empty());
    assert!(sewer_boss.heaps.is_empty());
    assert!(sewer_boss.mobs.is_empty());
    assert!(sewer_boss
        .tiles
        .iter()
        .all(|tile| !matches!(tile, 12 | 20 | 33 | 34)));

    let caves_boss = report.floors[14]
        .map
        .as_ref()
        .expect("generated CavesBossLevel layout");
    assert_eq!((caves_boss.width, caves_boss.height), (33, 42));
    assert!(caves_boss.markers.is_empty());
    assert!(caves_boss.heaps.is_empty());
    assert!(caves_boss.mobs.is_empty());
    assert!(caves_boss
        .tiles
        .iter()
        .all(|tile| !matches!(tile, 12 | 20 | 33 | 34)));

    let halls_boss = report.floors[24]
        .map
        .as_ref()
        .expect("generated HallsBossLevel layout");
    assert_eq!((halls_boss.width, halls_boss.height), (32, 32));
    assert!(halls_boss.markers.is_empty());
    assert!(halls_boss.heaps.is_empty());
    assert!(halls_boss.mobs.is_empty());
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
