//! Initial `Mob.state` for mobs placed during level creation.
//!
//! Pinned `Mob.state` defaults to `SLEEPING`; `RegularLevel.createMobs` and
//! the room painters that place map mobs keep each class's initializer.

use crate::report::MapMobState;

pub(crate) fn initial_state(class_name: &str) -> MapMobState {
    match class_name {
        // `state = PASSIVE` initializers, including every `NPC` subclass.
        "Statue" | "ArmoredStatue" | "Mimic" | "GoldenMimic" | "EbonyMimic" | "CrystalMimic"
        | "Pylon" | "CrystalSpire" | "FungalCore" | "GnollExile" | "RotHeart" | "DemonSpawner"
        | "NPC" | "Shopkeeper" | "ImpShopkeeper" | "Wandmaker" | "Blacksmith" | "Imp"
        | "Sentry" | "Sheep" => MapMobState::Passive,
        // `state = WANDERING` initializers; `RotLasher` and `FungalSentry`
        // replace `WANDERING` with their stationary `Waiting` state.
        "Ghost" | "Bee" | "FetidRat" | "GnollTrickster" | "GreatCrab" | "Wraith" | "RotLasher"
        | "FungalSentry" => MapMobState::Wandering,
        _ => MapMobState::Sleeping,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_mobs_keep_class_initial_state() {
        assert_eq!(initial_state("Rat"), MapMobState::Sleeping);
        assert_eq!(initial_state("Piranha"), MapMobState::Sleeping);
        assert_eq!(initial_state("Skeleton"), MapMobState::Sleeping);
        assert_eq!(initial_state("Mimic"), MapMobState::Passive);
        assert_eq!(initial_state("Shopkeeper"), MapMobState::Passive);
        assert_eq!(initial_state("Statue"), MapMobState::Passive);
        assert_eq!(initial_state("Ghost"), MapMobState::Wandering);
        assert_eq!(initial_state("RotLasher"), MapMobState::Wandering);
    }
}
