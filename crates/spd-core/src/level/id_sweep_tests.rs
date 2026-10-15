//! Every class name the generator emits must have a `spd-objects` variant.
//! `Unknown` (id 0) on the wire means a list in `spd-objects` is incomplete.

use std::collections::BTreeSet;

use serde_json::Value;

use super::*;
use crate::run::{dungeon_from_run, init_run};

const ID_KEYS: &[&str] = &[
    "item",
    "items",
    "candidates",
    "fixed_item",
    "required_item",
    "mob",
    "room",
    "rooms",
    "trap",
    "plant",
    "blob",
    "source",
    "type",
    "enchantment",
    "catalyst_options",
    "transmutation_sequence",
];

fn collect_unknown(value: &Value, path: &str, found: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let child_path = format!("{path}.{key}");
                let is_id_key = ID_KEYS.contains(&key.as_str());
                match child {
                    Value::Number(number) if is_id_key && number.as_u64() == Some(0) => {
                        found.insert(child_path);
                    }
                    Value::Array(values) if is_id_key => {
                        for value in values {
                            if value.as_u64() == Some(0) {
                                found.insert(child_path.clone());
                            } else {
                                collect_unknown(value, &child_path, found);
                            }
                        }
                    }
                    _ => collect_unknown(child, &child_path, found),
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_unknown(value, path, found);
            }
        }
        _ => {}
    }
}

fn sweep_seed_count() -> i64 {
    std::env::var("SPD_ID_SWEEP_SEEDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(6)
}

#[test]
fn internal_maps_and_branches_never_emit_unknown_ids() {
    let mut found = BTreeSet::new();
    for seed in 0..sweep_seed_count() {
        let mut dungeon = dungeon_from_run(init_run(seed * 7_919));
        trinkets::reset(dungeon.seed);
        for depth in 1..=26 {
            dungeon.depth = depth;
            dungeon.branch = 0;
            trinkets::set_held(None);
            let level = create_level_partial_with_profile(&mut dungeon, true);
            let mut report = level.to_floor_report_with_trinket_availability(true, 1);
            replay::attach_side_branches(&dungeon, true, &mut report);
            let context = format!("seed {} depth {depth}", seed * 7_919);
            collect_unknown(
                &serde_json::to_value(&level.map).unwrap(),
                &format!("{context} internal map"),
                &mut found,
            );
            collect_unknown(
                &serde_json::to_value(&report).unwrap(),
                &format!("{context} report"),
                &mut found,
            );
        }
    }
    assert!(found.is_empty(), "Unknown ids: {found:#?}");
}

#[test]
fn analyzed_reports_never_emit_unknown_ids() {
    let mut found = BTreeSet::new();
    for seed in ["AAA-AAA-AAA", "0", "RZN-LKU-EFS"] {
        let report = crate::analyze_seed(seed, 26).expect("analyze");
        collect_unknown(&serde_json::to_value(&report).unwrap(), seed, &mut found);
    }
    assert!(found.is_empty(), "Unknown ids: {found:#?}");
}
