//! Pinned `MobSpawner` rotations and mob-construction RNG side effects.

use crate::random::Random;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MobKind {
    Rat,
    Albino,
    Snake,
    Gnoll,
    GnollExile,
    Swarm,
    Crab,
    HermitCrab,
    Slime,
    CausticSlime,
    Skeleton,
    Thief,
    Bandit,
    Dm100,
    Guard,
    Necromancer,
    SpectralNecromancer,
    Bat,
    Brute,
    ArmoredBrute,
    RedShaman,
    BlueShaman,
    PurpleShaman,
    Spinner,
    Dm200,
    Dm201,
    Ghoul,
    FireElemental,
    FrostElemental,
    ShockElemental,
    ChaosElemental,
    Warlock,
    Monk,
    Senior,
    Golem,
    Succubus,
    Eye,
    Scorpio,
    Acidic,
}

impl MobKind {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Rat => "Rat",
            Self::Albino => "Albino",
            Self::Snake => "Snake",
            Self::Gnoll => "Gnoll",
            Self::GnollExile => "GnollExile",
            Self::Swarm => "Swarm",
            Self::Crab => "Crab",
            Self::HermitCrab => "HermitCrab",
            Self::Slime => "Slime",
            Self::CausticSlime => "CausticSlime",
            Self::Skeleton => "Skeleton",
            Self::Thief => "Thief",
            Self::Bandit => "Bandit",
            Self::Dm100 => "DM100",
            Self::Guard => "Guard",
            Self::Necromancer => "Necromancer",
            Self::SpectralNecromancer => "SpectralNecromancer",
            Self::Bat => "Bat",
            Self::Brute => "Brute",
            Self::ArmoredBrute => "ArmoredBrute",
            Self::RedShaman => "RedShaman",
            Self::BlueShaman => "BlueShaman",
            Self::PurpleShaman => "PurpleShaman",
            Self::Spinner => "Spinner",
            Self::Dm200 => "DM200",
            Self::Dm201 => "DM201",
            Self::Ghoul => "Ghoul",
            Self::FireElemental => "FireElemental",
            Self::FrostElemental => "FrostElemental",
            Self::ShockElemental => "ShockElemental",
            Self::ChaosElemental => "ChaosElemental",
            Self::Warlock => "Warlock",
            Self::Monk => "Monk",
            Self::Senior => "Senior",
            Self::Golem => "Golem",
            Self::Succubus => "Succubus",
            Self::Eye => "Eye",
            Self::Scorpio => "Scorpio",
            Self::Acidic => "Acidic",
        }
    }

    pub(super) fn is_large(self) -> bool {
        matches!(self, Self::Dm200 | Self::Dm201 | Self::Golem)
    }
}

pub(super) fn next_mob(depth: i32, rotation: &mut Vec<MobKind>) -> MobKind {
    if rotation.is_empty() {
        *rotation = standard_rotation(depth);
        add_rare_mob(depth, rotation);

        // `swapMobAlts` rolls for every slot, including classes which have no
        // alternate. Keeping that apparently redundant draw is required before
        // the list shuffle.
        for mob in &mut *rotation {
            if Random::float() < 1.0 / 50.0 {
                *mob = rare_alt(*mob);
            }
        }
        Random::shuffle_list(rotation);
    }

    let mob = rotation.remove(0);
    burn_constructor_draws(mob);
    // `ChampionEnemy.rollForChampion` always rolls, even when the challenge is
    // disabled and no champion can be attached.
    let _ = Random::int_max(6);
    mob
}

pub(super) fn standard_rotation(depth: i32) -> Vec<MobKind> {
    use MobKind::*;
    match depth {
        2 => vec![Rat, Rat, Snake, Gnoll, Gnoll],
        3 => vec![Rat, Snake, Gnoll, Gnoll, Gnoll, Swarm, Crab],
        4 | 5 => vec![Gnoll, Swarm, Crab, Crab, Slime, Slime],
        6 => vec![Skeleton, Skeleton, Skeleton, Thief, Swarm],
        7 => vec![Skeleton, Skeleton, Skeleton, Thief, Dm100, Guard],
        8 => vec![
            Skeleton,
            Skeleton,
            Thief,
            Dm100,
            Dm100,
            Guard,
            Guard,
            Necromancer,
        ],
        9 | 10 => vec![
            Skeleton,
            Thief,
            Dm100,
            Dm100,
            Guard,
            Guard,
            Necromancer,
            Necromancer,
        ],
        11 => vec![Bat, Bat, Bat, Brute, random_shaman()],
        12 => vec![Bat, Bat, Brute, Brute, random_shaman(), Spinner],
        13 => vec![
            Bat,
            Brute,
            Brute,
            random_shaman(),
            random_shaman(),
            Spinner,
            Spinner,
            Dm200,
        ],
        14 | 15 => vec![
            Bat,
            Brute,
            random_shaman(),
            random_shaman(),
            Spinner,
            Spinner,
            Dm200,
            Dm200,
        ],
        16 => vec![Ghoul, Ghoul, Ghoul, random_elemental(), Warlock],
        17 => vec![Ghoul, random_elemental(), random_elemental(), Warlock, Monk],
        18 => vec![
            Ghoul,
            random_elemental(),
            Warlock,
            Warlock,
            Monk,
            Monk,
            Golem,
        ],
        19 | 20 => vec![
            random_elemental(),
            Warlock,
            Warlock,
            Monk,
            Monk,
            Golem,
            Golem,
            Golem,
        ],
        21 => vec![Succubus, Succubus, Eye],
        22 => vec![Succubus, Eye],
        23 => vec![Succubus, Eye, Eye, Scorpio],
        24..=26 => vec![Succubus, Eye, Eye, Scorpio, Scorpio, Scorpio],
        // The pinned switch defaults to the depth-one sewer rotation.
        _ => vec![Rat, Rat, Rat, Snake],
    }
}

fn random_shaman() -> MobKind {
    let roll = Random::float();
    if roll < 0.4 {
        MobKind::RedShaman
    } else if roll < 0.8 {
        MobKind::BlueShaman
    } else {
        MobKind::PurpleShaman
    }
}

fn random_elemental() -> MobKind {
    // `Elemental.random` has a short-circuit: Chaos consumes no subtype roll.
    if Random::float() < 1.0 / 50.0 {
        return MobKind::ChaosElemental;
    }
    let roll = Random::float();
    if roll < 0.4 {
        MobKind::FireElemental
    } else if roll < 0.8 {
        MobKind::FrostElemental
    } else {
        MobKind::ShockElemental
    }
}

fn add_rare_mob(depth: i32, rotation: &mut Vec<MobKind>) {
    let rare = match depth {
        4 => Some(MobKind::Thief),
        9 => Some(MobKind::Bat),
        14 => Some(MobKind::Ghoul),
        19 => Some(MobKind::Succubus),
        _ => None,
    };
    if let Some(rare) = rare {
        if Random::float() < 0.025 {
            rotation.push(rare);
        }
    }
}

fn rare_alt(mob: MobKind) -> MobKind {
    match mob {
        MobKind::Rat => MobKind::Albino,
        MobKind::Gnoll => MobKind::GnollExile,
        MobKind::Crab => MobKind::HermitCrab,
        MobKind::Slime => MobKind::CausticSlime,
        MobKind::Thief => MobKind::Bandit,
        MobKind::Necromancer => MobKind::SpectralNecromancer,
        MobKind::Brute => MobKind::ArmoredBrute,
        MobKind::Dm200 => MobKind::Dm201,
        MobKind::Monk => MobKind::Senior,
        MobKind::Scorpio => MobKind::Acidic,
        other => other,
    }
}

fn burn_constructor_draws(mob: MobKind) {
    match mob {
        // Instance initializers select their carried loot category.
        MobKind::Thief | MobKind::Bandit | MobKind::Dm200 | MobKind::Dm201 | MobKind::Golem => {
            let _ = Random::int_max(2);
        }
        // The Elemental base initializer rolls `rangedCooldown` before the
        // champion roll. `NormalIntRange` consumes two Java floats.
        MobKind::FireElemental
        | MobKind::FrostElemental
        | MobKind::ShockElemental
        | MobKind::ChaosElemental => {
            let _ = Random::normal_int_range(3, 5);
        }
        _ => {}
    }
}
