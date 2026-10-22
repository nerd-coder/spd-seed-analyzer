//! Run history only reaches level generation through the ARTIFACT deck
//! (`specs/generator-decks.md` §11). Replaying a seed with runtime artifact
//! draws injected before a floor must leave every entity the public map
//! calls exact unchanged.

use super::*;
use crate::generator::Category;
use crate::report::FloorMap;
use crate::run::{dungeon_from_run, init_run};

const SEEDS: [&str; 5] = [
    "AAA-AAA-AAA",
    "FVX-YTN-SLC",
    "ABC-DEF-GHI",
    "GFX-PZH-DCH",
    "ZZZ-ZZZ-ZZZ",
];

/// Public floor maps for the default profile, after `runtime_artifacts`
/// runtime artifact draws (Ring of Wealth, Thief, Transmutation, …) made on
/// the floor above `before_depth`.
fn public_maps(seed: &str, before_depth: i32, runtime_artifacts: u32) -> Vec<Option<FloorMap>> {
    let info = crate::parse_seed(seed).unwrap();
    let mut dungeon = dungeon_from_run(init_run(info.numeric));
    dungeon.challenges = 0;
    dungeon.expose_variable_items = true;
    trinkets::reset(dungeon.seed);
    (1..=26)
        .map(|depth| {
            dungeon.depth = depth;
            dungeon.branch = 0;
            trinkets::set_held(None);
            if depth == before_depth {
                Random::push_generator_seeded(0x5eed);
                for _ in 0..runtime_artifacts {
                    dungeon
                        .generator
                        .random_using_defaults(Category::Artifact, depth - 1);
                }
                Random::pop_generator();
            }
            let report = create_level_internal(&mut dungeon, true, LevelOutput::FULL)
                .to_floor_report_with_map(true);
            report.map.or(report.assumed_map)
        })
        .collect()
}

fn assert_exact_entities_survive(seed: &str, before_depth: i32, runtime_artifacts: u32) {
    let baseline = public_maps(seed, 0, 0);
    let shifted = public_maps(seed, before_depth, runtime_artifacts);
    let checked: usize = baseline
        .iter()
        .flatten()
        .map(|map| map.heaps.len() + map.mobs.len())
        .sum();
    assert!(checked > 200, "{seed}: only {checked} entities");
    for (index, (base, shifted)) in baseline.iter().zip(&shifted).enumerate() {
        let (Some(base), Some(shifted)) = (base, shifted) else {
            continue;
        };
        let depth = index + 1;
        let exact = |cell: &u32| !base.uncertain_cells.contains(cell);
        for heap in base.heaps.iter().filter(|heap| exact(&heap.cell)) {
            assert_eq!(
                shifted.heaps.iter().find(|other| other.cell == heap.cell),
                Some(heap),
                "{seed} +{runtime_artifacts} artifacts before {before_depth}: depth {depth} heap"
            );
        }
        for mob in base.mobs.iter().filter(|mob| exact(&mob.cell)) {
            assert!(
                shifted.mobs.contains(mob),
                "{seed} +{runtime_artifacts} artifacts before {before_depth}: depth {depth} mob {mob:?}"
            );
        }
    }
}

#[test]
fn runtime_artifact_draws_leave_exact_map_entities_unchanged() {
    for seed in SEEDS {
        for (before_depth, runtime_artifacts) in [(2, 1), (2, 3), (4, 2), (7, 1), (12, 2), (2, 20)]
        {
            assert_exact_entities_survive(seed, before_depth, runtime_artifacts);
        }
    }
}

/// Sewer floors desync only after an artifact draw, so many entities stay exact.
#[test]
fn sewer_floors_keep_exact_entities() {
    let (entities, uncertain) = SEEDS
        .iter()
        .flat_map(|seed| public_maps(seed, 0, 0).into_iter().take(4).skip(1))
        .flatten()
        .map(|map| (map.heaps.len() + map.mobs.len(), map.uncertain_cells.len()))
        .fold((0, 0), |(e, u), (de, du)| (e + de, u + du));
    assert!(entities > 200, "{entities}");
    assert!(
        uncertain * 3 < entities * 2,
        "{uncertain} of {entities} uncertain"
    );
}
