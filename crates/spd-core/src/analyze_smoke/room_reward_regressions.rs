use crate::{analyze_seed, report};

#[test]
fn floor_one_ring_room_contract_collapses_into_its_exact_forced_reward() {
    let report = analyze_seed("MWH-KAE-DHG", 1).expect("analyze");
    let floor = &report.floors[0];
    let ring_room_prizes: Vec<_> = floor
        .items
        .iter()
        .filter(|item| item.source.as_deref() == Some("RingRoom"))
        .collect();

    assert_eq!(ring_room_prizes.len(), 1);
    assert_eq!(ring_room_prizes[0].class_name.as_deref(), Some("Pasty"));
    assert_eq!(
        ring_room_prizes[0].prediction,
        report::ItemPredictionKind::Exact
    );
    assert!(!floor.items.iter().any(|item| {
        item.name == "conditional guaranteed item" && item.source.as_deref() == Some("RingRoom")
    }));

    let wealth_floor_drop = floor
        .items
        .iter()
        .find(|item| {
            item.class_name.as_deref() == Some("RingOfWealth")
                && item.source.as_deref() == Some("heap")
        })
        .expect("separate Ring of Wealth floor drop");
    assert_eq!(
        wealth_floor_drop.prediction,
        report::ItemPredictionKind::Exact
    );
}

#[test]
fn floor_two_room_contracts_pair_seed_constraints_with_fresh_baselines() {
    let report = analyze_seed("MWH-KAE-DHG", 2).expect("analyze");
    let floor = &report.floors[1];

    let hidden = floor
        .items
        .iter()
        .find(|item| item.source.as_deref() == Some("CrystalChoiceRoom:hidden_reward"))
        .expect("Crystal Choice hidden reward");
    assert_eq!(hidden.variants.len(), 2);
    assert_eq!(hidden.variants[0].name, "hidden crystal-choice reward");
    assert_eq!(
        hidden.variants[1].prediction,
        report::ItemPredictionKind::Baseline
    );
    assert_eq!(
        hidden.variants[1].class_name.as_deref(),
        Some("WandOfTransfusion")
    );

    let honeypot_bomb = floor
        .items
        .iter()
        .find(|item| item.source.as_deref() == Some("SecretHoneypotRoom:bomb"))
        .expect("Secret Honeypot bomb reward");
    assert_eq!(honeypot_bomb.variants.len(), 2);
    assert_eq!(
        honeypot_bomb.variants[0].candidate_classes,
        ["Bomb", "DoubleBomb"]
    );
    assert_eq!(
        honeypot_bomb.variants[1].prediction,
        report::ItemPredictionKind::Baseline
    );
    assert_eq!(
        honeypot_bomb.variants[1].class_name.as_deref(),
        Some("Bomb")
    );

    let grave_prize = floor
        .items
        .iter()
        .find(|item| item.source.as_deref() == Some("GrassyGraveRoom:prize"))
        .expect("Grassy Grave general reward");
    assert_eq!(grave_prize.variants.len(), 2);
    assert_eq!(
        grave_prize.variants[0].name,
        "Grassy Grave Generator reward"
    );
    assert_eq!(grave_prize.variants[1].class_name.as_deref(), Some("Gold"));
    assert_eq!(grave_prize.variants[1].quantity, 62);
    assert!(floor.items.iter().any(|item| {
        item.name == "1 Grassy Grave gold reward (50–100 gold)"
            && item.source.as_deref() == Some("GrassyGraveRoom:gold_tombs")
    }));

    assert_eq!(
        floor
            .items
            .iter()
            .filter(|item| {
                item.source.as_deref() == Some("SecretHoneypotRoom")
                    && matches!(
                        item.class_name.as_deref(),
                        Some("ShatteredPot" | "Honeypot")
                    )
            })
            .count(),
        2
    );
}
