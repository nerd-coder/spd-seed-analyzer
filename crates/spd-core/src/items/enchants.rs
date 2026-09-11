//! Weapon enchantments and armor glyphs (random selection tables).
//!
//! Weapon arrays match `Weapon.Enchantment` in `Weapon.java:518-540` (v4.0.0).
//! Names only — `proc` and Unstable's on-hit pool are combat (07).

use crate::random::Random;

const WEAPON_COMMON: &[&str] = &["Blazing", "Chilling", "Kinetic", "Shocking", "Venomous"];
const WEAPON_UNCOMMON: &[&str] = &[
    "Blocking",
    "Blooming",
    "Eldritch",
    "Elastic",
    "Lucky",
    "Projecting",
    "Unstable",
    "Vorpal",
];
const WEAPON_RARE: &[&str] = &["Corrupting", "Crystal", "Grim", "Vampiric"];
const WEAPON_CURSES: &[&str] = &[
    "Annoying",
    "Displacing",
    "Dazzling",
    "Explosive",
    "Friendly",
    "Polarized",
    "Pressurized",
    "Sacrificial",
    "Wayward",
    "Wondrous",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_enchant_tables_match_java_v4() {
        assert_eq!(
            WEAPON_COMMON,
            &["Blazing", "Chilling", "Kinetic", "Shocking", "Venomous"]
        );
        assert_eq!(
            WEAPON_UNCOMMON,
            &[
                "Blocking",
                "Blooming",
                "Eldritch",
                "Elastic",
                "Lucky",
                "Projecting",
                "Unstable",
                "Vorpal"
            ]
        );
        assert_eq!(WEAPON_RARE, &["Corrupting", "Crystal", "Grim", "Vampiric"]);
        assert_eq!(
            WEAPON_CURSES,
            &[
                "Annoying",
                "Displacing",
                "Dazzling",
                "Explosive",
                "Friendly",
                "Polarized",
                "Pressurized",
                "Sacrificial",
                "Wayward",
                "Wondrous"
            ]
        );
        assert_eq!(WEAPON_TYPE_CHANCES, &[50.0, 40.0, 10.0]);
        for name in [
            "Venomous",
            "Eldritch",
            "Vorpal",
            "Crystal",
            "Pressurized",
            "Wondrous",
        ] {
            assert!(
                WEAPON_COMMON.contains(&name)
                    || WEAPON_UNCOMMON.contains(&name)
                    || WEAPON_RARE.contains(&name)
                    || WEAPON_CURSES.contains(&name),
                "{name} missing from weapon enchant/curse tables"
            );
        }
    }

    #[test]
    fn armor_glyph_tables_unchanged() {
        assert_eq!(
            ARMOR_COMMON,
            &["Obfuscation", "Swiftness", "Viscosity", "Potential"]
        );
        assert_eq!(
            ARMOR_UNCOMMON,
            &[
                "Brimstone",
                "Stone",
                "Entanglement",
                "Repulsion",
                "Camouflage",
                "Flow"
            ]
        );
        assert_eq!(ARMOR_RARE, &["Affection", "AntiMagic", "Thorns"]);
        assert_eq!(
            ARMOR_CURSES,
            &[
                "AntiEntropy",
                "Corrosion",
                "Displacement",
                "Metabolism",
                "Multiplicity",
                "Stench",
                "Overgrowth",
                "Bulk"
            ]
        );
        assert_eq!(ARMOR_TYPE_CHANCES, &[50.0, 40.0, 10.0]);
    }
}
