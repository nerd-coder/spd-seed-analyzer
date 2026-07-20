//! Run initialization sequence from `Dungeon.init()`.

use crate::dungeon::{DungeonState, LimitedDrops};
use crate::generator::{self, GeneratorState};
use crate::items::{IdentityMaps, init_identities};
use crate::random::Random;
use crate::rooms::{self, RoomRunState};

/// Full run state produced at dungeon init (seed+1 generator).
#[derive(Debug, Clone)]
pub struct RunState {
    pub seed: i64,
    pub identities: IdentityMaps,
    pub rooms: RoomRunState,
    pub generator: GeneratorState,
}

/// Mirrors:
/// ```text
/// Random.pushGenerator(seed + 1);
/// Scroll.initLabels(); Potion.initColors(); Ring.initGems();
/// SpecialRoom.initForRun(); SecretRoom.initForRun();
/// Generator.fullReset();
/// Random.resetGenerators();
/// ```
pub fn init_run(seed: i64) -> RunState {
    Random::reset_generators();
    Random::push_generator_seeded(seed.wrapping_add(1));

    let identities = init_identities();
    let rooms = rooms::init_rooms_for_run();
    let generator = generator::full_reset();

    Random::reset_generators();

    RunState {
        seed,
        identities,
        rooms,
        generator,
    }
}

/// Build a mutable dungeon context from a finished run init.
pub fn dungeon_from_run(run: RunState) -> DungeonState {
    DungeonState {
        seed: run.seed,
        depth: 1,
        branch: 0,
        challenges: 0,
        identities: run.identities,
        rooms: run.rooms,
        generator: run.generator,
        limited: LimitedDrops::reset(),
        items_to_spawn: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identities_deterministic_for_seed() {
        let a = init_run(12345);
        let b = init_run(12345);
        assert_eq!(a.identities, b.identities);
        assert_eq!(a.rooms.specials, b.rooms.specials);
        assert_eq!(a.rooms.secrets, b.rooms.secrets);
        assert_eq!(a.rooms.region_secrets, b.rooms.region_secrets);
        assert_eq!(a.generator.using_first_deck(), b.generator.using_first_deck());
    }

    #[test]
    fn identities_cover_all_labels() {
        let run = init_run(1);
        assert_eq!(run.identities.potions.len(), 12);
        assert_eq!(run.identities.scrolls.len(), 12);
        assert_eq!(run.identities.rings.len(), 12);

        let mut colors: Vec<_> = run
            .identities
            .potions
            .iter()
            .map(|e| e.appearance.clone())
            .collect();
        colors.sort();
        let mut expected = crate::items::catalog::POTION_COLORS.to_vec();
        expected.sort();
        assert_eq!(colors, expected);
    }

    #[test]
    fn different_seeds_usually_differ() {
        let a = init_run(1);
        let b = init_run(2);
        assert_ne!(a.identities, b.identities);
    }
}
