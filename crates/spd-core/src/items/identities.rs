//! Potion colors, scroll runes, and ring gems for a run.

use serde::{Deserialize, Serialize};

use super::catalog::{
    POTION_COLORS, POTION_ITEMS, RING_GEMS, RING_ITEMS, SCROLL_ITEMS, SCROLL_RUNES, display_name,
};
use super::status_handler::assign_labels;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityEntry {
    /// Java simple class name.
    pub item: String,
    /// Human-readable English title.
    pub name: String,
    /// Appearance label (color / rune / gem).
    pub appearance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityMaps {
    pub potions: Vec<IdentityEntry>,
    pub scrolls: Vec<IdentityEntry>,
    pub rings: Vec<IdentityEntry>,
}

fn to_entries(pairs: Vec<(String, String)>) -> Vec<IdentityEntry> {
    pairs
        .into_iter()
        .map(|(item, appearance)| IdentityEntry {
            name: display_name(&item),
            item,
            appearance,
        })
        .collect()
}

/// Consumes RNG like Scroll.initLabels / Potion.initColors / Ring.initGems.
pub fn init_identities() -> IdentityMaps {
    // Order in Dungeon.init: Scroll, Potion, Ring
    let scrolls = to_entries(assign_labels(SCROLL_ITEMS, SCROLL_RUNES));
    let potions = to_entries(assign_labels(POTION_ITEMS, POTION_COLORS));
    let rings = to_entries(assign_labels(RING_ITEMS, RING_GEMS));
    IdentityMaps {
        potions,
        scrolls,
        rings,
    }
}
