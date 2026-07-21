//! Pure Rust port of Shattered Pixel Dungeon generation logic needed for seed analysis.
//!
//! Target game source: local clone of 00-Evan/shattered-pixel-dungeon (see README).

pub mod builders;
pub mod dungeon;
pub mod dungeon_seed;
pub mod generator;
pub mod geom;
pub mod items;
pub mod java_random;
pub mod level;
pub mod quests;
pub mod random;
pub mod report;
pub mod rooms;
pub mod run;

pub use dungeon_seed::{DungeonSeed, SeedError, TOTAL_SEEDS};
pub use items::IdentityMaps;
pub use java_random::JavaRandom;
pub use random::Random;
pub use report::{AnalyzeError, FloorReport, SeedInfo, SeedReport};
pub use run::{dungeon_from_run, init_run, RunState};

/// Pinned SPD version this port targets (from local clone at scaffold time).
pub const SPD_VERSION: &str = "v3.3.8";
pub const SPD_COMMIT: &str = "7b8b845a7";

/// Parse a user seed string into display info (no levelgen).
pub fn parse_seed(input: &str) -> Result<SeedInfo, SeedError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(SeedError::Empty);
    }
    let numeric = DungeonSeed::convert_from_text(trimmed)?;
    let code = if numeric < TOTAL_SEEDS {
        DungeonSeed::convert_to_code(numeric).ok()
    } else {
        None
    };
    Ok(SeedInfo {
        input: trimmed.to_string(),
        numeric,
        code,
        formatted: DungeonSeed::format_text(trimmed),
    })
}

/// Analyze a seed for the given number of floors.
///
/// Returns identity maps plus **partial** per-floor data (forced drops / feelings).
/// Full room loot is not yet ported.
pub fn analyze_seed(input: &str, floors: u32) -> Result<SeedReport, AnalyzeError> {
    let info = parse_seed(input)?;
    let floors = floors.clamp(1, 26);
    let run = init_run(info.numeric);
    let mut dungeon = dungeon_from_run(run);
    let identities = dungeon.identities.clone();
    let floor_reports = level::analyze_floors(&mut dungeon, floors);

    Ok(SeedReport {
        seed: info,
        spd_version: SPD_VERSION.to_string(),
        spd_commit: SPD_COMMIT.to_string(),
        floors_requested: floors,
        identities,
        floors: floor_reports,
        status: "partial".to_string(),
        message: Some(
            "Partial analysis: layout builder + special/secret-room prizes (including full SecretMaze layout, RotGarden, WeakFloor, and DemonSpawner paint) + shop/crystal loot (approx.) + Ghost/Wandmaker/Blacksmith/Imp quest rewards + water/grass/trap painter + paintDoors merge/Graph + generic/region standard-room geometry + main createItems drops. Full createMobs, remaining special-room geometry, region room-count parity, and figure-eight builder are still incomplete — results may not match the game yet."
                .to_string(),
        ),
    })
}

#[cfg(test)]
mod analyze_smoke {
    use super::*;

    #[test]
    fn analyze_seed_smoke() {
        let r = analyze_seed("GFX-PZH-DCH", 4).expect("analyze");
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
        }
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
                    // Reward is always a cursed +2 (or more) ring
                    let ring = f
                        .items
                        .iter()
                        .find(|i| i.source.as_deref() == Some("Imp.Quest"));
                    if let Some(ring) = ring {
                        assert!(
                            ring.cursed && ring.name.contains("Ring"),
                            "imp reward cursed ring: cursed={} name={}",
                            ring.cursed,
                            ring.name
                        );
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
        }
    }
}
