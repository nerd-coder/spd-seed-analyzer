//! Serializable analysis report types.

use serde::{Deserialize, Serialize};

use crate::dungeon_seed::SeedError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedInfo {
    pub input: String,
    pub numeric: i64,
    /// Canonical `ABC-DEF-GHI` when representable.
    pub code: Option<String>,
    pub formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorReport {
    pub depth: u32,
    pub items: Vec<ItemEntry>,
    pub quests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEntry {
    pub name: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedReport {
    pub seed: SeedInfo,
    pub spd_version: String,
    pub spd_commit: String,
    pub floors_requested: u32,
    pub floors: Vec<FloorReport>,
    /// `"scaffold"` until full analysis lands; then `"ok"`.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalyzeError {
    #[error(transparent)]
    Seed(#[from] SeedError),
}
