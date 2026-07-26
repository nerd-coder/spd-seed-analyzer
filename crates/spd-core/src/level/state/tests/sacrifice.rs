use super::*;

#[test]
fn projection_emits_one_static_contract_and_never_the_sampled_weapon() {
    let mut floor = LevelState {
        depth: 13,
        feeling: Feeling::None,
        builder: None,
        rooms: vec!["SacrificeRoom".into()],
        room_bounds: vec![],
        build_ok: true,
        forced_items: vec![],
        initial_forced_items: vec![],
        placed_items: vec![],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        quest_public_labels: vec![],
        runtime_sensitive_map: false,
        runtime_sensitive_layout: false,
        runtime_sensitive_feeling: false,
        room_public_facts: vec![super::super::super::room_public::RoomPublicFact::new(
            "SacrificeRoom",
            13,
        )
        .expect("Sacrifice contract")],
        complete: true,
        map: None,
        layout_map: None,
        pre_items_rng_probe: vec![],
        pre_mobs_rng_probe: vec![],
        pre_paint_rng_probe: vec![],
    };
    let mut sampled = GeneratedItem::new("Sword", ItemCategory::Weapon);
    sampled.level = 2;
    sampled.cursed = true;
    sampled.enchantment = Some("Corrupting".into());
    sampled.source = Some("SacrificeRoom".into());
    floor.placed_items.push(sampled);

    let report = floor.to_floor_report();
    let rewards: Vec<_> = report
        .items
        .iter()
        .filter(|item| item.source.as_deref() == Some("SacrificeRoom"))
        .collect();
    assert_eq!(rewards.len(), 1);
    assert_eq!(rewards[0].tier, None);
    assert_eq!(
        rewards[0].tier_range,
        Some(crate::report::NumericRange { min: 3, max: 5 })
    );
    assert_eq!(rewards[0].cursed, Some(true));
    assert_eq!(
        rewards[0].level_range,
        Some(crate::report::NumericRange { min: 0, max: 3 })
    );
    assert!(rewards[0].class_name.is_none());
    let json = serde_json::to_string(&report).expect("serialize Sacrifice projection");
    for secret in ["Sword", "Corrupting"] {
        assert!(!json.contains(secret), "leaked {secret}: {json}");
    }
}
