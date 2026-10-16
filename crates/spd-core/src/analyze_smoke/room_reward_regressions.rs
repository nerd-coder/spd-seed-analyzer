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

    // The relocated forced prize is an Iron Key: level plumbing, not loot.
    assert!(ring_room_prizes.is_empty());
    assert!(!floor.items.iter().any(|item| {
        item.name == "conditional guaranteed item" && item.source.as_deref() == Some("RingRoom")
    }));

    let wealth_floor_drop = floor
        .items
        .iter()
        .find(|item| {
            item.class_name() == Some("RingOfWealth")
                && item.source.as_deref() == Some("CrystalVaultRoom")
        })
        .expect("separate Ring of Wealth crystal-vault prize");
    assert_eq!(
        wealth_floor_drop.prediction,
        report::ItemPredictionKind::Exact
    );
}

#[test]
fn keys_and_pages_never_reach_public_item_lists() {
    const NON_ITEMS: [&str; 7] = [
        "IronKey",
        "GoldenKey",
        "CrystalKey",
        "AlchemyPage",
        "GuidePage",
        "DocumentPage",
        "TornPage",
    ];
    for seed in ["HDN-UJD-MJF", "MWH-KAE-DHG", "GFX-PZH-DCH"] {
        let report = analyze_seed(seed, 26).expect("analyze");
        for floor in &report.floors {
            for item in floor.item_variants() {
                assert!(
                    !item
                        .class_name()
                        .is_some_and(|class| NON_ITEMS.contains(&class)),
                    "{seed} floor {} publishes {:?}",
                    floor.depth,
                    item.class_name()
                );
            }
        }
    }
}

#[test]
fn floor_two_room_contracts_pair_seed_constraints_with_variable_items() {
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
        report::ItemPredictionKind::Variable
    );
    assert_eq!(hidden.variants[1].class_name(), Some("WandOfTransfusion"));

    let honeypot_bomb = floor
        .items
        .iter()
        .find(|item| item.source.as_deref() == Some("SecretHoneypotRoom:bomb"))
        .expect("Secret Honeypot bomb reward");
    assert_eq!(honeypot_bomb.variants.len(), 2);
    assert_eq!(
        honeypot_bomb.variants[0].candidate_classes(),
        ["Bomb", "DoubleBomb"]
    );
    assert_eq!(
        honeypot_bomb.variants[1].prediction,
        report::ItemPredictionKind::Variable
    );
    assert_eq!(honeypot_bomb.variants[1].class_name(), Some("Bomb"));

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
    assert_eq!(grave_prize.variants[1].class_name(), Some("StoneOfBlink"));
    assert_eq!(
        grave_prize.variants[1].prediction,
        report::ItemPredictionKind::Variable
    );
    assert!(floor.items.iter().any(|item| {
        item.name == "2 Grassy Grave gold rewards (50–100 gold each)"
            && item.source.as_deref() == Some("GrassyGraveRoom:gold_tombs")
    }));

    assert_eq!(
        floor
            .items
            .iter()
            .filter(|item| {
                item.source.as_deref() == Some("SecretHoneypotRoom")
                    && matches!(item.class_name(), Some("ShatteredPot" | "Honeypot"))
            })
            .count(),
        2
    );
}

#[test]
fn room_prize_pulled_from_spawn_queue_is_published_once_at_its_room() {
    let report = analyze_seed("HDN-UJD-MJF", 4).expect("analyze");
    let floor = &report.floors[3];
    let strength: Vec<_> = floor
        .item_variants()
        .filter(|item| item.class_name() == Some("PotionOfStrength"))
        .map(|item| (item.source.as_deref(), item.prediction))
        .collect();
    assert_eq!(
        strength,
        [(Some("RingRoom:forced"), report::ItemPredictionKind::Exact)]
    );
}

#[test]
fn crystal_vault_pairs_contract_with_both_default_profile_prizes() {
    let report = analyze_seed("HDN-UJD-MJF", 4).expect("analyze");
    let floor = &report.floors[3];
    let vault = floor
        .items
        .iter()
        .find(|item| item.source.as_deref() == Some("CrystalVaultRoom"))
        .expect("Crystal Vault");
    assert_eq!(vault.variants.len(), 3);
    assert_eq!(vault.variants[0].name, "two crystal-vault reward sources");
    assert_eq!(
        vault.variants[0].prediction,
        report::ItemPredictionKind::Constrained
    );
    let samples: Vec<_> = vault
        .variants
        .iter()
        .skip(1)
        .map(|item| (item.class_name(), item.prediction, item.cursed, item.level))
        .collect();
    assert_eq!(
        samples,
        [
            (
                Some("UnstableSpellbook"),
                report::ItemPredictionKind::Variable,
                Some(false),
                Some(0),
            ),
            (
                Some("RingOfEvasion"),
                report::ItemPredictionKind::Variable,
                Some(true),
                Some(0),
            ),
        ]
    );
}

fn public_catalyst_groups(floor: &report::FloorReport) -> Vec<&report::ItemGroup> {
    floor
        .items
        .iter()
        .filter(|item| item.class_name() == Some("TrinketCatalyst"))
        .collect()
}

#[test]
fn floor_one_magical_fire_catalyst_keeps_canonical_name_and_offers() {
    let report = analyze_seed("NCZ-LFD-SCX", 4).expect("analyze");
    let floor = &report.floors[0];
    let catalyst = floor
        .items
        .iter()
        .find(|item| {
            item.source.as_deref() == Some("MagicalFireRoom")
                && item.class_name() == Some("TrinketCatalyst")
        })
        .expect("floor-one MagicalFireRoom catalyst");

    assert_eq!(catalyst.name, "Trinket Catalyst");
    assert_eq!(catalyst.prediction, report::ItemPredictionKind::Exact);
    assert_eq!(
        report.trinket_selection.catalyst_options,
        [
            "ShardOfOblivion",
            "CrackedSpyglass",
            "ParchmentScrap",
            "RatSkull"
        ]
    );
    assert_eq!(
        catalyst.candidates,
        report.trinket_selection.catalyst_options
    );
    assert_eq!(public_catalyst_groups(floor).len(), 1);
    assert!(
        report.floors[1..]
            .iter()
            .all(|floor| public_catalyst_groups(floor).is_empty()),
        "catalyst must not reappear after MagicalFireRoom consumes it"
    );
}

#[test]
fn floor_one_public_catalyst_copies_seed_offers() {
    let mut seen = 0;
    for seed in 0..40 {
        let report = analyze_seed(&seed.to_string(), 4).expect("analyze");
        if report.trinket_selection.catalyst_depth != 1 {
            continue;
        }
        let catalysts = public_catalyst_groups(&report.floors[0]);
        assert_eq!(catalysts.len(), 1, "seed {seed} floor-one catalyst group");
        let catalyst = catalysts[0];
        assert_eq!(catalyst.name, "Trinket Catalyst", "seed {seed}");
        assert_eq!(
            catalyst.class_name(),
            Some("TrinketCatalyst"),
            "seed {seed}"
        );
        assert_eq!(
            catalyst.prediction,
            report::ItemPredictionKind::Exact,
            "seed {seed}"
        );
        assert_eq!(
            catalyst.candidates, report.trinket_selection.catalyst_options,
            "seed {seed}"
        );
        assert!(
            report.floors[1..]
                .iter()
                .all(|floor| public_catalyst_groups(floor).is_empty()),
            "seed {seed} duplicate catalyst"
        );
        seen += 1;
    }
    assert!(seen > 0, "expected a depth-1 catalyst among 40 seeds");
}
