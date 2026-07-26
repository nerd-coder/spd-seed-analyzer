use super::*;
use crate::items::model::ForcedDropRole;

#[test]
fn public_projection_omits_every_regular_map_while_retaining_internal_maps() {
    for seed in 0..8 {
        let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(seed));
        for depth in 1..=4 {
            dungeon.depth = depth;
            let state = crate::level::create_level_partial(&mut dungeon);
            assert!(
                state.map.is_some(),
                "internal depth-{depth} map for seed {seed}"
            );
            assert!(state.to_floor_report().map.is_none());
        }
    }
}

#[test]
fn otherwise_untainted_regular_floor_keeps_constraints_without_a_public_map() {
    for seed in 0..256 {
        let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(seed));
        dungeon.depth = 3;
        let state = crate::level::create_level_partial(&mut dungeon);
        if state.map.is_some()
            && !state.runtime_sensitive_map
            && !state.runtime_sensitive_layout
            && !state.runtime_sensitive_feeling
        {
            let public = state.to_floor_report();
            assert!(public.map.is_none());
            assert!(public.items.iter().any(|item| {
                item.source.as_deref() == Some("initial forced queue")
                    && item.prediction == ItemPredictionKind::Constrained
            }));
            return;
        }
    }
    panic!("expected an otherwise-untainted depth-three regular floor");
}

#[test]
fn projection_ignores_sampled_food_identity_and_room_consumption() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    dungeon.depth = 1;
    let state = crate::level::create_level_partial(&mut dungeon);
    let original = state.to_floor_report();
    let mut altered = state.clone();
    let food = altered
        .initial_forced_items
        .iter_mut()
        .find(|item| item.provenance == ItemProvenance::Forced(ForcedDropRole::BaseFood))
        .expect("base food queue item");
    food.class_name = "MysteryMeat".into();
    altered.forced_items.clear();
    assert_eq!(
        serde_json::to_value(original).expect("original public report"),
        serde_json::to_value(altered.to_floor_report()).expect("altered public report")
    );
}

#[test]
fn halls_torches_precede_food_and_survive_public_taint_as_one_constraint() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    dungeon.public_generation_tainted = true;
    dungeon.depth = 21;
    let state = crate::level::create_level_partial(&mut dungeon);
    assert_eq!(state.initial_forced_items[0].class_name, "Torch");
    assert_eq!(state.initial_forced_items[1].class_name, "Torch");
    assert_eq!(
        state.initial_forced_items[0].provenance,
        ItemProvenance::Forced(ForcedDropRole::HallsTorch)
    );
    assert_eq!(
        state.initial_forced_items[1].provenance,
        ItemProvenance::Forced(ForcedDropRole::HallsTorch)
    );
    assert_eq!(
        state.initial_forced_items[2].provenance,
        ItemProvenance::Forced(ForcedDropRole::BaseFood)
    );
    let public = state.to_floor_report();
    let torch_entries: Vec<_> = public
        .items
        .iter()
        .filter(|item| item.name.contains("Torches"))
        .collect();
    assert_eq!(torch_entries.len(), 1);
    assert_eq!(torch_entries[0].prediction, ItemPredictionKind::Constrained);
    assert!(torch_entries[0].class_name.is_none());
    assert!(torch_entries[0].conditional_notes[0].contains("before the base food"));
    assert!(torch_entries[0].conditional_notes[0].contains("final cells are not asserted"));
}

#[test]
fn upgrade_scroll_contract_distinguishes_odd_and_even_schedules() {
    let mut odd = GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll);
    odd.provenance = ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
        forbidden_runes_sensitive: false,
    });
    let odd_entries = forced_public_entries(3, &[odd]);
    let odd_entry = odd_entries
        .iter()
        .find(|entry| entry.name.contains("Scroll of Upgrade"))
        .expect("odd Scroll contract");
    assert_eq!(odd_entry.name, "initially queued Scroll of Upgrade");
    assert!(odd_entry.conditional_notes[0].contains("even with Forbidden Runes"));

    let mut even = GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll);
    even.provenance = ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
        forbidden_runes_sensitive: true,
    });
    let even_entries = forced_public_entries(3, &[even]);
    let even_entry = even_entries
        .iter()
        .find(|entry| entry.name.contains("Scroll of Upgrade"))
        .expect("even Scroll contract");
    assert_eq!(
        even_entry.name,
        "conditional Scroll of Upgrade queued source"
    );
    assert!(even_entry.conditional_notes[0].contains("suppresses its enqueue"));
    assert!(odd_entry.class_name.is_none() && even_entry.class_name.is_none());
}

#[test]
fn even_scheduled_upgrade_scroll_suppresses_the_public_map() {
    for seed in 0..64 {
        let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(seed));
        for depth in 1..=4 {
            dungeon.depth = depth;
            let state = crate::level::create_level_partial(&mut dungeon);
            if state.initial_forced_items.iter().any(|item| {
                item.provenance
                    == ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
                        forbidden_runes_sensitive: true,
                    })
            }) {
                assert!(state.runtime_sensitive_map);
                assert!(state.to_floor_report().map.is_none());
                return;
            }
        }
    }
    panic!("expected an even scheduled Scroll of Upgrade");
}

#[test]
fn held_trinket_sensitive_default_feeling_hides_the_prebuild_floor_tail() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    let mut state = None;
    for depth in 1..=3 {
        dungeon.depth = depth;
        state = Some(crate::level::create_level_partial(&mut dungeon));
    }
    let state = state.expect("depth-three state");
    assert!(state.runtime_sensitive_feeling);
    assert!(state.map.is_some(), "internal exact map is retained");
    let public = state.to_floor_report();
    assert!(public.feeling.is_none());
    assert!(public.builder.is_none());
    assert!(public.rooms.is_empty());
    assert!(public.map.is_none());
    assert!(public.items.iter().all(|item| {
        item.source.as_deref() == Some("initial forced queue")
            && item.prediction == ItemPredictionKind::Constrained
    }));
}

#[test]
fn public_generation_taint_suppresses_later_floor_samples() {
    let mut tainted = crate::run::dungeon_from_run(crate::run::init_run(0));
    tainted.depth = 1;
    crate::level::create_level_partial(&mut tainted);
    assert!(tainted.public_generation_tainted);

    let mut clean_control = tainted.clone();
    clean_control.public_generation_tainted = false;
    tainted.depth = 2;
    clean_control.depth = 2;
    let tainted_floor = crate::level::create_level_partial(&mut tainted);
    let clean_floor = crate::level::create_level_partial(&mut clean_control);

    let public = tainted_floor.to_floor_report();
    assert!(
        tainted_floor.map.is_some(),
        "internal parity map is retained"
    );
    assert!(public.feeling.is_none());
    assert!(public.builder.is_none());
    assert!(public.rooms.is_empty());
    assert!(public.map.is_none());
    assert!(public.items.iter().all(|item| {
        item.source.as_deref() == Some("initial forced queue")
            && item.prediction == ItemPredictionKind::Constrained
    }));
    assert!(
        !clean_floor.to_floor_report().rooms.is_empty(),
        "the control proves suppression comes from inherited taint"
    );
}

#[test]
fn public_generation_taint_survives_a_nonregular_boss_floor() {
    let mut dungeon = crate::run::dungeon_from_run(crate::run::init_run(0));
    dungeon.public_generation_tainted = true;

    dungeon.depth = 5;
    let boss = crate::level::create_level_partial(&mut dungeon);
    assert!(boss.map.is_none(), "boss generation remains nonregular");
    assert!(dungeon.public_generation_tainted);

    dungeon.depth = 6;
    let prison = crate::level::create_level_partial(&mut dungeon);
    assert!(
        prison.map.is_some(),
        "internal regular-floor map is retained"
    );
    let public = prison.to_floor_report();
    assert!(public.feeling.is_none());
    assert!(public.builder.is_none());
    assert!(public.rooms.is_empty());
    assert!(public.map.is_none());
    assert!(public.items.iter().all(|item| {
        item.source.as_deref() == Some("initial forced queue")
            && item.prediction == ItemPredictionKind::Constrained
    }));
}
