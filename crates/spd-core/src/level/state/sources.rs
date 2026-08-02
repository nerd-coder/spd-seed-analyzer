use crate::items::model::{GeneratedItem, ItemProvenance, RoomLootRole};

fn specific_room_source(item: &GeneratedItem) -> Option<&'static str> {
    match item.provenance {
        ItemProvenance::Room(RoomLootRole::CrystalChoiceHidden) => {
            Some("CrystalChoiceRoom:hidden_reward")
        }
        ItemProvenance::Room(RoomLootRole::SecretHoneypotBomb) => Some("SecretHoneypotRoom:bomb"),
        ItemProvenance::Room(RoomLootRole::GrassyGravePrize) => Some("GrassyGraveRoom:prize"),
        ItemProvenance::Room(RoomLootRole::GrassyGraveGold) => Some("GrassyGraveRoom:gold_tombs"),
        _ => None,
    }
}

pub(super) fn reported_source(item: &GeneratedItem, exact_floor_one: bool) -> Option<String> {
    specific_room_source(item).map(str::to_string).or_else(|| {
        item.source.as_deref().map(|source| {
            if exact_floor_one {
                source.strip_suffix(":forced").unwrap_or(source).to_string()
            } else {
                source.to_string()
            }
        })
    })
}
