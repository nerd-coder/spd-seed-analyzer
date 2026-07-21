//! Port of `SpecialRoom` run/floor init and `createRoom`.

use crate::random::Random;

use super::types::RoomSpec;

/// `EQUIP_SPECIALS` declaration order.
const EQUIP_SPECIALS: &[&str] = &[
    "WeakFloorRoom",
    "CryptRoom",
    "PoolRoom",
    "ArmoryRoom",
    "SentryRoom",
    "StatueRoom",
    "CrystalVaultRoom",
    "CrystalChoiceRoom",
    "SacrificeRoom",
];

/// `CONSUMABLE_SPECIALS` declaration order.
const CONSUMABLE_SPECIALS: &[&str] = &[
    "RunestoneRoom",
    "GardenRoom",
    "LibraryRoom",
    "StorageRoom",
    "TreasuryRoom",
    "MagicWellRoom",
    "ToxicGasRoom",
    "MagicalFireRoom",
    "TrapsRoom",
    "CrystalPathRoom",
];

const CRYSTAL_KEY_SPECIALS: &[&str] = &[
    "PitRoom",
    "CrystalVaultRoom",
    "CrystalChoiceRoom",
    "CrystalPathRoom",
];

const POTION_SPAWN_ROOMS: &[&str] = &[
    "PoolRoom",
    "SentryRoom",
    "StorageRoom",
    "ToxicGasRoom",
    "MagicalFireRoom",
    "TrapsRoom",
];

/// Returns the ordered `runSpecials` deck for the run.
pub fn init_for_run() -> Vec<&'static str> {
    let mut run_equip: Vec<&str> = EQUIP_SPECIALS.to_vec();
    let mut run_cons: Vec<&str> = CONSUMABLE_SPECIALS.to_vec();

    Random::shuffle(&mut run_equip);
    Random::shuffle(&mut run_cons);

    let mut run_specials: Vec<&str> = Vec::new();
    run_specials.push(run_cons.remove(0));

    while !run_equip.is_empty() || !run_cons.is_empty() {
        if !run_equip.is_empty() {
            run_specials.push(run_equip.remove(0));
        }
        if !run_cons.is_empty() {
            run_specials.push(run_cons.remove(0));
        }
    }

    run_specials
}

#[derive(Debug, Clone)]
pub struct SpecialFloorState {
    pub floor_specials: Vec<&'static str>,
    pub pit_needed_depth: i32,
}

impl SpecialFloorState {
    /// `SpecialRoom.initForFloor` (+ optional lab room at front).
    pub fn init_for_floor(
        run_specials: &[&'static str],
        lab_needed: bool,
        lab_room_count: &mut i32,
    ) -> Self {
        let mut floor_specials = run_specials.to_vec();
        if lab_needed {
            *lab_room_count += 1;
            floor_specials.insert(0, "LaboratoryRoom");
        }
        Self {
            floor_specials,
            pit_needed_depth: -1,
        }
    }

    fn use_type(&mut self, run_specials: &mut Vec<&'static str>, type_name: &'static str) {
        self.floor_specials.retain(|t| *t != type_name);
        if CRYSTAL_KEY_SPECIALS.contains(&type_name) {
            self.floor_specials
                .retain(|t| !CRYSTAL_KEY_SPECIALS.contains(t));
        }
        if POTION_SPAWN_ROOMS.contains(&type_name) {
            self.floor_specials
                .retain(|t| !POTION_SPAWN_ROOMS.contains(t));
        }
        if let Some(pos) = run_specials.iter().position(|t| *t == type_name) {
            let t = run_specials.remove(pos);
            run_specials.push(t);
        }
    }

    /// `SpecialRoom.createRoom()`.
    pub fn create_room(
        &mut self,
        run_specials: &mut Vec<&'static str>,
        depth: i32,
        boss_next: bool,
    ) -> RoomSpec {
        if depth == self.pit_needed_depth {
            self.pit_needed_depth = -1;
            self.use_type(run_specials, "PitRoom");
            return RoomSpec::special("PitRoom");
        }

        if self.floor_specials.iter().any(|t| *t == "LaboratoryRoom") {
            self.use_type(run_specials, "LaboratoryRoom");
            return RoomSpec::special("LaboratoryRoom");
        }

        if boss_next {
            self.floor_specials.retain(|t| *t != "WeakFloorRoom");
        }

        let mut index = Random::chances(&[6., 3., 1.]);
        while index >= self.floor_specials.len() as i32 {
            index -= 1;
        }
        if index < 0 {
            index = 0;
        }
        let name = self.floor_specials[index as usize];
        if name == "WeakFloorRoom" {
            self.pit_needed_depth = depth + 1;
        }
        self.use_type(run_specials, name);
        RoomSpec::special(name)
    }
}
