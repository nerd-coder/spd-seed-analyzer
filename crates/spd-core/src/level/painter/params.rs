//! Region water/grass fill rates and trap class tables.

use crate::level::Feeling;

#[derive(Debug, Clone, Copy)]
pub struct TrapDef {
    pub name: &'static str,
    pub avoids_hallways: bool,
    /// Whether items cannot spawn on this trap (`randomDropCell` filter).
    pub destroys_items: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrapMetadata {
    pub color: u8,
    pub shape: u8,
    pub can_be_hidden: bool,
}

/// Pinned `Trap` constructor fields used by terrain and feature rendering.
pub fn trap_metadata(name: &str) -> Option<TrapMetadata> {
    let (color, shape) = match name {
        "AlarmTrap" => (0, 0),
        "BlazingTrap" => (1, 3),
        "BurningTrap" => (1, 0),
        "ChillingTrap" => (6, 0),
        "ConfusionTrap" => (4, 2),
        "CorrosionTrap" => (7, 2),
        "CursingTrap" => (5, 1),
        "DisarmingTrap" => (0, 6),
        "DisintegrationTrap" => (5, 5),
        "DistortionTrap" => (4, 6),
        "ExplosiveTrap" => (1, 4),
        "FlashingTrap" => (7, 3),
        "FlockTrap" => (6, 1),
        "FrostTrap" => (6, 3),
        "GatewayTrap" => (4, 5),
        "GeyserTrap" => (4, 4),
        "GrimTrap" => (7, 6),
        "GrippingTrap" => (7, 0),
        "GuardianTrap" => (0, 3),
        "OozeTrap" => (3, 0),
        "PitfallTrap" => (0, 4),
        "PoisonDartTrap" => (3, 5),
        "RockfallTrap" => (7, 4),
        "ShockingTrap" => (2, 0),
        "StormTrap" => (2, 3),
        "SummoningTrap" => (4, 1),
        "TeleportationTrap" => (4, 0),
        "ToxicTrap" => (3, 2),
        "WarpingTrap" => (4, 3),
        "WeakeningTrap" => (3, 1),
        "WornDartTrap" => (7, 5),
        _ => return None,
    };
    Some(TrapMetadata {
        color,
        shape,
        can_be_hidden: !matches!(
            name,
            "DisintegrationTrap" | "GrimTrap" | "PoisonDartTrap" | "RockfallTrap" | "WornDartTrap"
        ),
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worn_dart_visuals_and_visibility_match_pinned_trap_constructor() {
        assert_eq!(
            trap_metadata("WornDartTrap"),
            Some(TrapMetadata {
                color: 7,
                shape: 5,
                can_be_hidden: false,
            })
        );
    }
}
