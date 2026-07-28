//! Challenge-sensitive guaranteed floor spawns from `Level.create`.

use crate::dungeon::DungeonState;
use crate::items::model::{ForcedDropRole, GeneratedItem, ItemCategory, ItemProvenance};

/// Applies the pinned Forbidden Runes rule to a scheduled Upgrade Scroll.
///
/// Returns whether the queue is challenge-sensitive because the caller did
/// not provide a profile. SPD increments the limited-drop counter even when
/// the challenge suppresses the item.
pub(super) fn queue_upgrade_scroll(
    dungeon: &mut DungeonState,
    configured_profile: bool,
    items_to_spawn: &mut Vec<GeneratedItem>,
    forced: &mut Vec<GeneratedItem>,
) -> bool {
    if !dungeon.sou_needed() {
        return false;
    }

    dungeon.limited.upgrade_scrolls += 1;
    let every_second_scroll = dungeon.limited.upgrade_scrolls % 2 == 0;
    let unresolved_challenge = !configured_profile && every_second_scroll;
    if dungeon.forbidden_runes() && every_second_scroll {
        return unresolved_challenge;
    }

    let mut scroll = GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll);
    scroll.source = Some("forced".into());
    scroll.provenance = ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
        forbidden_runes_sensitive: unresolved_challenge,
    });
    items_to_spawn.push(scroll.clone());
    forced.push(scroll);
    unresolved_challenge
}
