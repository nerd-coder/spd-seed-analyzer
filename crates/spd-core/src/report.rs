//! Serializable analysis report types.

mod compact;
mod quests;

use serde::{Deserialize, Serialize};

use crate::dungeon_seed::SeedError;
use crate::items::IdentityMaps;
use crate::trinkets::{
    ArtifactEvent, Challenge, ProfileError, TrinketEvent, TrinketSelectionReport,
};

pub(crate) use compact::is_baseline_highlight;
pub use quests::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedInfo {
    pub input: String,
    pub numeric: i64,
    /// Canonical `ABC-DEF-GHI` when representable.
    pub code: Option<String>,
    pub formatted: String,
    #[serde(default)]
    pub daily: bool,
}

/// Floor map for canvas rendering (SPD terrain IDs + tileset key).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FloorMap {
    pub width: u32,
    pub height: u32,
    /// Tileset key: `sewers` | `prison` | `caves` | `city` | `halls`
    pub tileset: String,
    /// Row-major SPD `Terrain` values
    pub tiles: Vec<u16>,
    /// SPD `DungeonTileSheet.setupVariance` values for deterministic tile alternates.
    pub tile_variance: Vec<u8>,
    /// Pinned `Level.cleanWalls()` visibility mask, row-major and parallel to `tiles`.
    #[serde(default)]
    pub discoverable: Vec<bool>,
    /// Exact cells known to the partial generator, including depth-one ambient mobs.
    #[serde(default)]
    pub markers: Vec<MapMarker>,
    /// Exact placed heap cells, types, and ordered item stacks.
    #[serde(default)]
    pub heaps: Vec<MapHeap>,
    /// Exact placed mob cells and pinned Java class names.
    #[serde(default)]
    pub mobs: Vec<MapMob>,
    /// Pinned Java `LevelTransition` facts, sorted by center cell then type.
    #[serde(default)]
    pub transitions: Vec<MapTransition>,
    /// Pinned Java trap facts, sorted by cell.
    #[serde(default)]
    pub traps: Vec<MapTrap>,
    /// Pinned Java plant facts, sorted by cell. Empty until a covered painter plants one.
    #[serde(default)]
    pub plants: Vec<MapPlant>,
    /// Active pinned Java blob concentrations, sorted by class then cell.
    #[serde(default)]
    pub blobs: Vec<MapBlob>,
    /// Pinned custom terrain overlays, ordered by their visual layer.
    #[serde(default)]
    pub custom_tiles: Vec<MapCustomTile>,
    /// Pinned custom wall overlays, ordered by their visual layer.
    #[serde(default)]
    pub custom_walls: Vec<MapCustomTile>,
    /// Internal provenance for cells whose main-loop loot facts depend on
    /// runtime-sensitive persistent Generator history. Never serialized.
    #[serde(skip)]
    #[doc(hidden)]
    pub runtime_sensitive_loot_cells: Vec<u32>,
    /// Stable heap positions whose concrete equipment contents/properties are
    /// player-state-sensitive. Never serialized.
    #[serde(skip)]
    #[doc(hidden)]
    pub constrained_equipment_cells: Vec<u32>,
}

/// A pinned Java `CustomTilemap` layer, including its resolved source tiles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapCustomTile {
    #[serde(rename = "class")]
    pub class_name: String,
    pub texture: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub static_data: Vec<i16>,
}

impl FloorMap {
    /// Terrain/layout projection. Entity placement is intentionally excluded.
    pub(crate) fn into_layout_only(mut self) -> Self {
        self.markers.clear();
        self.heaps.clear();
        self.mobs.clear();
        self.runtime_sensitive_loot_cells.clear();
        self.constrained_equipment_cells.clear();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapHeap {
    pub cell: u32,
    pub heap_type: String,
    pub items: Vec<MapHeapItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapHeapItem {
    #[serde(rename = "class")]
    pub class_name: String,
    pub quantity: i32,
    pub level: i32,
    pub cursed: bool,
    /// Internal generation provenance for parity checks; never serialized.
    #[doc(hidden)]
    #[serde(skip)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapMob {
    pub cell: u32,
    #[serde(rename = "class")]
    pub class_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapTransition {
    pub cell: u32,
    #[serde(rename = "type")]
    pub transition_type: String,
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub dest_depth: i32,
    pub dest_branch: i32,
    pub dest_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapTrap {
    pub cell: u32,
    #[serde(rename = "class")]
    pub class_name: String,
    pub visible: bool,
    pub active: bool,
    /// Pinned `Trap` sprite color index.
    #[serde(default)]
    pub color: u8,
    /// Pinned `Trap` sprite shape index.
    #[serde(default)]
    pub shape: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapPlant {
    pub cell: u32,
    #[serde(rename = "class")]
    pub class_name: String,
    /// Pinned `Plant.image` sprite index.
    #[serde(default)]
    pub image: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapBlob {
    #[serde(rename = "class")]
    pub class_name: String,
    pub volume: u32,
    pub always_visible: bool,
    pub cells: Vec<MapBlobCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapBlobCell {
    pub cell: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapMarker {
    /// Row-major index into `FloorMap.tiles`.
    pub cell: u32,
    pub kind: MapMarkerKind,
    pub label: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MapMarkerKind {
    Item,
    Mob,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FloorReport {
    pub depth: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeling: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    /// Room types selected by `initRooms` (order after shuffle).
    #[serde(default)]
    pub rooms: Vec<String>,
    /// Room types that can occur in the modeled player/trinket profiles.
    ///
    /// `rooms` remains the no-extra-player-state baseline for compatibility.
    /// This field is the machine-readable projection when a profile can alter
    /// the layout. Each entry's conditions are alternatives (logical OR).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub possible_rooms: Vec<PossibleRoom>,
    /// Seed-determined non-loot features that are guaranteed to exist on this floor.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guaranteed_appearances: Vec<GuaranteedAppearance>,
    /// Exact non-positional entity summary for the floor's initial generated state.
    /// Runtime summons and respawns are deliberately excluded.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_encounters: Vec<InitialEncounter>,
    /// Logical item spawns and their exact, constrained, or fresh-baseline variants.
    /// Only exact variants are finder evidence.
    pub items: Vec<ItemGroup>,
    pub quests: Vec<QuestReport>,
    /// Present when geometry build succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<FloorMap>,
    /// Layout produced by the analyzer's baseline continuation after an
    /// unresolved player/meta-state branch. This is never an exact public
    /// prediction and must be presented with its assumption warning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_map: Option<FloorMap>,
}

impl FloorReport {
    pub fn item_variants(&self) -> impl Iterator<Item = &ItemEntry> {
        self.items.iter().flat_map(|group| &group.variants)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InitialEncounter {
    #[serde(rename = "class")]
    pub class_name: String,
    pub name: String,
    pub quantity: u32,
    /// Base rewards associated with defeating this generated entity. Empty means
    /// the pinned class has no seed-analysis combat reward.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub combat_rewards: Vec<CombatReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatReward {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    pub category: String,
    pub prediction: CombatRewardPrediction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chance: Option<RewardChance>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CombatRewardPrediction {
    /// A fixed class reward on an ordinary eligible defeat.
    Guaranteed,
    /// The occurrence and/or identity is rolled on the gameplay RNG stream.
    RuntimeChance,
    /// The concrete carried reward was generated with the floor.
    GeneratedWithFloor,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RewardChance {
    pub numerator: u32,
    pub denominator: u32,
}

/// A room type and count observed in one or more modeled generation profiles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PossibleRoom {
    #[serde(rename = "class")]
    pub class_name: String,
    /// Number of rooms of this type for the associated profile condition.
    pub quantity: u32,
    /// Alternative player-state profiles under which this exact count occurs.
    /// An empty list means the modeled dependency axes do not change it.
    #[serde(
        default,
        skip_serializing_if = "spawn_conditions_empty_after_normalization",
        serialize_with = "serialize_spawn_conditions"
    )]
    pub spawn_conditions: Vec<ItemSpawnCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuaranteedAppearance {
    pub name: String,
    pub kind: GuaranteedAppearanceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GuaranteedAppearanceKind {
    AlchemyPot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemEntry {
    pub name: String,
    /// Number of identical items represented by this entry.
    #[serde(default = "default_item_quantity")]
    pub quantity: i32,
    /// Java simple class name (e.g. `Sword`, `PotionOfHealing`) for icons/lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// Ordered seed-determined identities when run history can shift a deck index.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidate_classes: Vec<String>,
    pub category: String,
    /// Equipment tier when it is stable even though the concrete class is not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
    /// Inclusive equipment-tier bounds when the tier is constrained but not exact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_range: Option<NumericRange>,
    /// Current SPD item upgrade level (`0` is unupgraded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// Inclusive upgrade-level bounds when the final level is constrained but not exact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_range: Option<NumericRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursed: Option<bool>,
    /// Seed-determined enchantment or glyph, whether unconditional or conditional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enchantment: Option<ItemEnchantment>,
    pub prediction: ItemPredictionKind,
    /// Alternative player-state profiles under which this item may spawn.
    /// Any clause is sufficient; an empty list means the modeled dependency
    /// axes do not change this item.
    #[serde(
        default,
        skip_serializing_if = "spawn_conditions_empty_after_normalization",
        serialize_with = "serialize_spawn_conditions"
    )]
    pub spawn_conditions: Vec<ItemSpawnCondition>,
    /// Gate-only conditions that affect this item's presence or retained effect.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ItemCondition>,
    /// Legacy internal diagnostics retained for parity tests; never serialized.
    #[serde(skip)]
    pub notes: Vec<String>,
    /// Internal provenance copied to `ItemGroup::source` in floor reports.
    #[serde(skip)]
    pub source: Option<String>,
}

/// One logical item spawn. Multiple variants are alternative projections of
/// that spawn, rather than additional items.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ItemGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub variants: Vec<ItemEntry>,
}

impl ItemGroup {
    pub fn single(item: ItemEntry) -> Self {
        Self {
            source: item.source.clone(),
            variants: vec![item],
        }
    }
}

impl From<ItemEntry> for ItemGroup {
    fn from(item: ItemEntry) -> Self {
        Self::single(item)
    }
}

impl std::ops::Deref for ItemGroup {
    type Target = ItemEntry;

    fn deref(&self) -> &Self::Target {
        self.variants
            .first()
            .expect("public item groups always contain a variant")
    }
}

impl<'de> Deserialize<'de> for ItemGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireGroup {
            source: Option<String>,
            variants: Vec<ItemEntry>,
        }

        let mut wire = WireGroup::deserialize(deserializer)?;
        for variant in &mut wire.variants {
            variant.source = wire.source.clone();
        }
        Ok(Self {
            source: wire.source,
            variants: wire.variants,
        })
    }
}

pub(crate) fn group_item_entries(entries: Vec<ItemEntry>) -> Vec<ItemGroup> {
    let mut groups: Vec<ItemGroup> = Vec::with_capacity(entries.len());
    for entry in entries {
        let paired_room_projection = matches!(
            entry.source.as_deref(),
            Some(
                "SacrificeRoom"
                    | "CryptRoom"
                    | "StatueRoom"
                    | "PoolRoom:equipment"
                    | "SuspiciousChestRoom:gold"
                    | "SuspiciousChestRoom:mimic_reward"
                    | "CrystalChoiceRoom:hidden_reward"
                    | "SecretHoneypotRoom:bomb"
                    | "GrassyGraveRoom:prize"
            )
        );
        let paired_prediction = matches!(
            entry.prediction,
            ItemPredictionKind::Constrained | ItemPredictionKind::Baseline
        );
        let matching = (paired_room_projection && paired_prediction).then(|| {
            groups.iter_mut().find(|group| {
                group.source == entry.source
                    && group.variants.len() == 1
                    && (group.variants[0].category == entry.category
                        || matches!(
                            entry.source.as_deref(),
                            Some(
                                "PoolRoom:equipment"
                                    | "SuspiciousChestRoom:mimic_reward"
                                    | "CrystalChoiceRoom:hidden_reward"
                                    | "GrassyGraveRoom:prize"
                            )
                        ))
                    && matches!(
                        (group.variants[0].prediction, entry.prediction),
                        (
                            ItemPredictionKind::Constrained,
                            ItemPredictionKind::Baseline
                        ) | (
                            ItemPredictionKind::Baseline,
                            ItemPredictionKind::Constrained
                        )
                    )
            })
        });
        if let Some(Some(group)) = matching {
            group.variants.push(entry);
            group
                .variants
                .sort_by_key(|variant| variant.prediction == ItemPredictionKind::Baseline);
        } else {
            groups.push(ItemGroup::single(entry));
        }
    }
    groups
}

const fn default_item_quantity() -> i32 {
    1
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumericRange {
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemPredictionKind {
    /// Concrete identity and properties are safe to use for display and search.
    Exact,
    /// Concrete result from the fresh, no-history replay. It is useful for
    /// planning but player-controlled prior generation can change it.
    Baseline,
    /// Only the explicitly populated constraints are seed-only guarantees.
    Constrained,
}

/// One conjunction in an item's possible-spawn condition. Multiple clauses on
/// an item are alternatives (logical OR).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemSpawnCondition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<ItemDependencyCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemEnchantment {
    #[serde(rename = "type")]
    pub enchantment_type: String,
    pub conditions: Vec<ItemCondition>,
}

/// Typed gate conditions for item presence or retained effects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemCondition {
    Challenge {
        challenge: Challenge,
        enabled: bool,
    },
    Trinket {
        events: Vec<TrinketEvent>,
    },
    Artifact {
        events: Vec<ArtifactEvent>,
    },
    Quest {
        quest_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        depth: Option<u32>,
    },
    Choice {
        group_id: String,
        option_count: u32,
        selected_count: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        favor_requirement: Option<u32>,
    },
    Inventory {
        requirement_id: String,
    },
    Runtime {
        state_id: String,
    },
}

/// A condition axis that is allowed to affect public item presence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemDependencyCondition {
    Challenge {
        challenge: Challenge,
        enabled: bool,
    },
    /// An empty event list means no generation-affecting held trinket.
    Trinket {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        events: Vec<TrinketEvent>,
    },
    /// An empty event list means no external artifact history.
    Artifact {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        events: Vec<ArtifactEvent>,
    },
}

fn spawn_conditions_empty_after_normalization(conditions: &[ItemSpawnCondition]) -> bool {
    conditions.iter().all(|clause| {
        clause.all_of.iter().all(|dependency| {
            matches!(dependency, ItemDependencyCondition::Trinket { events } if events.is_empty())
        })
    })
}

fn serialize_spawn_conditions<S>(
    conditions: &[ItemSpawnCondition],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let normalized: Vec<ItemSpawnCondition> = conditions
        .iter()
        .filter_map(|clause| {
            let all_of = clause
                .all_of
                .iter()
                .filter(|dependency| {
                    !matches!(dependency, ItemDependencyCondition::Trinket { events } if events.is_empty())
                })
                .cloned()
                .collect::<Vec<_>>();
            (!all_of.is_empty()).then_some(ItemSpawnCondition { all_of })
        })
        .collect();
    normalized.serialize(serializer)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedReport {
    pub seed: SeedInfo,
    pub spd_version: String,
    pub spd_commit: String,
    pub floors_requested: u32,
    pub identities: IdentityMaps,
    pub trinket_selection: TrinketSelectionReport,
    pub floors: Vec<FloorReport>,
    /// `"partial"` while only forced drops exist; `"ok"` when full levelgen lands.
    pub status: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzeError {
    #[error(transparent)]
    Seed(#[from] SeedError),
    #[error(transparent)]
    Profile(#[from] ProfileError),
}
