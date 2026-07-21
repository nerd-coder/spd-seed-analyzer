//! Generated item representation for analysis reports.

use serde::{Deserialize, Serialize};

use super::catalog::display_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratedItem {
    /// Java simple class name (e.g. `Sword`, `PotionOfHealing`).
    pub class_name: String,
    pub category: ItemCategory,
    pub level: i32,
    pub quantity: i32,
    pub cursed: bool,
    /// Enchantment or glyph simple name, if any.
    pub enchantment: Option<String>,
    pub source: Option<String>,
}

impl GeneratedItem {
    pub fn new(class_name: impl Into<String>, category: ItemCategory) -> Self {
        Self {
            class_name: class_name.into(),
            category,
            level: 0,
            quantity: 1,
            cursed: false,
            enchantment: None,
            source: None,
        }
    }

    /// English-ish title similar to seed-finder output.
    pub fn title(&self) -> String {
        let base = match self.category {
            ItemCategory::Potion | ItemCategory::Scroll | ItemCategory::Ring => {
                display_name(&self.class_name)
            }
            ItemCategory::Gold => format!("{} gold", self.quantity),
            _ => humanize_class(&self.class_name),
        };

        let mut out = String::new();
        if self.cursed {
            out.push_str("cursed ");
        }
        if let Some(ref ench) = self.enchantment {
            out.push_str(&humanize_class(ench));
            out.push(' ');
        }
        out.push_str(&base);
        if self.level > 0
            && !matches!(
                self.category,
                ItemCategory::Potion
                    | ItemCategory::Scroll
                    | ItemCategory::Stone
                    | ItemCategory::Seed
                    | ItemCategory::Food
                    | ItemCategory::Gold
            )
        {
            out.push_str(&format!(" +{}", self.level));
        }
        out
    }
}

fn humanize_class(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(if i == 0 { ch.to_ascii_lowercase() } else { ch });
    }
    // First letter lower for seed-finder style ("projecting crossbow")
    if let Some(first) = out.chars().next() {
        if first.is_uppercase() {
            let rest: String = out.chars().skip(1).collect();
            return format!("{}{}", first.to_ascii_lowercase(), rest);
        }
    }
    out
}
