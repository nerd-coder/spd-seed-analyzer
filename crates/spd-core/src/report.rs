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
    /// Current SPD item upgrade level (`0` is unupgraded).
    #[serde(default)]
    pub level: i32,
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
