use super::*;

#[test]
fn projection_emits_one_static_contract_and_never_the_sampled_weapon() {
    let mut floor = LevelState {
        depth: 13,
        feeling: Feeling::None,
        builder: None,
        rooms: vec!["SacrificeRoom".into()],
        room_bounds: vec![],
        pre_shuffle_room_bounds: vec![],
        build_ok: true,
        forced_items: vec![],
        initial_forced_items: vec![],
        placed_items: vec![],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        runtime_sensitive_map: false,
        runtime_sensitive_layout: false,
        runtime_sensitive_rooms: false,
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
        room_paint_rng_checkpoints: vec![],
        post_doors_rng_probe: vec![],
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

#[test]
fn floor_one_sacrifice_reward_is_an_exact_seed_fact() {
    let mut floor = LevelState {
        depth: 1,
        feeling: Feeling::None,
        builder: None,
        rooms: vec!["SacrificeRoom".into()],
        room_bounds: vec![],
        pre_shuffle_room_bounds: vec![],
        build_ok: true,
        forced_items: vec![],
        initial_forced_items: vec![],
        placed_items: vec![],
        runtime_sensitive_placed_items_from: None,
        runtime_sensitive_quests_from: None,
        quests: vec![],
        runtime_sensitive_map: false,
        runtime_sensitive_layout: false,
        runtime_sensitive_rooms: false,
        runtime_sensitive_feeling: false,
        room_public_facts: vec![super::super::super::room_public::RoomPublicFact::new(
            "SacrificeRoom",
            1,
        )
        .expect("Sacrifice contract")],
        complete: true,
        map: None,
        layout_map: None,
        pre_items_rng_probe: vec![],
        pre_mobs_rng_probe: vec![],
        pre_paint_rng_probe: vec![],
        room_paint_rng_checkpoints: vec![],
        post_doors_rng_probe: vec![],
    };
    let mut sampled = GeneratedItem::new("Mace", ItemCategory::Weapon);
    sampled.level = 1;
    sampled.cursed = true;
    sampled.enchantment = Some("Corrupting".into());
    sampled.source = Some("SacrificeRoom".into());
    floor.placed_items.push(sampled);

    let report = floor.to_floor_report();
    assert_eq!(
        report
            .items
            .iter()
            .filter(|item| item.source.as_deref() == Some("SacrificeRoom"))
            .collect::<Vec<_>>(),
        vec![&crate::report::ItemEntry {
            name: "corrupting mace +1".into(),
            quantity: 1,
            class_name: Some("Mace".into()),
            candidate_classes: Vec::new(),
            category: "weapon".into(),
            tier: None,
            tier_range: None,
            level: Some(1),
            level_range: None,
            cursed: Some(true),
            enchantment: None,
            prediction: crate::report::ItemPredictionKind::Exact,
            spawn_conditions: Vec::new(),
            conditions: Vec::new(),
            notes: Vec::new(),
            source: Some("SacrificeRoom".into()),
        }]
    );
}
