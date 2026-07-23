//! Port of `com.shatteredpixel.shatteredpixeldungeon.items.Generator`.

mod categories;
mod state;

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "tests/rollover.rs"]
mod rollover_tests;

pub use categories::{FLOOR_SET_TIER_PROBS, MIS_TIERS, WEP_TIERS};
pub use state::{full_reset, GeneratorState};

/// Categories in `Generator.Category.values()` declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Trinket,
    Weapon,
    WepT1,
    WepT2,
    WepT3,
    WepT4,
    WepT5,
    Armor,
    Missile,
    MisT1,
    MisT2,
    MisT3,
    MisT4,
    MisT5,
    Wand,
    Ring,
    Artifact,
    Food,
    Potion,
    Seed,
    Scroll,
    Stone,
    Gold,
}

impl Category {
    pub const ALL: &'static [Category] = &[
        Category::Trinket,
        Category::Weapon,
        Category::WepT1,
        Category::WepT2,
        Category::WepT3,
        Category::WepT4,
        Category::WepT5,
        Category::Armor,
        Category::Missile,
        Category::MisT1,
        Category::MisT2,
        Category::MisT3,
        Category::MisT4,
        Category::MisT5,
        Category::Wand,
        Category::Ring,
        Category::Artifact,
        Category::Food,
        Category::Potion,
        Category::Seed,
        Category::Scroll,
        Category::Stone,
        Category::Gold,
    ];

    pub(crate) fn index(self) -> usize {
        Category::ALL
            .iter()
            .position(|&c| c == self)
            .expect("category in ALL")
    }
}
