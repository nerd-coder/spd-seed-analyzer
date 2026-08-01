use super::*;
use crate::report::{ItemEntry, ItemPredictionKind};

fn exact_item(name: &str, class_name: &str, quantity: i32, level: i32) -> ItemEntry {
    ItemEntry {
        name: name.into(),
        quantity,
        class_name: Some(class_name.into()),
        candidate_classes: Vec::new(),
        category: "other".into(),
        tier: None,
        tier_range: None,
        level: Some(level),
        level_range: None,
        cursed: Some(false),
        enchantment: None,
        prediction: ItemPredictionKind::Exact,
        spawn_conditions: Vec::new(),
        conditions: Vec::new(),
        notes: Vec::new(),
        source: Some("ShopRoom".into()),
    }
}

#[test]
fn identical_exact_entries_merge_and_sum_stack_quantities() {
    let merged = merge_identical_items(vec![
        exact_item("Torch", "Torch", 1, 0),
        exact_item("Torch", "Torch", 1, 0),
        exact_item("Torch", "Torch", 3, 0),
    ]);

    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].quantity, 5);
}

#[test]
fn visually_distinct_entries_remain_separate() {
    let merged = merge_identical_items(vec![
        exact_item("Sword", "Sword", 1, 0),
        exact_item("Sword +1", "Sword", 1, 1),
    ]);

    assert_eq!(merged.len(), 2);
    assert!(merged.iter().all(|item| item.quantity == 1));
}

#[test]
fn unresolved_same_name_entries_do_not_claim_identical_items() {
    let mut first = exact_item("weapon stock", "Sword", 1, 0);
    first.class_name = None;
    first.prediction = ItemPredictionKind::Constrained;
    let second = first.clone();

    let merged = merge_identical_items(vec![first, second]);

    assert_eq!(merged.len(), 2);
}
