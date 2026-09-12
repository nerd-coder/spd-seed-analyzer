//! `ExoticPotion.regToExo` / `ExoticScroll.regToExo` (pinned v4.0.0).

const REG_TO_EXO: &[(&str, &str)] = &[
    ("PotionOfStrength", "PotionOfMastery"),
    ("PotionOfHealing", "PotionOfShielding"),
    ("PotionOfMindVision", "PotionOfMagicalSight"),
    ("PotionOfFrost", "PotionOfSnapFreeze"),
    ("PotionOfLiquidFlame", "PotionOfDragonsBreath"),
    ("PotionOfToxicGas", "PotionOfCorrosiveGas"),
    ("PotionOfHaste", "PotionOfStamina"),
    ("PotionOfInvisibility", "PotionOfShroudingFog"),
    ("PotionOfLevitation", "PotionOfStormClouds"),
    ("PotionOfParalyticGas", "PotionOfEarthenArmor"),
    ("PotionOfPurity", "PotionOfCleansing"),
    ("PotionOfExperience", "PotionOfDivineInspiration"),
    ("ScrollOfUpgrade", "ScrollOfEnchantment"),
    ("ScrollOfIdentify", "ScrollOfDivination"),
    ("ScrollOfRemoveCurse", "ScrollOfAntiMagic"),
    ("ScrollOfMirrorImage", "ScrollOfPrismaticImage"),
    ("ScrollOfRecharging", "ScrollOfMysticalEnergy"),
    ("ScrollOfTeleportation", "ScrollOfPassage"),
    ("ScrollOfLullaby", "ScrollOfSirensSong"),
    ("ScrollOfMagicMapping", "ScrollOfForesight"),
    ("ScrollOfRage", "ScrollOfChallenge"),
    ("ScrollOfRetribution", "ScrollOfPsionicBlast"),
    ("ScrollOfTerror", "ScrollOfDread"),
    ("ScrollOfTransmutation", "ScrollOfMetamorphosis"),
];

pub(crate) fn regular_to_exotic(class_name: &str) -> Option<&'static str> {
    REG_TO_EXO
        .iter()
        .find(|(regular, _)| *regular == class_name)
        .map(|(_, exotic)| *exotic)
}

pub(crate) fn exotic_to_regular(class_name: &str) -> Option<&'static str> {
    REG_TO_EXO
        .iter()
        .find(|(_, exotic)| *exotic == class_name)
        .map(|(regular, _)| *regular)
}

/// Identity CrystalPath uses when de-exotifying for duplicate checks and rarity sort.
pub(crate) fn regular_consumable_class(class_name: &str) -> &str {
    exotic_to_regular(class_name).unwrap_or(class_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_and_exotic_maps_are_inverses() {
        for &(regular, exotic) in REG_TO_EXO {
            assert_eq!(regular_to_exotic(regular), Some(exotic));
            assert_eq!(exotic_to_regular(exotic), Some(regular));
            assert_eq!(regular_consumable_class(exotic), regular);
            assert_eq!(regular_consumable_class(regular), regular);
        }
        assert_eq!(regular_to_exotic("Gold"), None);
        assert_eq!(regular_consumable_class("Gold"), "Gold");
    }
}
