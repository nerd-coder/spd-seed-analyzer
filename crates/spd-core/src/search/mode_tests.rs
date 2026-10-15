//! The finder's item-only pass must agree with the full seed-only analysis.

use spd_objects::ItemId;

use super::*;

const PARITY_SEEDS: &[&str] = &["0", "1", "AAA-AAA-AAA", "RZN-LKU-EFS", "123456789"];

fn parity_seed_count() -> i64 {
    std::env::var("SPD_SEARCH_PARITY_SEEDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

#[test]
fn item_only_pass_matches_full_seed_only_items_and_quests() {
    let extra = (0..parity_seed_count()).map(|seed| (seed * 104_729).to_string());
    for seed in PARITY_SEEDS
        .iter()
        .map(|seed| seed.to_string())
        .chain(extra)
    {
        let full = crate::analyze_seed_seed_only(&seed, 26).expect("full analysis");
        let (info, identities, floors) =
            search_seed_floors(&seed, 26, |_| true).expect("item-only pass");
        assert_eq!(info.numeric, full.seed.numeric, "seed {seed}");
        assert_eq!(identities, full.identities, "seed {seed}");
        assert_eq!(floors.len(), full.floors.len(), "seed {seed}");
        for (fast, full) in floors.iter().zip(&full.floors) {
            assert_eq!(fast.depth, full.depth);
            assert_eq!(fast.items, full.items, "seed {seed} depth {}", full.depth);
            assert_eq!(fast.quests, full.quests, "seed {seed} depth {}", full.depth);
            assert_eq!(fast.rooms, full.rooms, "seed {seed} depth {}", full.depth);
            assert!(fast.map.is_none() && fast.assumed_map.is_none());
            assert!(fast.branches.is_empty());
        }
    }
}

#[test]
fn item_only_pass_stops_when_asked() {
    let (_, _, floors) =
        search_seed_floors("0", 26, |so_far| so_far.len() < 3).expect("item-only pass");
    assert_eq!(floors.len(), 3);
}

fn reference_matches(request: &SeedSearchRequest) -> Vec<i64> {
    (request.start_seed..request.start_seed + i64::from(request.candidate_count))
        .filter(|seed| {
            let mut report =
                crate::analyze_seed_seed_only(&seed.to_string(), request.generation_floors())
                    .expect("reference analysis");
            if request.deep_search {
                widen_quest_candidates(&mut report.floors);
            }
            let evidence = matching_evidence(
                &report.floors,
                &request.constraints,
                request.include_variable,
            );
            match request.match_mode {
                MatchMode::Any => !evidence.is_empty(),
                MatchMode::All => evidence.len() == request.constraints.len(),
            }
        })
        .collect()
}

fn constraint(item_group: &str, item: Option<ItemId>, min: u32, max: u32) -> ItemConstraint {
    ItemConstraint {
        item_group: item_group.into(),
        item,
        min_level: None,
        min_depth: min,
        max_depth: max,
    }
}

/// Early exit must never change which seeds match: a window that closes
/// unmatched rules the seed out, and every surviving seed is fully checked.
#[test]
fn early_exit_search_matches_reference_search() {
    let requests = [
        vec![
            constraint("ring", None, 1, 2),
            constraint("wand", None, 1, 6),
        ],
        vec![
            constraint("armor", None, 1, 1),
            constraint("artifact", None, 4, 9),
        ],
        vec![
            constraint("potion", Some(ItemId::PotionOfStrength), 1, 3),
            constraint("weapon", None, 2, 5),
            constraint("scroll", Some(ItemId::ScrollOfUpgrade), 1, 2),
        ],
    ];
    for constraints in requests {
        for deep_search in [false, true] {
            let request = SeedSearchRequest {
                start_seed: 40,
                candidate_count: 24,
                floors: 1,
                constraints: constraints.clone(),
                match_mode: MatchMode::All,
                include_variable: true,
                deep_search,
                max_matches: MAX_SEARCH_MATCHES,
            };
            let found = search_seeds(&request)
                .expect("search")
                .matches
                .iter()
                .map(|found| found.seed.numeric)
                .collect::<Vec<_>>();
            assert_eq!(found, reference_matches(&request), "{constraints:?}");
        }
    }
}
