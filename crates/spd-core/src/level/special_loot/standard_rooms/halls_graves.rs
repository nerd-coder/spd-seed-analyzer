//! Standard-room loot for Ritual and GrassyGrave rooms.

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::placement::find_prize_item;

pub(super) fn ritual_prize(
    dungeon: &mut DungeonState,
    map: &mut TerrainMap,
    center: Point,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut prize = if Random::int_max(2) == 0 {
        find_prize_item(items_to_spawn, None)
    } else {
        None
    }
    .unwrap_or_else(|| {
        let category = *Random::one_of(&[Category::Potion, Category::Scroll]);
        dungeon.generator.random_category(category, dungeon.depth)
    });
    prize.source = Some("RitualRoom".into());
    if let Some(cell) = map.point_to_cell(center.x, center.y) {
        map.item_allowed[cell] = false;
    }
    vec![PlacedLoot {
        item: prize,
        heap_type: "heap",
    }]
}

pub(super) fn grassy_graves(
    dungeon: &mut DungeonState,
    room: &Room,
    map: &mut TerrainMap,
) -> Vec<PlacedLoot> {
    let width = room.width() - 2;
    let height = room.height() - 2;
    let grave_count = width.max(height) / 2;
    let prize_index = Random::int_max(grave_count);
    let shift = Random::int_max(2);
    let mut graves = Vec::with_capacity(grave_count as usize);

    for i in 0..grave_count {
        let (x, y) = if width > height {
            (
                room.left + 1 + shift + i * 2,
                room.top + 2 + Random::int_max(height - 2),
            )
        } else {
            (
                room.left + 2 + Random::int_max(width - 2),
                room.top + 1 + shift + i * 2,
            )
        };

        // Java evaluates the position before generating this grave's item.
        let mut item = if i == prize_index {
            dungeon.generator.random(dungeon.depth)
        } else {
            let mut gold = GeneratedItem::new("Gold", ItemCategory::Gold);
            randomize_item(&mut gold, dungeon.depth);
            gold
        };
        item.source = Some("GrassyGraveRoom".into());
        if let Some(cell) = map.point_to_cell(x, y) {
            map.item_allowed[cell] = false;
        }
        graves.push(PlacedLoot {
            item,
            heap_type: "tomb",
        });
    }
    graves
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
    fn ritual_prize_consumes_forced_item_but_variants_do_not() {
        Random::reset_generators();
        let run = init_run(13);
        let ritual = room("RitualRoom");
        let center = Point::new(5, 5);
        let mut selected = None;
        for seed in 0..32 {
            let mut dungeon = dungeon_from_run(run.clone());
            dungeon.depth = 23;
            let mut items = vec![GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll)];
            let mut map = map(&ritual);
            Random::push_generator_seeded(seed);
            let loot = ritual_prize(&mut dungeon, &mut map, center, &mut items);
            Random::pop_generator();
            if items.is_empty() {
                selected = Some((loot, map));
                break;
            }
        }
        let (loot, ritual_map) = selected.expect("a seed should select the forced ritual prize");
        assert_eq!(loot[0].item.class_name, "ScrollOfUpgrade");
        assert_eq!(loot[0].item.source.as_deref(), Some("RitualRoom"));
        assert!(
            !ritual_map.item_allowed[ritual_map
                .point_to_cell(center.x, center.y)
                .expect("center")]
        );

        let variant = room("RitualEntranceRoom");
        let mut dungeon = dungeon_from_run(run);
        dungeon.depth = 23;
        let items = [GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll)];
        let variant_map = map(&variant);
        Random::push_generator_seeded(99);
        let before = Random::long();
        Random::pop_generator();
        Random::push_generator_seeded(99);
        let after = Random::long();
        Random::pop_generator();
        assert_eq!(after, before);
        assert_eq!(items.len(), 1);
        assert!(variant_map.item_allowed.iter().all(|&allowed| allowed));
    }

    #[test]
    fn grassy_graves_generate_one_prize_and_occupy_every_tomb_cell() {
        Random::reset_generators();
        let run = init_run(21);
        let mut grave_room = room("GrassyGraveRoom");
        grave_room.right = 10;
        grave_room.bottom = 8;
        let mut dungeon = dungeon_from_run(run);
        dungeon.depth = 22;
        let mut map = map(&grave_room);

        Random::push_generator_seeded(0x6A4E);
        let loot = grassy_graves(&mut dungeon, &grave_room, &mut map);
        let next = Random::long();
        Random::pop_generator();

        assert_eq!(loot.len(), 4);
        assert!(loot.iter().all(|grave| {
            grave.heap_type == "tomb" && grave.item.source.as_deref() == Some("GrassyGraveRoom")
        }));
        assert_eq!(
            loot.iter()
                .filter(|grave| grave.item.class_name != "Gold")
                .count(),
            1
        );
        assert_eq!(
            map.item_allowed.iter().filter(|&&allowed| !allowed).count(),
            4
        );
        assert_eq!(next, -7_692_918_352_596_686_609);
    }
}
