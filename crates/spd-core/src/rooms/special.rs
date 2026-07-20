//! Port of `SpecialRoom.initForRun` RNG / deck setup.

use crate::random::Random;

/// `EQUIP_SPECIALS` class simple names, declaration order.
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

/// `CONSUMABLE_SPECIALS` class simple names, declaration order.
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

/// Returns the ordered `runSpecials` deck for the run.
pub fn init_for_run() -> Vec<&'static str> {
    let mut run_equip: Vec<&str> = EQUIP_SPECIALS.to_vec();
    let mut run_cons: Vec<&str> = CONSUMABLE_SPECIALS.to_vec();

    Random::shuffle(&mut run_equip);
    Random::shuffle(&mut run_cons);

    let mut run_specials: Vec<&str> = Vec::new();
    // Always a consumable special first
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
