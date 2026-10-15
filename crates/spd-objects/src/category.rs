use serde::{Deserialize, Serialize};

/// Public item category. Serialized as its snake_case name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    Weapon,
    Armor,
    Missile,
    Wand,
    Ring,
    Artifact,
    Potion,
    Scroll,
    Stone,
    Seed,
    Food,
    Gold,
    Trinket,
    Other,
}

impl ItemCategory {
    pub const ALL: &'static [ItemCategory] = &[
        ItemCategory::Weapon,
        ItemCategory::Armor,
        ItemCategory::Missile,
        ItemCategory::Wand,
        ItemCategory::Ring,
        ItemCategory::Artifact,
        ItemCategory::Potion,
        ItemCategory::Scroll,
        ItemCategory::Stone,
        ItemCategory::Seed,
        ItemCategory::Food,
        ItemCategory::Gold,
        ItemCategory::Trinket,
        ItemCategory::Other,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            ItemCategory::Weapon => "weapon",
            ItemCategory::Armor => "armor",
            ItemCategory::Missile => "missile",
            ItemCategory::Wand => "wand",
            ItemCategory::Ring => "ring",
            ItemCategory::Artifact => "artifact",
            ItemCategory::Potion => "potion",
            ItemCategory::Scroll => "scroll",
            ItemCategory::Stone => "stone",
            ItemCategory::Seed => "seed",
            ItemCategory::Food => "food",
            ItemCategory::Gold => "gold",
            ItemCategory::Trinket => "trinket",
            ItemCategory::Other => "other",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|category| category.as_str() == value)
    }
}

impl std::fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
