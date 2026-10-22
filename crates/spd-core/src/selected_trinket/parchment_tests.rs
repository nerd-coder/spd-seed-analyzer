use super::*;
use crate::level::trinkets::{self, parchment_retention, set_held};
use crate::report::{FloorMap, FloorReport};
use crate::trinkets::ActiveTrinket;

fn hold_parchment(level: Option<u8>) {
    trinkets::reset(1);
    set_held(level.map(|level| ActiveTrinket {
        trinket: TrinketKind::ParchmentScrap,
        level,
        instance: 1,
    }));
}

#[test]
fn retention_reports_the_minimum_level_only_without_parchment() {
    hold_parchment(None);
    assert_eq!(parchment_retention(0.2, 0.2), (true, None));
    assert_eq!(parchment_retention(0.2, 0.3), (false, Some(0)));
    assert_eq!(parchment_retention(0.2, 0.9), (false, Some(2)));
    assert_eq!(parchment_retention(0.3, 0.7), (false, Some(1)));

    hold_parchment(Some(0));
    assert_eq!(parchment_retention(0.2, 0.3), (true, None));
    assert_eq!(parchment_retention(0.2, 0.5), (false, None));

    hold_parchment(Some(3));
    assert_eq!(parchment_retention(0.2, 0.99), (true, None));
    set_held(None);
}

fn parchment_seeds() -> impl Iterator<Item = i64> {
    (0..60).filter(|&seed| {
        TrinketSelectionReport::for_run(&init_run(seed))
            .catalyst_options
            .iter()
            .any(|item| item.class_name() == TrinketKind::ParchmentScrap.class_name())
    })
}

fn layout(floor: &FloorReport) -> Option<&FloorMap> {
    floor.map.as_ref().or(floor.assumed_map.as_ref())
}

fn is_equipment(category: &str) -> bool {
    matches!(category, "weapon" | "armor" | "missile")
}

fn enchanted_equipment(report: &SeedReport) -> usize {
    report
        .floors
        .iter()
        .flat_map(|floor| &floor.items)
        .flat_map(|group| &group.variants)
        .filter(|item| is_equipment(&item.category) && item.enchantment.is_some())
        .count()
}

/// Parchment rolls run on a pushed generator, so holding it never moves a
/// cell, mob, or heap; only curse and enchantment state change.
#[test]
fn parchment_keeps_every_layout_and_raises_enchantments() {
    let mut checked = 0;
    for seed in parchment_seeds().take(4) {
        let input = seed.to_string();
        let default = analyze_seed(&input, 12).unwrap();
        let picked =
            analyze_seed_with_trinket(&input, 12, Some(TrinketKind::ParchmentScrap)).unwrap();
        for (before, after) in default.floors.iter().zip(&picked.floors) {
            let (Some(before), Some(after)) = (layout(before), layout(after)) else {
                continue;
            };
            assert_eq!(before.tiles, after.tiles, "seed {seed}");
            let cells = |map: &FloorMap| {
                let heaps: Vec<_> = map
                    .heaps
                    .iter()
                    .map(|heap| {
                        (
                            heap.cell,
                            heap.items.iter().map(|i| i.item).collect::<Vec<_>>(),
                        )
                    })
                    .collect();
                let mobs: Vec<_> = map.mobs.iter().map(|mob| (mob.cell, mob.mob)).collect();
                (heaps, mobs)
            };
            assert_eq!(cells(before), cells(after), "seed {seed}");
        }
        assert!(
            enchanted_equipment(&picked) > enchanted_equipment(&default),
            "seed {seed}"
        );
        checked += 1;
    }
    assert!(checked > 0, "no seed offered Parchment Scrap");
}

#[test]
fn held_parchment_resolves_ghost_enchantment_conditions() {
    let mut checked = 0;
    for seed in parchment_seeds() {
        let input = seed.to_string();
        let report =
            analyze_seed_with_trinket(&input, 4, Some(TrinketKind::ParchmentScrap)).unwrap();
        let first = report.trinket_selection.first_effective_depth;
        let Some(floor) = report.floors.iter().find(|floor| {
            floor
                .items
                .iter()
                .any(|group| group.source.as_deref() == Some("Ghost.Quest"))
        }) else {
            continue;
        };
        if floor.depth < first {
            continue;
        }
        for item in floor
            .items
            .iter()
            .filter(|group| group.source.as_deref() == Some("Ghost.Quest"))
            .flat_map(|group| &group.variants)
        {
            let enchantment = item
                .enchantment
                .as_ref()
                .expect("+3 keeps the Ghost effect");
            assert!(enchantment.conditions.is_empty(), "seed {seed}");
            assert!(item.notes.iter().all(|note| !note.contains("Parchment")));
        }
        checked += 1;
    }
    assert!(
        checked > 0,
        "no Parchment seed spawned the Ghost after pickup"
    );
}
