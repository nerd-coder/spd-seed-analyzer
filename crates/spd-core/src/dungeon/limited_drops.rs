//! `Dungeon.LimitedDrops` counters used by levelgen.

#[derive(Debug, Clone, Default)]
pub struct LimitedDrops {
    pub strength_potions: i32,
    pub upgrade_scrolls: i32,
    pub arcane_styli: i32,
    pub ench_stone: bool,
    pub int_stone: bool,
    pub trinket_cata: bool,
    pub lab_room: i32,
    /// Hero always starts with a velvet pouch → treated as already dropped.
    pub velvet_pouch: bool,
    pub scroll_holder: bool,
    pub potion_bandolier: bool,
    pub magical_holster: bool,
}

impl LimitedDrops {
    pub fn reset() -> Self {
        Self {
            // All heroes start with VelvetPouch (SPD HeroClass).
            velvet_pouch: true,
            ..Self::default()
        }
    }
}
