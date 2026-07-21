//! Serializable analysis report types.

use serde::{Deserialize, Serialize};

use crate::dungeon_seed::SeedError;
use crate::items::IdentityMaps;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedInfo {
    pub input: String,
    pub numeric: i64,
    /// Canonical `ABC-DEF-GHI` when representable.
    pub code: Option<String>,
    pub formatted: String,
}

/// Floor map for canvas rendering (SPD terrain IDs + tileset key).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorMap {
    pub width: u32,
    pub height: u32,
    /// Tileset key: `sewers` | `prison` | `caves` | `city` | `halls`
    pub tileset: String,
    /// Row-major SPD `Terrain` values
    pub tiles: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorReport {
    pub depth: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeling: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    /// Room types selected by `initRooms` (order after shuffle).
    #[serde(default)]
    pub rooms: Vec<String>,
    pub items: Vec<ItemEntry>,
    pub quests: Vec<String>,
    /// Present when geometry build succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<FloorMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEntry {
    pub name: String,
    /// Java simple class name (e.g. `Sword`, `PotionOfHealing`) for icons/lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    pub category: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub cursed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedReport {
    pub seed: SeedInfo,
    pub spd_version: String,
    pub spd_commit: String,
    pub floors_requested: u32,
    pub identities: IdentityMaps,
    pub floors: Vec<FloorReport>,
    /// `"partial"` while only forced drops exist; `"ok"` when full levelgen lands.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzeError {
    #[error(transparent)]
    Seed(#[from] SeedError),
}
