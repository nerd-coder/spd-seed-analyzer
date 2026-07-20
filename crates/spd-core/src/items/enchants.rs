//! Weapon enchantments and armor glyphs (random selection tables).

use crate::random::Random;

const WEAPON_COMMON: &[&str] = &["Blazing", "Chilling", "Kinetic", "Shocking"];
const WEAPON_UNCOMMON: &[&str] = &[
    "Blocking",
    "Blooming",
    "Elastic",
    "Lucky",
    "Projecting",
    "Unstable",
];
const WEAPON_RARE: &[&str] = &["Corrupting", "Grim", "Vampiric"];
const WEAPON_CURSES: &[&str] = &[
    "Annoying",
    "Displacing",
    "Dazzling",
    "Explosive",
    "Sacrificial",
    "Wayward",
    "Polarized",
    "Friendly",
];
const WEAPON_TYPE_CHANCES: &[f32] = &[50.0, 40.0, 10.0];

const ARMOR_COMMON: &[&str] = &["Obfuscation", "Swiftness", "Viscosity", "Potential"];
const ARMOR_UNCOMMON: &[&str] = &[
    "Brimstone",
    "Stone",
    "Entanglement",
    "Repulsion",
    "Camouflage",
    "Flow",
];
const ARMOR_RARE: &[&str] = &["Affection", "AntiMagic", "Thorns"];
const ARMOR_CURSES: &[&str] = &[
    "AntiEntropy",
    "Corrosion",
    "Displacement",
    "Metabolism",
    "Multiplicity",
    "Stench",
    "Overgrowth",
    "Bulk",
];
const ARMOR_TYPE_CHANCES: &[f32] = &[50.0, 40.0, 10.0];

fn pick_from(list: &[&'static str], ignore: Option<&str>) -> &'static str {
    let filtered: Vec<&str> = list
        .iter()
        .copied()
        .filter(|n| ignore != Some(*n))
        .collect();
    if filtered.is_empty() {
        // Java falls back to random() without ignore — for curses this is rare
        return list[Random::int_max(list.len() as i32) as usize];
    }
    filtered[Random::int_max(filtered.len() as i32) as usize]
}

pub fn random_weapon_enchant(ignore: Option<&str>) -> &'static str {
    match Random::chances(WEAPON_TYPE_CHANCES) {
        0 => pick_from(WEAPON_COMMON, ignore),
        1 => pick_from(WEAPON_UNCOMMON, ignore),
        _ => pick_from(WEAPON_RARE, ignore),
    }
}

pub fn random_weapon_curse(ignore: Option<&str>) -> &'static str {
    pick_from(WEAPON_CURSES, ignore)
}

pub fn random_armor_glyph(ignore: Option<&str>) -> &'static str {
    match Random::chances(ARMOR_TYPE_CHANCES) {
        0 => pick_from(ARMOR_COMMON, ignore),
        1 => pick_from(ARMOR_UNCOMMON, ignore),
        _ => pick_from(ARMOR_RARE, ignore),
    }
}

pub fn random_armor_curse(ignore: Option<&str>) -> &'static str {
    pick_from(ARMOR_CURSES, ignore)
}
