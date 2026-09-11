//! Pinned Imp `VaultLevel` side-branch generation (rooms + GridBuilder only).

#![allow(dead_code)] // nested BranchFloorReport is PR 4; tests call generate now

mod loot;
mod rooms;

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::random::Random;
use crate::rooms::room::{clear_all_connections, Room};

use super::state::LevelRoomFact;

const BUILD_RETRY_LIMIT: u32 = 10_000;

#[derive(Debug, Clone)]
pub(crate) struct VaultLayout {
    pub rooms: Vec<LevelRoomFact>,
    pub dropped_before: [i32; 3],
    pub dropped_after: [i32; 3],
}

/// Forced branch-1 vault at `dungeon.depth`. Does not paint and does not emit
/// a public `BranchFloorReport` (those land in later PRs).
pub(crate) fn generate(dungeon: &mut DungeonState) -> Option<VaultLayout> {
    let depth = dungeon.depth;
    if !(17..=19).contains(&depth) {
        return None;
    }
    let depth_seed = crate::dungeon::seed_for_depth(dungeon.seed, depth, 1);
    Random::push_generator_seeded(depth_seed);
    let dropped_before = dropped_triple(&dungeon.generator);

    loot::queue_floor_loot(dungeon);
    let mut room_list = rooms::init();
    Random::shuffle_list(&mut room_list);
    for (id, room) in room_list.iter_mut().enumerate() {
        room.id = id;
    }

    let mut built = false;
    for _ in 0..BUILD_RETRY_LIMIT {
        clear_all_connections(&mut room_list);
        if crate::builders::build_grid_rooms(&mut room_list) {
            built = true;
            break;
        }
    }
    if !built {
        Random::pop_generator();
        return None;
    }

    // RegularPainter padding is 1 (vault is never CHASM). Bounds after this
    // shift match createMobs; paint itself is PR 3.
    shift_rooms(&mut room_list, 1);
    let dropped_after = dropped_triple(&dungeon.generator);
    Random::pop_generator();

    Some(VaultLayout {
        rooms: room_list
            .into_iter()
            .map(|room| LevelRoomFact {
                class_name: room.name,
                left: room.left,
                top: room.top,
                right: room.right,
                bottom: room.bottom,
            })
            .collect(),
        dropped_before,
        dropped_after,
    })
}

fn dropped_triple(generator: &crate::generator::GeneratorState) -> [i32; 3] {
    [
        generator.deck_dropped(Category::Ring),
        generator.deck_dropped(Category::Wand),
        generator.deck_dropped(Category::Artifact),
    ]
}

fn shift_rooms(rooms: &mut [Room], padding: i32) {
    let left = rooms.iter().map(|room| room.left).min().unwrap_or_default();
    let top = rooms.iter().map(|room| room.top).min().unwrap_or_default();
    for room in rooms {
        room.shift(padding - left, padding - top);
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;
    use crate::run::{dungeon_from_run, init_run};

    #[derive(Deserialize)]
    struct Fixture {
        schema_version: u32,
        contract: String,
        dropped: Dropped,
        rooms: Vec<FixtureRoom>,
    }

    #[derive(Deserialize)]
    struct Dropped {
        #[serde(rename = "RING")]
        ring: Counter,
        #[serde(rename = "WAND")]
        wand: Counter,
        #[serde(rename = "ARTIFACT")]
        artifact: Counter,
    }

    #[derive(Deserialize)]
    struct Counter {
        before: i32,
        after: i32,
    }

    #[derive(Deserialize)]
    struct FixtureRoom {
        #[serde(rename = "class")]
        class_name: String,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    fn sort_key(class_name: &str, left: i32, top: i32, right: i32, bottom: i32) -> String {
        format!("{class_name}:{left},{top},{right},{bottom}")
    }

    #[test]
    fn aaa_floor_seventeen_room_multiset_matches_java() {
        let expected: Fixture = serde_json::from_str(include_str!(
            "../../../../../tools/java-oracle/fixtures/vault/aaa-aaa-aaa-floor-17.json"
        ))
        .expect("vault fixture");
        assert_eq!(expected.schema_version, 1);
        assert_eq!(expected.contract, "vault_level_using_defaults");

        let mut dungeon = dungeon_from_run(init_run(0));
        dungeon.depth = 17;
        dungeon.branch = 1;
        dungeon.imp.spawned = true;
        let generated = generate(&mut dungeon).expect("vault rooms");

        assert_eq!(generated.dropped_before, [0, 0, 0]);
        assert_eq!(generated.dropped_after, [0, 0, 0]);
        assert_eq!(expected.dropped.ring.before, 0);
        assert_eq!(expected.dropped.ring.after, 0);
        assert_eq!(expected.dropped.wand.before, 0);
        assert_eq!(expected.dropped.wand.after, 0);
        assert_eq!(expected.dropped.artifact.before, 0);
        assert_eq!(expected.dropped.artifact.after, 0);

        let mut actual: Vec<_> = generated
            .rooms
            .iter()
            .map(|room| {
                sort_key(
                    &room.class_name,
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                )
            })
            .collect();
        let mut expected_rooms: Vec<_> = expected
            .rooms
            .iter()
            .map(|room| {
                sort_key(
                    &room.class_name,
                    room.left,
                    room.top,
                    room.right,
                    room.bottom,
                )
            })
            .collect();
        actual.sort();
        expected_rooms.sort();
        assert_eq!(actual, expected_rooms, "vault room-name multiset + bounds");
    }
}
