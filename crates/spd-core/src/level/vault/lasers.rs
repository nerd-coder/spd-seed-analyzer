//! `VaultLaser` firing schedules and beam paths.

use crate::level::ballistica::{ballistica, STOP_SOLID};
use crate::level::terrain::TerrainMap;
use crate::report::MapLaserBeam;

/// The fields a vault room sets on a `VaultLaser` that fires. Visual-only
/// lasers (`curCooldown = Integer.MAX_VALUE`) never fire and have no spec.
#[derive(Debug, Clone)]
pub(super) struct LaserSpec {
    pub cell: usize,
    /// `laserDirs[0]`; every vault laser has a single direction.
    pub target: usize,
    /// Initial `curCooldown`.
    pub cooldown: u32,
    /// `afterShotCooldown`.
    pub interval: u32,
    /// `shotsAfterCooldown`.
    pub shots: u32,
    /// `giveWarning`.
    pub warning: bool,
}

impl LaserSpec {
    /// `VaultLasersRoom`: one shot per cycle, with a targeted-cell warning.
    pub(super) fn room(cell: usize, target: usize, cooldown: i32, interval: i32) -> Self {
        Self {
            cell,
            target,
            cooldown: cooldown as u32,
            interval: interval as u32,
            shots: 1,
            warning: true,
        }
    }

    /// Treasure-room gauntlets: staggered cooldowns, no warning.
    pub(super) fn gauntlet(
        cell: usize,
        target: usize,
        cooldown: i32,
        interval: u32,
        shots: u32,
    ) -> Self {
        Self {
            cell,
            target,
            cooldown: cooldown as u32,
            interval,
            shots,
            warning: false,
        }
    }
}

/// `new Ballistica(pos, laserDirs[0], STOP_SOLID).subPath(1, dist)` on the final map.
pub(super) fn project(map: &TerrainMap, specs: &[LaserSpec]) -> Vec<MapLaserBeam> {
    specs
        .iter()
        .map(|spec| {
            let beam = ballistica(map, spec.cell, spec.target, STOP_SOLID);
            MapLaserBeam {
                cell: spec.cell as u32,
                cooldown: spec.cooldown,
                interval: spec.interval,
                shots: spec.shots,
                warning: spec.warning,
                path: beam
                    .sub_path(1, beam.dist)
                    .iter()
                    .map(|&c| c as u32)
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::run::{dungeon_from_run, init_run};

    #[test]
    fn firing_lasers_cross_open_cells_on_a_schedule() {
        let mut checked = 0;
        for seed in 0..40 {
            let mut dungeon = dungeon_from_run(init_run(seed));
            dungeon.depth = 17;
            dungeon.imp.spawned = true;
            let generated = super::super::generate(&mut dungeon, true).expect("vault");
            let map = generated.report.map.expect("public vault map");
            let lasers = map
                .mobs
                .iter()
                .filter(|mob| mob.class_name() == "VaultLaser")
                .count();
            assert!(map.laser_beams.len() <= lasers, "seed {seed}");
            for beam in &map.laser_beams {
                assert!(beam.cooldown >= 1, "seed {seed}");
                if beam.warning {
                    assert!((7..=9).contains(&beam.interval));
                    assert!(beam.cooldown <= beam.interval);
                    assert_eq!(beam.shots, 1);
                } else {
                    assert!(matches!((beam.interval, beam.shots), (3, 1) | (2, 6)));
                }
                assert!(!beam.path.is_empty());
                assert!(!beam.path.contains(&beam.cell));
                checked += 1;
            }
        }
        assert!(checked > 0, "no lasers across sampled vaults");
    }
}
