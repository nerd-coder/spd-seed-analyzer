//! Region water/grass fill rates and trap class tables.

use crate::level::Feeling;

#[derive(Debug, Clone, Copy)]
pub struct TrapDef {
    pub name: &'static str,
    pub avoids_hallways: bool,
    /// Whether items cannot spawn on this trap (`randomDropCell` filter).
    pub destroys_items: bool,
}

/// `RegularLevel.nTraps()` — main-stream RNG (`NormalIntRange(2, 3 + depth/5)`).
pub fn n_traps(depth: i32) -> i32 {
    crate::random::Random::normal_int_range(2, 3 + depth / 5)
}

/// `(fill, smoothness)` for `setWater`.
pub fn water_params(depth: i32, feeling: Feeling) -> (f32, i32) {
    let water_feel = feeling == Feeling::Water;
    match depth {
        1..=5 => (if water_feel { 0.85 } else { 0.30 }, 5), // sewers
        6..=10 => (if water_feel { 0.90 } else { 0.30 }, 4), // prison
        11..=15 => (if water_feel { 0.85 } else { 0.30 }, 6), // caves
        16..=20 => (if water_feel { 0.90 } else { 0.30 }, 4), // city
        _ => (if water_feel { 0.70 } else { 0.15 }, 6),     // halls
    }
}

/// `(fill, smoothness)` for `setGrass`.
pub fn grass_params(depth: i32, feeling: Feeling) -> (f32, i32) {
    let grass_feel = feeling == Feeling::Grass;
    match depth {
        1..=5 => (if grass_feel { 0.80 } else { 0.20 }, 4),
        6..=10 => (if grass_feel { 0.80 } else { 0.20 }, 3),
        11..=15 => (if grass_feel { 0.65 } else { 0.15 }, 3),
        16..=20 => (if grass_feel { 0.80 } else { 0.20 }, 3),
        _ => (if grass_feel { 0.65 } else { 0.10 }, 3),
    }
}

const fn t(name: &'static str, avoids: bool, destroys: bool) -> TrapDef {
    TrapDef {
        name,
        avoids_hallways: avoids,
        destroys_items: destroys,
    }
}

/// Trap class list + chances for the region / depth.
pub fn trap_table(depth: i32) -> (&'static [TrapDef], &'static [f32]) {
    match depth {
        1 => (DEPTH1_TRAPS, DEPTH1_CHANCES),
        2..=5 => (SEWER_TRAPS, SEWER_CHANCES),
        6..=10 => (PRISON_TRAPS, PRISON_CHANCES),
        11..=15 => (CAVES_TRAPS, CAVES_CHANCES),
        16..=20 => (CITY_TRAPS, CITY_CHANCES),
        _ => (HALLS_TRAPS, HALLS_CHANCES),
    }
}

const DEPTH1_TRAPS: &[TrapDef] = &[t("WornDartTrap", true, false)];
const DEPTH1_CHANCES: &[f32] = &[1.0];
const SEWER_CHANCES: &[f32] = &[4., 4., 4., 4., 2., 2., 1., 1., 1., 1., 1.];
const PRISON_CHANCES: &[f32] = &[4., 4., 4., 4., 4., 2., 2., 2., 1., 1., 1., 1., 1., 1.];
const CAVES_CHANCES: &[f32] = &[4., 4., 4., 4., 4., 2., 2., 2., 1., 1., 1., 1., 1., 1.];
const CITY_CHANCES: &[f32] = &[
    4., 4., 4., 4., 4., 2., 2., 2., 2., 1., 1., 1., 1., 1., 1., 1., 1.,
];
const HALLS_CHANCES: &[f32] = &[
    4., 4., 4., 4., 4., 2., 2., 2., 2., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
];

// destroys_items: Burning, Blazing, Chilling, Frost, Explosive, Disintegration, Pitfall
const SEWER_TRAPS: &[TrapDef] = &[
    t("ChillingTrap", false, true),
    t("ShockingTrap", false, false),
    t("ToxicTrap", false, false),
    t("WornDartTrap", true, false),
    t("AlarmTrap", false, false),
    t("OozeTrap", false, false),
    t("ConfusionTrap", false, false),
    t("FlockTrap", false, false),
    t("SummoningTrap", false, false),
    t("TeleportationTrap", false, false),
    t("GatewayTrap", true, false),
];

const PRISON_TRAPS: &[TrapDef] = &[
    t("ChillingTrap", false, true),
    t("ShockingTrap", false, false),
    t("ToxicTrap", false, false),
    t("BurningTrap", false, true),
    t("PoisonDartTrap", true, false),
    t("AlarmTrap", false, false),
    t("OozeTrap", false, false),
    t("GrippingTrap", true, false),
    t("ConfusionTrap", false, false),
    t("FlockTrap", false, false),
    t("SummoningTrap", false, false),
    t("TeleportationTrap", false, false),
    t("GatewayTrap", true, false),
    t("GeyserTrap", false, false),
];

const CAVES_TRAPS: &[TrapDef] = &[
    t("BurningTrap", false, true),
    t("PoisonDartTrap", true, false),
    t("FrostTrap", false, true),
    t("StormTrap", false, false),
    t("CorrosionTrap", false, false),
    t("GrippingTrap", true, false),
    t("RockfallTrap", true, false),
    t("GuardianTrap", false, false),
    t("ConfusionTrap", false, false),
    t("SummoningTrap", false, false),
    t("WarpingTrap", false, false),
    t("PitfallTrap", false, true),
    t("GatewayTrap", true, false),
    t("GeyserTrap", false, false),
];

const CITY_TRAPS: &[TrapDef] = &[
    t("FrostTrap", false, true),
    t("StormTrap", false, false),
    t("CorrosionTrap", false, false),
    t("BlazingTrap", false, true),
    t("DisintegrationTrap", true, true),
    t("RockfallTrap", true, false),
    t("FlashingTrap", true, false),
    t("GuardianTrap", false, false),
    t("WeakeningTrap", false, false),
    t("DisarmingTrap", false, false),
    t("SummoningTrap", false, false),
    t("WarpingTrap", false, false),
    t("CursingTrap", false, false),
    t("PitfallTrap", false, true),
    t("DistortionTrap", false, false),
    t("GatewayTrap", true, false),
    t("GeyserTrap", false, false),
];

const HALLS_TRAPS: &[TrapDef] = &[
    t("FrostTrap", false, true),
    t("StormTrap", false, false),
    t("CorrosionTrap", false, false),
    t("BlazingTrap", false, true),
    t("DisintegrationTrap", true, true),
    t("RockfallTrap", true, false),
    t("FlashingTrap", true, false),
    t("GuardianTrap", false, false),
    t("WeakeningTrap", false, false),
    t("DisarmingTrap", false, false),
    t("SummoningTrap", false, false),
    t("WarpingTrap", false, false),
    t("CursingTrap", false, false),
    t("GrimTrap", true, false),
    t("PitfallTrap", false, true),
    t("DistortionTrap", false, false),
    t("GatewayTrap", true, false),
    t("GeyserTrap", false, false),
];
