//! Serializable player-facing profile data.

use serde::{Deserialize, Serialize};

/// Player-facing challenges, mapped to SPD's nine-bit challenge mask.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Challenge {
    ChampionEnemies,
    BadderBosses,
    OnDiet,
    FaithIsMyArmor,
    Pharmacophobia,
    BarrenLand,
    SwarmIntelligence,
    IntoDarkness,
    ForbiddenRunes,
}

impl Challenge {
    pub const ALL: [Self; 9] = [
        Self::ChampionEnemies,
        Self::BadderBosses,
        Self::OnDiet,
        Self::FaithIsMyArmor,
        Self::Pharmacophobia,
        Self::BarrenLand,
        Self::SwarmIntelligence,
        Self::IntoDarkness,
        Self::ForbiddenRunes,
    ];

    pub const fn mask(self) -> i32 {
        match self {
            Self::OnDiet => 1,
            Self::FaithIsMyArmor => 2,
            Self::Pharmacophobia => 4,
            Self::BarrenLand => 8,
            Self::SwarmIntelligence => 16,
            Self::IntoDarkness => 32,
            Self::ForbiddenRunes => 64,
            Self::ChampionEnemies => 128,
            Self::BadderBosses => 256,
        }
    }
}

/// All trinkets in the pinned game's first trinket deck.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TrinketKind {
    RatSkull,
    ParchmentScrap,
    PetrifiedSeed,
    ExoticCrystals,
    MossyClump,
    DimensionalSundial,
    ThirteenLeafClover,
    TrapMechanism,
    MimicTooth,
    WondrousResin,
    EyeOfNewt,
    SaltCube,
    VialOfBlood,
    ShardOfOblivion,
    ChaoticCenser,
    FerretTuft,
    CrackedSpyglass,
}

impl TrinketKind {
    pub const ALL: [Self; 17] = [
        Self::RatSkull,
        Self::ParchmentScrap,
        Self::PetrifiedSeed,
        Self::ExoticCrystals,
        Self::MossyClump,
        Self::DimensionalSundial,
        Self::ThirteenLeafClover,
        Self::TrapMechanism,
        Self::MimicTooth,
        Self::WondrousResin,
        Self::EyeOfNewt,
        Self::SaltCube,
        Self::VialOfBlood,
        Self::ShardOfOblivion,
        Self::ChaoticCenser,
        Self::FerretTuft,
        Self::CrackedSpyglass,
    ];

    pub const fn class_name(self) -> &'static str {
        match self {
            Self::RatSkull => "RatSkull",
            Self::ParchmentScrap => "ParchmentScrap",
            Self::PetrifiedSeed => "PetrifiedSeed",
            Self::ExoticCrystals => "ExoticCrystals",
            Self::MossyClump => "MossyClump",
            Self::DimensionalSundial => "DimensionalSundial",
            Self::ThirteenLeafClover => "ThirteenLeafClover",
            Self::TrapMechanism => "TrapMechanism",
            Self::MimicTooth => "MimicTooth",
            Self::WondrousResin => "WondrousResin",
            Self::EyeOfNewt => "EyeOfNewt",
            Self::SaltCube => "SaltCube",
            Self::VialOfBlood => "VialOfBlood",
            Self::ShardOfOblivion => "ShardOfOblivion",
            Self::ChaoticCenser => "ChaoticCenser",
            Self::FerretTuft => "FerretTuft",
            Self::CrackedSpyglass => "CrackedSpyglass",
        }
    }

    pub(crate) fn from_class_name(class_name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|trinket| trinket.class_name() == class_name)
    }
}

/// Artifacts that can be present in the pinned generator's unique artifact deck.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    AlchemistsToolkit,
    ChaliceOfBlood,
    CloakOfShadows,
    DriedRose,
    EtherealChains,
    HolyTome,
    HornOfPlenty,
    MasterThievesArmband,
    SandalsOfNature,
    SkeletonKey,
    TalismanOfForesight,
    TimekeepersHourglass,
    UnstableSpellbook,
}

/// A chronological change to the one held trinket.
///
/// `before_depth` is the first main-path floor generated after the action.
/// Acquiring or transmuting constructs a new trinket instance; upgrading
/// preserves the active instance and its instance-local state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrinketEvent {
    pub before_depth: u32,
    #[serde(flatten)]
    pub action: TrinketEventAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TrinketEventAction {
    Acquired {
        trinket: TrinketKind,
        /// Minimum upgrades required when this event describes a condition.
        /// Omitted for an ordinary acquisition at +0.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_upgrades: Option<u8>,
    },
    Upgraded,
    Transmuted {
        trinket: TrinketKind,
    },
}

/// An externally observed artifact event which occurs before a floor is first
/// generated. The profile records what the player did or found, never a deck
/// counter; the lifecycle replay will derive deck mutations from these events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactEvent {
    pub before_depth: u32,
    #[serde(flatten)]
    pub action: ArtifactEventAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ArtifactEventAction {
    Obtained { artifact: ArtifactKind },
    Transmuted { artifact: ArtifactKind },
}

/// Player state consulted only when a generated quest reward is claimed.
/// It does not advance level-generation RNG.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parchment_scrap_level: Option<u8>,
}

/// Player-meaningful inputs for a first generation of the main-path floors.
///
/// This type intentionally contains chronological gameplay events rather than
/// `Generator` deck counters. The replay derives the latter from the seed.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapProfile {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub challenges: Vec<Challenge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trinket_events: Vec<TrinketEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_events: Vec<ArtifactEvent>,
    #[serde(default)]
    pub claim_state: ClaimState,
}

/// Resolved held state for a particular generated floor. Its instance is
/// derived from the chronological profile and remains internal to the replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActiveTrinket {
    pub trinket: TrinketKind,
    pub level: u8,
    pub instance: u32,
}
