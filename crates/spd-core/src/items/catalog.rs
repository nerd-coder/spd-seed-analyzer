//! Ordered item class names and appearance labels from SPD static tables.
//!
//! Order must match Java `Generator.Category` and `LinkedHashMap` insertion order.

/// `Generator.Category.POTION.classes` simple names (analysis keys).
pub const POTION_ITEMS: &[&str] = &[
    "PotionOfStrength",
    "PotionOfHealing",
    "PotionOfMindVision",
    "PotionOfFrost",
    "PotionOfLiquidFlame",
    "PotionOfToxicGas",
    "PotionOfHaste",
    "PotionOfInvisibility",
    "PotionOfLevitation",
    "PotionOfParalyticGas",
    "PotionOfPurity",
    "PotionOfExperience",
];

/// `Potion.colors` LinkedHashMap keys.
pub const POTION_COLORS: &[&str] = &[
    "crimson",
    "amber",
    "golden",
    "jade",
    "turquoise",
    "azure",
    "indigo",
    "magenta",
    "bistre",
    "charcoal",
    "silver",
    "ivory",
];

/// `Generator.Category.SCROLL.classes`.
pub const SCROLL_ITEMS: &[&str] = &[
    "ScrollOfUpgrade",
    "ScrollOfIdentify",
    "ScrollOfRemoveCurse",
    "ScrollOfMirrorImage",
    "ScrollOfRecharging",
    "ScrollOfTeleportation",
    "ScrollOfLullaby",
    "ScrollOfMagicMapping",
    "ScrollOfRage",
    "ScrollOfRetribution",
    "ScrollOfTerror",
    "ScrollOfTransmutation",
];

/// `Scroll.runes` LinkedHashMap keys.
pub const SCROLL_RUNES: &[&str] = &[
    "KAUNAN", "SOWILO", "LAGUZ", "YNGVI", "GYFU", "RAIDO", "ISAZ", "MANNAZ", "NAUDIZ", "BERKANAN",
    "ODAL", "TIWAZ",
];

/// `Generator.Category.RING.classes`.
pub const RING_ITEMS: &[&str] = &[
    "RingOfAccuracy",
    "RingOfArcana",
    "RingOfElements",
    "RingOfEnergy",
    "RingOfEvasion",
    "RingOfForce",
    "RingOfFuror",
    "RingOfHaste",
    "RingOfMight",
    "RingOfSharpshooting",
    "RingOfTenacity",
    "RingOfWealth",
];

/// `Ring.gems` LinkedHashMap keys.
pub const RING_GEMS: &[&str] = &[
    "garnet",
    "ruby",
    "topaz",
    "emerald",
    "onyx",
    "opal",
    "tourmaline",
    "sapphire",
    "amethyst",
    "quartz",
    "agate",
    "diamond",
];

/// Display name for UI (English, approximate game titles).
pub fn display_name(class_name: &str) -> String {
    // Strip common prefixes and insert spaces before capitals.
    let stripped = class_name
        .strip_prefix("PotionOf")
        .or_else(|| class_name.strip_prefix("ScrollOf"))
        .or_else(|| class_name.strip_prefix("RingOf"))
        .unwrap_or(class_name);

    let mut out = String::new();
    for (i, ch) in stripped.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }

    if class_name.starts_with("PotionOf") {
        format!("Potion of {out}")
    } else if class_name.starts_with("ScrollOf") {
        format!("Scroll of {out}")
    } else if class_name.starts_with("RingOf") {
        format!("Ring of {out}")
    } else {
        out
    }
}
