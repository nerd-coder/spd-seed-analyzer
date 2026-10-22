//! `ChampionEnemy.rollForChampion` as `RegularLevel.createMobs` sees it.

use super::rotation::MobKind;

/// `ChampionEnemy.rollForChampion` maps roll 3 to `Giant`, which is `LARGE`.
const GIANT_ROLL: i32 = 3;

/// `Dungeon.mobsToChampion` during one floor's population.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ChampionCounter {
    /// Champion Enemies is off: no mob gains a buff.
    Off,
    /// The counter is known. `Dungeon.init` sets it to 1 before floor 1.
    Known(f32),
    /// Runtime spawns on earlier floors also decrement it.
    Unknown,
}

/// How a rolled mob's champion buff affects its placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GiantState {
    No,
    Yes,
    /// Giant when the unknown counter has run out.
    Maybe,
}

impl ChampionCounter {
    /// Fresh-run counter for `depth` when `enabled`.
    pub(crate) fn for_floor(enabled: bool, depth: i32) -> Self {
        match (enabled, depth) {
            (false, _) => Self::Off,
            (true, 1) => Self::Known(1.0),
            (true, _) => Self::Unknown,
        }
    }

    /// Mirrors `rollForChampion` after its constant `Random.Int(6)` draw.
    pub(super) fn roll(&mut self, mob: MobKind, roll: i32, depth: i32) -> GiantState {
        let giant = roll == GIANT_ROLL;
        match self {
            Self::Off => GiantState::No,
            Self::Unknown => {
                if giant && !blocked(mob, depth) {
                    GiantState::Maybe
                } else {
                    GiantState::No
                }
            }
            Self::Known(counter) => {
                *counter -= 1.0;
                if *counter > 0.0 || blocked(mob, depth) {
                    return GiantState::No;
                }
                *counter += 8.0 - (depth - 1).min(20) as f32 / 10.0;
                if giant {
                    GiantState::Yes
                } else {
                    GiantState::No
                }
            }
        }
    }
}

/// Standout enemies that `rollForChampion` never buffs on early floors.
fn blocked(mob: MobKind, depth: i32) -> bool {
    use MobKind::*;
    match mob {
        Crab | HermitCrab => depth <= 3,
        Thief | Bandit => depth <= 4,
        Guard => depth <= 7,
        Bat => depth <= 9,
        _ => false,
    }
}
