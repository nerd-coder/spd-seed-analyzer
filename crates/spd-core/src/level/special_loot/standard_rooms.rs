//! RNG-visible center loot for generic standard rooms.

mod halls_graves;

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::{TerrainMap, PEDESTAL};
use crate::random::Random;
use crate::rooms::room::Room;

use super::placement::find_prize_item;

pub(super) fn paint_center_loot(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    center: Option<Point>,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    match (room.name.as_str(), center) {
        ("StudyRoom", Some(_)) => vec![study_prize(dungeon, items_to_spawn)],
        ("RitualRoom", Some(center)) => {
            halls_graves::ritual_prize(dungeon, map, center, items_to_spawn)
        }
        ("RingRoom", Some(center)) => ring_prize(map, center, items_to_spawn),
        ("GrassyGraveRoom", _) => halls_graves::grassy_graves(dungeon, room, map),
        ("SuspiciousChestRoom", _) => suspicious_chest(dungeon, room, map, items_to_spawn),
        _ => Vec::new(),
    }
}

fn ring_prize(
    map: &mut TerrainMap,
    center: Point,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let Some(mut prize) = find_prize_item(items_to_spawn, None) else {
        return Vec::new();
    };
    prize.source = Some("RingRoom".into());
    if let Some(cell) = map.point_to_cell(center.x, center.y) {
        // Java's heap occupies the center only when `findPrizeItem` succeeded.
        map.item_allowed[cell] = false;
    }
    vec![PlacedLoot {
        item: prize,
        heap_type: "heap",
    }]
}

fn study_prize(dungeon: &mut DungeonState, items_to_spawn: &mut Vec<GeneratedItem>) -> PlacedLoot {
    let mut prize = if Random::int_max(2) == 0 {
        find_prize_item(items_to_spawn, None)
    } else {
        None
    }
    .unwrap_or_else(|| {
        let category = *Random::one_of(&[Category::Potion, Category::Scroll]);
        dungeon.generator.random_category(category, dungeon.depth)
    });
    prize.source = Some("StudyRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "heap",
    }
}

fn suspicious_chest(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut prize = find_prize_item(items_to_spawn, None).unwrap_or_else(|| {
        let mut gold = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut gold, dungeon.depth);
        gold
    });

    // Unlike StudyRoom, SuspiciousChestRoom selects its prize before center().
    let center = room.as_rect().center_room();
    if let Some(cell) = map.point_to_cell(center.x, center.y) {
        map.map[cell] = PEDESTAL;
        map.item_allowed[cell] = false;
    }

    if Random::float() < 1.0 / 3.0 {
        prize.source = Some("SuspiciousChestRoom:mimic".into());
        let mut reward = mimic_reward(dungeon);
        reward.source = Some("SuspiciousChestRoom:mimic".into());
        vec![
            PlacedLoot {
                item: prize,
                heap_type: "mimic",
            },
            PlacedLoot {
                item: reward,
                heap_type: "mimic",
            },
        ]
    } else {
        prize.source = Some("SuspiciousChestRoom".into());
        vec![PlacedLoot {
            item: prize,
            heap_type: "chest",
        }]
    }
}

/// `Mimic.generatePrize(true)`; analyzer runs have no item-blocking challenges.
fn mimic_reward(dungeon: &mut DungeonState) -> GeneratedItem {
    match Random::int_max(5) {
        0 => {
            let mut gold = GeneratedItem::new("Gold", ItemCategory::Gold);
            randomize_item(&mut gold, dungeon.depth);
            gold
        }
        1 => dungeon
            .generator
            .random_missile(dungeon.depth / 5, false, dungeon.depth),
        2 => dungeon
            .generator
            .random_armor(dungeon.depth / 5, dungeon.depth),
        3 => dungeon
            .generator
            .random_weapon(dungeon.depth / 5, false, dungeon.depth),
        _ => dungeon
            .generator
            .random_category(Category::Ring, dungeon.depth),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::types::RoomKind;
    use crate::run::{dungeon_from_run, init_run};

    fn room(name: &str) -> Room {
        let mut room = Room::new(0, name, RoomKind::Standard, 1, 16, 7, 10, 7, 10);
        room.left = 1;
        room.top = 1;
        room.right = 9;
        room.bottom = 9;
        room
    }

    fn map(room: &Room) -> TerrainMap {
        crate::level::terrain::paint_minimal(std::slice::from_ref(room)).expect("map")
    }

    #[test]
    fn study_consumes_a_forced_prize_when_selected() {
        Random::reset_generators();
        let run = init_run(1);
        let room = room("StudyRoom");
        let mut selected = None;
        for seed in 0..32 {
            let mut dungeon = dungeon_from_run(run.clone());
            dungeon.depth = 4;
            let mut items = vec![GeneratedItem::new("PotionOfStrength", ItemCategory::Potion)];
            let mut map = map(&room);
            Random::push_generator_seeded(seed);
            let loot = paint_center_loot(
                &mut dungeon,
                &room,
                &mut map,
                Some(Point::new(5, 5)),
                &mut items,
            );
            Random::pop_generator();
            if items.is_empty() {
                selected = loot.into_iter().next();
                break;
            }
        }

        let loot = selected.expect("a seed should select the forced prize");
        assert_eq!(loot.item.class_name, "PotionOfStrength");
        assert_eq!(loot.item.source.as_deref(), Some("StudyRoom"));
        assert_eq!(loot.item.category, ItemCategory::Potion);
    }

    #[test]
    fn suspicious_chest_mimic_generates_an_extra_reward() {
        Random::reset_generators();
        let run = init_run(9);
        let room = room("SuspiciousChestRoom");
        let mut mimic_case = None;
        for seed in 0..64 {
            let mut dungeon = dungeon_from_run(run.clone());
            dungeon.depth = 8;
            let mut map = map(&room);
            Random::push_generator_seeded(seed);
            let loot = paint_center_loot(&mut dungeon, &room, &mut map, None, &mut Vec::new());
            Random::pop_generator();
            if loot.len() == 2 {
                mimic_case = Some((loot, map));
                break;
            }
        }

        let (loot, map) = mimic_case.expect("a seed should produce the one-third mimic case");
        assert_eq!(loot[0].item.class_name, "Gold");
        assert!(loot.iter().all(|drop| {
            drop.item
                .source
                .as_deref()
                .is_some_and(|source| source.starts_with("SuspiciousChestRoom"))
        }));
        assert!(loot.iter().all(|drop| drop.heap_type == "mimic"));
        assert_eq!(map.map.iter().filter(|&&tile| tile == PEDESTAL).count(), 1);
    }

    #[test]
    fn ring_center_only_becomes_occupied_when_prize_exists() {
        Random::reset_generators();
        let run = init_run(4);
        let room = room("RingRoom");
        let center = Point::new(5, 5);

        let mut dungeon = dungeon_from_run(run.clone());
        dungeon.depth = 3;
        let mut ring_map = map(&room);
        let cell = ring_map.point_to_cell(center.x, center.y).expect("center");
        let mut items = vec![GeneratedItem::new("Food", ItemCategory::Food)];
        let loot = paint_center_loot(&mut dungeon, &room, &mut ring_map, Some(center), &mut items);
        assert_eq!(loot.len(), 1);
        assert_eq!(loot[0].item.source.as_deref(), Some("RingRoom"));
        assert!(!ring_map.item_allowed[cell]);

        let mut dungeon = dungeon_from_run(run);
        dungeon.depth = 3;
        let mut empty_map = map(&room);
        let loot = paint_center_loot(
            &mut dungeon,
            &room,
            &mut empty_map,
            Some(center),
            &mut Vec::new(),
        );
        assert!(loot.is_empty());
        assert!(empty_map.item_allowed[cell]);
    }
}
