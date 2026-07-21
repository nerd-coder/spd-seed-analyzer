//! Standard special-room prize painters (Crypt, Armory, Library, …).

mod consumable;
mod equip;

pub(super) use consumable::{
    laboratory_prizes, library_prizes, runestone_prizes, storage_prize, storage_prizes,
    treasury_prizes,
};
pub(super) use equip::{armory_prizes, bomb_random, crypt_prize, pool_prize, statue_weapon};

use crate::items::model::GeneratedItem;

pub(super) fn is_curse_enchant(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => matches!(
            e,
            "Annoying"
                | "Displacing"
                | "Dazzling"
                | "Explosive"
                | "Sacrificial"
                | "Wayward"
                | "Polarized"
                | "Friendly"
                | "AntiEntropy"
                | "Corrosion"
                | "Displacement"
                | "Metabolism"
                | "Multiplicity"
                | "Stench"
                | "Overgrowth"
                | "Bulk"
        ),
        None => false,
    }
}

pub(super) fn is_good_glyph(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => !matches!(
            e,
            "AntiEntropy"
                | "Corrosion"
                | "Displacement"
                | "Metabolism"
                | "Multiplicity"
                | "Stench"
                | "Overgrowth"
                | "Bulk"
        ),
        None => false,
    }
}
