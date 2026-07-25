//! Public projection helpers for item facts.

use crate::items::model::{GeneratedItem, ShopStockRole};

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
