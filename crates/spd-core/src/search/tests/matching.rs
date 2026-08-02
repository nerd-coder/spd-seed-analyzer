use super::*;

#[test]
fn duplicate_constraints_require_distinct_item_occurrences() {
    let floor = exact_floor(3, &[("RingOfWealth", 2)]);
    let mut upgraded = constraint("RingOfWealth", 3, 3);
    upgraded.min_level = Some(2);

    let evidence = matching_evidence(
        &[floor],
        &[upgraded, constraint("RingOfWealth", 3, 3)],
        false,
    );

    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].constraint_index, 0);
}

#[test]
fn duplicate_constraints_find_a_distinct_upgraded_and_any_level_item() {
    let floor = exact_floor(3, &[("RingOfWealth", 2), ("RingOfWealth", 0)]);
    let mut upgraded = constraint("RingOfWealth", 3, 3);
    upgraded.min_level = Some(2);

    let evidence = matching_evidence(
        &[floor],
        &[constraint("RingOfWealth", 3, 3), upgraded],
        false,
    );

    assert_eq!(evidence.len(), 2);
    assert_eq!(evidence[0].constraint_index, 0);
    assert_eq!(evidence[0].level, 0);
    assert_eq!(evidence[1].constraint_index, 1);
    assert_eq!(evidence[1].level, 2);
}

#[test]
fn item_quantity_represents_distinct_matchable_occurrences() {
    let mut floor = exact_floor(3, &[("PotionOfHealing", 0)]);
    floor.items[0].variants[0].quantity = 2;
    let constraints = [
        constraint("PotionOfHealing", 3, 3),
        constraint("PotionOfHealing", 3, 3),
    ];

    assert_eq!(matching_evidence(&[floor], &constraints, false).len(), 2);
}

#[test]
fn alternative_variants_do_not_count_as_multiple_items() {
    let mut floor = exact_floor(3, &[("RingOfWealth", 2)]);
    let mut alternative = floor.items[0].variants[0].clone();
    alternative.name = "ring of wealth".into();
    alternative.prediction = ItemPredictionKind::Baseline;
    floor.items[0].variants.push(alternative);
    let constraints = [
        constraint("RingOfWealth", 3, 3),
        constraint("RingOfWealth", 3, 3),
    ];

    assert_eq!(matching_evidence(&[floor], &constraints, true).len(), 1);
}
