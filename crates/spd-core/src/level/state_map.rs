//! Public projection helpers for floor and item facts.

use crate::items::model::{GeneratedItem, ShopStockRole};
use crate::report::{GuaranteedAppearance, GuaranteedAppearanceKind};

pub(super) fn guaranteed_appearances(
    rooms: &[String],
    exact_layout: bool,
) -> Vec<GuaranteedAppearance> {
    if !exact_layout {
        return Vec::new();
    }

    rooms
        .iter()
        .filter(|room| matches!(room.as_str(), "LaboratoryRoom" | "SecretLaboratoryRoom"))
        .map(|room| GuaranteedAppearance {
            name: "Alchemy pot".into(),
            kind: GuaranteedAppearanceKind::AlchemyPot,
            source: Some(room.clone()),
        })
        .collect()
}

pub(super) fn reported_level(
    item: &GeneratedItem,
    constrained: bool,
    shop_role: Option<ShopStockRole>,
) -> Option<i32> {
    if shop_role == Some(ShopStockRole::DeckRareArtifactOrRing) {
        // Pinned case 2 does not call level(0). If the artifact deck is
        // exhausted, Generator falls back to a Ring with its randomized level.
        None
    } else if constrained && shop_role.is_some() {
        Some(0)
    } else {
        (!constrained).then_some(item.level)
    }
}
