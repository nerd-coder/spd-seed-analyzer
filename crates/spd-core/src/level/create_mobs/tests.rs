use crate::level::create_level_partial;
use crate::run::{dungeon_from_run, init_run};

use super::rotation::{standard_rotation, MobKind};
use super::{mob_limit, next_mob, Random};

const LCG_MULTIPLIER: u64 = 0x5DEECE66D;
const LCG_ADDEND: u64 = 0xB;
const LCG_MASK: u64 = (1 << 48) - 1;

fn recover_state_after_first_int(probe: &[i32]) -> u64 {
    let high = probe[0] as u32 as u64;
    let next_high = probe[1] as u32 as u64;
    (0..=u16::MAX)
        .map(|low| (high << 16) | u64::from(low))
        .find(|&state| {
            ((state.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_ADDEND) & LCG_MASK) >> 16)
                == next_high
        })
        .expect("two consecutive nextInt values identify the 48-bit LCG state")
}

fn raw_lcg_steps(from: u64, to: u64, limit: usize) -> Option<usize> {
    let mut state = from;
    for steps in 0..=limit {
        if state == to {
            return Some(steps);
        }
        state = state.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_ADDEND) & LCG_MASK;
    }
    None
}

#[test]
fn depth_one_builder_painter_and_mob_boundaries_match_java() {
    let run = init_run(0);
    let mut dungeon = dungeon_from_run(run);
    dungeon.depth = 1;
    let level = create_level_partial(&mut dungeon);
    let rust_paint = recover_state_after_first_int(&level.pre_paint_rng_probe);
    let rust_pre = recover_state_after_first_int(&level.pre_mobs_rng_probe);
    let rust_post = recover_state_after_first_int(&level.pre_items_rng_probe);

    let java_paint = recover_state_after_first_int(&[
        1993374861,
        -149591753,
        -1380055091,
        368556635,
        1163123780,
        1118396506,
        798076888,
        -791762655,
    ]);
    let java_pre = recover_state_after_first_int(&[
        1726373121,
        -188171336,
        1867201434,
        -2048184778,
        717498613,
        -493803451,
        -1465937696,
        689471909,
    ]);
    let java_post = recover_state_after_first_int(&[
        -339886649,
        -1704611306,
        -1853649770,
        -1054644975,
        1614102425,
        -1139121137,
        -821014263,
        -184470637,
    ]);

    // Builder placement must arrive at the exact Java painter boundary.
    assert_eq!(rust_paint, java_paint);
    assert_eq!(raw_lcg_steps(java_paint, java_pre, 512), Some(190));
    assert_eq!(raw_lcg_steps(rust_paint, rust_pre, 512), Some(190));
    assert_eq!(rust_pre, java_pre);

    // The new depth-one createMobs pass itself has the exact Java draw shape.
    assert_eq!(raw_lcg_steps(rust_pre, rust_post, 512), Some(116));
    assert_eq!(raw_lcg_steps(java_pre, java_post, 512), Some(116));
    assert_eq!(rust_post, java_post);
}

fn rotation_labels(depth: i32, len: usize) -> Vec<&'static str> {
    Random::reset_generators();
    Random::push_generator_seeded(1);
    let mut rotation = Vec::new();
    let mut labels: Vec<_> = (0..len)
        .map(|_| next_mob(depth, &mut rotation).label())
        .collect();
    labels.sort_unstable();
    Random::reset_generators();
    labels
}

#[test]
fn prison_floor_seven_and_eight_rotations_match_the_pin() {
    assert_eq!(
        rotation_labels(7, 6),
        ["DM100", "Guard", "Skeleton", "Skeleton", "Skeleton", "Thief"]
    );
    assert_eq!(
        rotation_labels(8, 8),
        [
            "DM100",
            "DM100",
            "Guard",
            "Guard",
            "Necromancer",
            "Skeleton",
            "Skeleton",
            "Thief",
        ]
    );
}

fn rotation_families(depth: i32) -> Vec<&'static str> {
    Random::reset_generators();
    Random::push_generator_seeded(1);
    let mut labels: Vec<_> = standard_rotation(depth)
        .into_iter()
        .map(|mob| match mob {
            MobKind::RedShaman | MobKind::BlueShaman | MobKind::PurpleShaman => "Shaman",
            MobKind::FireElemental
            | MobKind::FrostElemental
            | MobKind::ShockElemental
            | MobKind::ChaosElemental => "Elemental",
            other => other.label(),
        })
        .collect();
    labels.sort_unstable();
    Random::reset_generators();
    labels
}

#[test]
fn region_rotation_tables_match_mob_spawner_through_floor_twenty_four() {
    assert_eq!(
        rotation_families(11),
        ["Bat", "Bat", "Bat", "Brute", "Shaman"]
    );
    assert_eq!(
        rotation_families(14),
        ["Bat", "Brute", "DM200", "DM200", "Shaman", "Shaman", "Spinner", "Spinner"]
    );
    assert_eq!(
        rotation_families(16),
        ["Elemental", "Ghoul", "Ghoul", "Ghoul", "Warlock"]
    );
    assert_eq!(
        rotation_families(19),
        [
            "Elemental",
            "Golem",
            "Golem",
            "Golem",
            "Monk",
            "Monk",
            "Warlock",
            "Warlock",
        ]
    );
    assert_eq!(rotation_families(21), ["Eye", "Succubus", "Succubus"]);
    assert_eq!(
        rotation_families(24),
        ["Eye", "Eye", "Scorpio", "Scorpio", "Scorpio", "Succubus"]
    );
}

#[test]
fn only_pinned_large_ambient_classes_require_open_space() {
    assert!(MobKind::Dm200.is_large());
    assert!(MobKind::Dm201.is_large());
    assert!(MobKind::Golem.is_large());
    for mob in [
        MobKind::Brute,
        MobKind::ArmoredBrute,
        MobKind::Eye,
        MobKind::Scorpio,
        MobKind::Acidic,
    ] {
        assert!(!mob.is_large(), "{} is not LARGE in the pin", mob.label());
    }
}

#[test]
fn regular_mob_limit_uses_pinned_large_feeling_rounding() {
    Random::reset_generators();
    Random::push_generator_seeded(7);
    let normal = mob_limit(14, false);
    Random::reset_generators();
    Random::push_generator_seeded(7);
    let large = mob_limit(14, true);
    assert_eq!(large, (normal as f32 * 1.33).ceil() as i32);
    assert_eq!(mob_limit(1, true), 8);
    Random::reset_generators();
}
