//! `VaultLevel` mob tiers and the `createMob` constructor RNG.

use crate::random::Random;

pub(super) const T1_MOBS: [VaultMob; 2] = [VaultMob::Skeleton, VaultMob::Dm100];
pub(super) const T2_MOBS: [VaultMob; 3] = [VaultMob::Shaman, VaultMob::Dm200, VaultMob::Ghoul];
pub(super) const T3_MOBS: [VaultMob; 2] = [VaultMob::Elemental, VaultMob::Golem];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VaultMob {
    Skeleton,
    Dm100,
    Shaman,
    Dm200,
    Ghoul,
    /// Queued `VaultElemental.class` before `VaultElemental.random()`.
    Elemental,
    FireElemental,
    FrostElemental,
    ShockElemental,
    Golem,
}

impl VaultMob {
    pub(super) fn large(self) -> bool {
        matches!(self, Self::Golem | Self::Dm200)
    }

    pub(super) fn tier(self) -> usize {
        match self {
            Self::Skeleton | Self::Dm100 => 1,
            Self::Shaman | Self::Dm200 | Self::Ghoul => 2,
            Self::Elemental
            | Self::FireElemental
            | Self::FrostElemental
            | Self::ShockElemental
            | Self::Golem => 3,
        }
    }

    /// Public mob class. `VaultElemental.Fire` and its siblings report as the
    /// elemental they extend.
    pub(super) fn class_name(self) -> &'static str {
        match self {
            Self::Skeleton => "VaultSkeleton",
            Self::Dm100 => "VaultDM100",
            Self::Shaman => "VaultShaman",
            Self::Dm200 => "VaultDM200",
            Self::Ghoul => "VaultGhoul",
            Self::Elemental => "VaultElemental",
            Self::FireElemental => "FireElemental",
            Self::FrostElemental => "FrostElemental",
            Self::ShockElemental => "ShockElemental",
            Self::Golem => "VaultGolem",
        }
    }

    fn ctor_rng(self) {
        match self {
            Self::Golem | Self::Dm200 => {
                let _ = Random::int_max(2);
            }
            Self::Shaman => {
                let _ = Random::int_max(5);
            }
            Self::FireElemental | Self::FrostElemental | Self::ShockElemental => {
                let _ = Random::normal_int_range(3, 5);
            }
            _ => {}
        }
    }

    pub(super) fn resolve(self) -> Self {
        let resolved = if self == Self::Elemental {
            let roll = Random::float();
            if roll < 0.4 {
                Self::FireElemental
            } else if roll < 0.8 {
                Self::FrostElemental
            } else {
                Self::ShockElemental
            }
        } else {
            self
        };
        resolved.ctor_rng();
        resolved
    }
}
