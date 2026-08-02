use crate::report::{ItemEntry, ItemPredictionKind};
use crate::rooms::room::Room;

pub(super) fn reward_count(room: &Room) -> u32 {
    let width = room.width() - 2;
    let height = room.height() - 2;
    (width.max(height) / 2) as u32
}

pub(super) fn entries(reward_count: u32, depth: i32) -> Vec<ItemEntry> {
    let gold_count = reward_count.saturating_sub(1);
    let gold_min = 30 + depth * 10;
    let gold_max = 60 + depth * 20;
    let gold_name = if gold_count == 1 {
        format!("1 Grassy Grave gold reward ({gold_min}–{gold_max} gold)")
    } else {
        format!("{gold_count} Grassy Grave gold rewards ({gold_min}–{gold_max} gold each)")
    };
    let note = format!(
        "Seed-determined room dimensions guarantee {reward_count} tomb rewards: one general Generator reward and {gold_count} gold rewards of {gold_min}–{gold_max} gold each."
    );
    vec![
        ItemEntry {
            name: "Grassy Grave Generator reward".into(),
            quantity: 1,
            class_name: None,
            candidate_classes: Vec::new(),
            category: "other".into(),
            tier: None,
            tier_range: None,
            level: None,
            level_range: None,
            cursed: None,
            enchantment: None,
            prediction: ItemPredictionKind::Constrained,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec![note.clone()],
            source: Some("GrassyGraveRoom:prize".into()),
        },
        ItemEntry {
            name: gold_name,
            quantity: 1,
            class_name: None,
            candidate_classes: Vec::new(),
            category: "gold".into(),
            tier: None,
            tier_range: None,
            level: Some(0),
            level_range: None,
            cursed: Some(false),
            enchantment: None,
            prediction: ItemPredictionKind::Constrained,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: vec![note],
            source: Some("GrassyGraveRoom:gold_tombs".into()),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::types::RoomKind;

    #[test]
    fn contract_uses_the_seed_determined_room_dimensions() {
        let mut room = Room::new(
            0,
            "GrassyGraveRoom",
            RoomKind::Standard,
            1,
            16,
            7,
            10,
            7,
            10,
        );
        room.left = 1;
        room.top = 1;
        room.right = 10;
        room.bottom = 8;
        let entries = entries(reward_count(&room), 2);

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "Grassy Grave Generator reward");
        assert_eq!(entries[0].source.as_deref(), Some("GrassyGraveRoom:prize"));
        assert_eq!(
            entries[1].name,
            "3 Grassy Grave gold rewards (50–100 gold each)"
        );
        assert_eq!(
            entries[1].source.as_deref(),
            Some("GrassyGraveRoom:gold_tombs")
        );
    }
}
