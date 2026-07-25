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
    assert!(
        r.floors
            .iter()
            .filter_map(|floor| floor.map.as_ref())
            .any(|map| !map.markers.is_empty()),
        "createItems should export at least one exact marker"
    );
}

#[test]
fn ghost_quest_spawns_within_sewers_sometime() {
    // Depth 4 always rolls Int(1)==0 if not yet spawned; over many seeds we
    // should see at least one Ghost.Quest reward before floor 5.
    let mut saw_ghost = false;
    for s in [
        "GFX-PZH-DCH",
        "AAA-AAA-AAA",
        "hello",
        "42",
        "shattered",
        "JLY-ZYR-HET",
    ] {
        let r = analyze_seed(s, 4).expect("analyze");
        for f in &r.floors {
            if f.quests.iter().any(|q| q.contains("Sad Ghost"))
                || f.items
                    .iter()
                    .any(|i| i.source.as_deref() == Some("Ghost.Quest"))
            {
                let rewards: Vec<_> = f
                    .items
                    .iter()
                    .filter(|i| i.source.as_deref() == Some("Ghost.Quest"))
                    .collect();
                let weapon = rewards
                    .iter()
                    .find(|item| item.category == "weapon")
                    .unwrap();
                assert_eq!(weapon.prediction, report::ItemPredictionKind::Constrained);
                assert!(weapon.class_name.is_none());
                assert!(weapon.tier.is_some() && weapon.level.is_some());
                let armor = rewards
                    .iter()
                    .find(|item| item.category == "armor")
                    .unwrap();
                assert_eq!(armor.prediction, report::ItemPredictionKind::Exact);
                assert!(armor.class_name.is_some());
                assert!(armor.tier.is_some() && armor.level.is_some());
                assert!(rewards.iter().all(|reward| reward.cursed == Some(false)));
                assert!(f.quests.iter().all(|q| !q.contains(" / ")));
                saw_ghost = true;
                break;
            }
        }
        if saw_ghost {
            break;
        }
    }
    assert!(saw_ghost, "expected Ghost.Quest on at least one sewer run");
}

#[test]
fn shop_stock_on_floor_6() {
    let r = analyze_seed("GFX-PZH-DCH", 6).expect("analyze");
    let f6 = r.floors.iter().find(|f| f.depth == 6).expect("floor 6");
    let shop: Vec<_> = f6
        .items
        .iter()
        .filter(|i| i.source.as_deref() == Some("ShopRoom"))
        .collect();
    assert!(
        !shop.is_empty(),
        "expected ShopRoom stock on depth 6, rooms={:?}",
        f6.rooms
    );
    assert!(shop.iter().any(|item| {
        item.prediction == report::ItemPredictionKind::Constrained
            && item.name == "weapon stock"
            && item.class_name.is_none()
            && item.tier == Some(2)
            && item.level == Some(0)
            && item.cursed == Some(false)
    }));
    assert!(shop.iter().any(|item| {
        item.prediction == report::ItemPredictionKind::Constrained
            && item.name == "inventory-dependent bag stock"
            && item.class_name.is_none()
    }));
    assert!(shop.iter().any(|item| {
        item.prediction == report::ItemPredictionKind::Constrained
            && item.name == "Hourglass sand stock"
            && item.class_name.is_none()
    }));
    if let Some(map) = &f6.map {
        assert!(map.heaps.iter().all(|heap| heap.heap_type != "for_sale"));
    }
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
                        && item.class_name.is_none()
                        && item.level.is_none()
                        && item.cursed == Some(false)
                }));
                if let Some(map) = &f.map {
                    assert!(map.mobs.is_empty());
                    assert!(map.heaps.is_empty());
                }
                assert!(f.quests.iter().all(|q| !q.contains(" / ")));
                saw = true;
                break;
            }
        }
        if saw {
            break;
        }
    }
    assert!(saw, "expected Wandmaker.Quest on at least one prison run");
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
                // Only the forced category/curse are public. Identity and
                // level is stable; only identity depends on persistent ring history.
                let ring = f
                    .items
                    .iter()
                    .find(|i| i.source.as_deref() == Some("Imp.Quest"));
                if let Some(ring) = ring {
                    assert_eq!(ring.prediction, report::ItemPredictionKind::Constrained);
                    assert_eq!(ring.name, "Imp ring reward");
                    assert!(ring.class_name.is_none());
                    assert!(ring.level.is_some());
                    assert_eq!(ring.cursed, Some(true));
                }
                break;
            }
        }
        if saw {
            break;
        }
    }
    assert!(saw, "expected Imp.Quest on at least one city run");
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
                    .all(|q| q.ends_with(" — reward options")));
                break;
            }
        }
        if saw {
            break;
        }
    }
    assert!(saw, "expected Blacksmith.Quest on at least one caves run");
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
    assert!(saw, "expected CrystalVaultRoom prizes on at least one seed");
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
        // Boss floors + last level have no regular map/items yet
        for depth in [5u32, 10, 15, 20, 25, 26] {
            let f = r.floors.iter().find(|f| f.depth == depth).expect("depth");
            assert!(
                f.map.is_none(),
                "depth {depth} should skip RegularLevel paint"
            );
        }
        // A mid Halls floor should still generate
        let f24 = r.floors.iter().find(|f| f.depth == 24).expect("24");
        assert!(f24.map.is_some() || !f24.rooms.is_empty() || f24.builder.is_some());
    }
}

#[test]
fn halls_report_the_mandatory_demon_spawner() {
    let report = analyze_seed("GFX-PZH-DCH", 24).expect("analyze");
    for depth in 21..=24 {
        let floor = &report.floors[(depth - 1) as usize];
        assert!(
            floor.rooms.iter().any(|room| room == "DemonSpawnerRoom"),
            "missing demon spawner on depth {depth}"
        );
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
