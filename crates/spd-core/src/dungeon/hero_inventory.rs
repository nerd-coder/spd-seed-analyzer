//! Hero inventory facts that affect deterministic level generation.

/// Which optional shop bag can hold an item in the hero's main backpack.
///
/// `ShopRoom.ChooseBag` only scans direct `backpack.items`, so items already
/// nested in a bag must not be represented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BagAffinity {
    VelvetPouch,
    ScrollHolder,
    PotionBandolier,
    MagicalHolster,
    None,
}

/// The inventory slice consulted while generating shop stock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeroInventory {
    pub main_backpack: Vec<BagAffinity>,
}

impl HeroInventory {
    /// Pinned Warrior initialization: Food, VelvetPouch, Waterskin, and
    /// ThrowingStone are direct backpack entries. Only the latter two score
    /// for still-available shop bags.
    pub fn fresh_warrior() -> Self {
        Self {
            main_backpack: vec![
                BagAffinity::None,
                BagAffinity::None,
                BagAffinity::PotionBandolier,
                BagAffinity::MagicalHolster,
            ],
        }
    }
}
