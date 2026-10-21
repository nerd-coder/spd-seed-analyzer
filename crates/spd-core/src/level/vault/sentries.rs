//! `VaultSentry` scan patterns: the cells each scan checks, in scan order.

use std::collections::BTreeSet;

use crate::level::ballistica::{ballistica, cone_aoe, STOP_SOLID, STOP_TARGET, WONT_STOP};
use crate::level::terrain::TerrainMap;
use crate::report::MapSentryScan;

/// The scan fields a vault room sets on its `VaultSentry`.
#[derive(Debug, Clone)]
pub(super) struct SentrySpec {
    pub cell: usize,
    pub length: f32,
    pub width: f32,
    pub dirs: Vec<Vec<usize>>,
    /// `afterScanCooldown`; `scansAfterCooldown` is always 1 in vaults.
    pub cooldown: u32,
}

fn offset(cell: usize, delta: i32) -> usize {
    (cell as i64 + i64::from(delta)) as usize
}

fn dirs(cell: usize, rows: &[&[i32]]) -> Vec<Vec<usize>> {
    rows.iter()
        .map(|row| row.iter().map(|&d| offset(cell, d)).collect())
        .collect()
}

/// `VaultCrossRoom`.
pub(super) fn cross(cell: usize, w: i32) -> SentrySpec {
    SentrySpec {
        cell,
        length: 4.0,
        width: 90.0,
        dirs: dirs(cell, &[&[-1], &[-w], &[1], &[w]]),
        cooldown: 3,
    }
}

/// `VaultCircleRoom`; `variant` is its `Random.Int(3)`.
pub(super) fn circle(cell: usize, w: i32, variant: i32) -> SentrySpec {
    let (width, rows): (f32, Vec<Vec<i32>>) = if variant == 0 {
        (
            90.0,
            [-1, -1 - w, -w, 1 - w, 1, 1 + w, w, w - 1]
                .iter()
                .map(|&d| vec![d])
                .collect(),
        )
    } else {
        (
            45.0,
            [
                -2,
                -2 - w,
                -2 - 2 * w,
                -1 - 2 * w,
                -2 * w,
                1 - 2 * w,
                2 - 2 * w,
                2 - w,
            ]
            .iter()
            .map(|&d| vec![d, -d])
            .collect(),
        )
    };
    let rows: Vec<&[i32]> = rows.iter().map(Vec::as_slice).collect();
    SentrySpec {
        cell,
        length: 4.49,
        width,
        dirs: dirs(cell, &rows),
        cooldown: 2,
    }
}

/// `VaultCircleScanTreasureRoom`.
pub(super) fn circle_scan(cell: usize, w: i32, clockwise: bool) -> SentrySpec {
    let mut deltas = [
        -2,
        -2 - w,
        -2 - 2 * w,
        -1 - 2 * w,
        -2 * w,
        1 - 2 * w,
        2 - 2 * w,
        2 - w,
        2,
        2 + w,
        2 + 2 * w,
        1 + 2 * w,
        2 * w,
        -1 + 2 * w,
        -2 + 2 * w,
        -2 + w,
    ];
    if !clockwise {
        deltas.reverse();
    }
    SentrySpec {
        cell,
        length: 4.49,
        width: 45.0,
        dirs: deltas.iter().map(|&d| vec![offset(cell, d)]).collect(),
        cooldown: 1,
    }
}

/// `VaultManyScansRoom` corner sentry aimed at the room center.
pub(super) fn many_scans(cell: usize, center: usize) -> SentrySpec {
    SentrySpec {
        cell,
        length: 7.0,
        width: 70.0,
        dirs: vec![vec![center]],
        cooldown: 1,
    }
}

/// Cells per scan on the final map. The sentry's own field of view is not
/// modelled; the solid-stopping cone rays already confine scans to its room.
pub(super) fn project(map: &TerrainMap, specs: &[SentrySpec]) -> Vec<MapSentryScan> {
    specs
        .iter()
        .map(|spec| MapSentryScan {
            cell: spec.cell as u32,
            cooldown: spec.cooldown,
            steps: spec
                .dirs
                .iter()
                .map(|targets| scan_cells(map, spec, targets))
                .collect(),
        })
        .collect()
}

fn scan_cells(map: &TerrainMap, spec: &SentrySpec, targets: &[usize]) -> Vec<u32> {
    let mut cells = BTreeSet::new();
    for &target in targets {
        let aim = ballistica(map, spec.cell, target, WONT_STOP);
        let cone = cone_aoe(map, &aim, spec.length, spec.width, STOP_SOLID | STOP_TARGET);
        if cone.is_empty() && aim.path.len() >= 2 {
            cells.insert(aim.path[1]);
        }
        cells.extend(cone);
    }
    cells.into_iter().map(|c| c as u32).collect()
}

#[cfg(test)]
mod tests {
    use crate::level::terrain::{TerrainMap, EMPTY, WALL};
    use crate::run::{dungeon_from_run, init_run};

    #[test]
    fn cross_scans_fan_out_along_each_arm() {
        let mut map = TerrainMap::filled(11, 11, WALL);
        for i in 1..10 {
            map.map[5 + i * 11] = EMPTY;
            map.map[i + 5 * 11] = EMPTY;
        }
        map.recompute_passable();
        let center = 5 + 5 * 11;
        let scans = super::project(&map, &[super::cross(center, 11)]);
        let steps = &scans[0].steps;
        assert_eq!(steps.len(), 4);
        assert_eq!(
            steps[0],
            [(1 + 5 * 11) as u32, 2 + 5 * 11, 3 + 5 * 11, 4 + 5 * 11]
        );
        assert!(steps[1].iter().all(|&c| c % 11 == 5 && c < center as u32));
    }

    #[test]
    fn every_vault_sentry_has_a_scan_cycle_over_open_cells() {
        let mut checked = 0;
        for seed in 0..12 {
            let mut dungeon = dungeon_from_run(init_run(seed));
            dungeon.depth = 17;
            dungeon.imp.spawned = true;
            let generated = super::super::generate(&mut dungeon, true).expect("vault");
            let map = generated.report.map.expect("public vault map");
            let sentries = map
                .mobs
                .iter()
                .filter(|mob| mob.class_name() == "VaultSentry")
                .count();
            assert_eq!(map.sentry_scans.len(), sentries, "seed {seed}");
            for scan in &map.sentry_scans {
                assert!(matches!(scan.steps.len(), 1 | 4 | 8 | 16));
                assert!(matches!(scan.cooldown, 1..=3));
                for step in &scan.steps {
                    assert!(!step.is_empty());
                    assert!(!step.contains(&scan.cell));
                }
                checked += 1;
            }
        }
        assert!(checked > 0, "no sentries across sampled vaults");
    }
}
