//! Focused coverage for room-paint additions to `itemsToSpawn`.

use super::super::secret_rooms::{secret_laboratory, secret_larder, secret_runestone};
use super::super::special_rooms::{
    armory_prizes_on_map, crypt_prize, library_prizes, paint_laboratory, pool_prize,
    runestone_prizes, statue_weapon, treasury_prizes_on_map,
};
use super::test_room;
use crate::geom::Point;
use crate::items::model::ItemCategory;
use crate::level::painter::DoorMap;
use crate::level::terrain::paint_minimal;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};

#[test]
fn runestone_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(31);
    Random::push_generator_seeded(101);
    let mut d = dungeon_from_run(run);
    d.depth = 7;
    let room = test_room("RunestoneRoom", 8, 8);
    let mut spawn = Vec::new();
    let loot = runestone_prizes(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert!((2..=3).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("RunestoneRoom")));
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn library_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(32);
    Random::push_generator_seeded(102);
    let mut d = dungeon_from_run(run);
    d.depth = 4;
    let room = test_room("LibraryRoom", 8, 8);
    let mut spawn = Vec::new();
    let loot = library_prizes(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert!((1..=3).contains(&loot.len()));
    assert!(matches!(
        loot[0].item.class_name.as_str(),
        "ScrollOfIdentify" | "ScrollOfRemoveCurse"
    ));
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn treasury_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(33);
    Random::push_generator_seeded(103);
    let mut d = dungeon_from_run(run);
    d.depth = 9;
    let room = test_room("TreasuryRoom", 8, 8);
    let mut spawn = Vec::new();
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("test map");
    let loot = treasury_prizes_on_map(&mut d, &room, &mut map, &mut spawn);
    Random::pop_generator();

    assert!((2..=3).contains(&loot.len()));
    assert!(loot.iter().all(|p| p
        .item
        .source
        .as_deref()
        .is_some_and(|s| s.starts_with("TreasuryRoom"))));
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn laboratory_room_records_fresh_prison_chapter_loot_blob_and_key() {
    Random::reset_generators();
    let run = init_run(34);
    Random::push_generator_seeded(104);
    let mut d = dungeon_from_run(run);
    d.depth = 8;
    let mut room = test_room("LaboratoryRoom", 8, 8);
    room.connected.push(1);
    let mut neighbour = Room::new(1, "TunnelRoom", RoomKind::Connection, 1, 16, 3, 10, 3, 10);
    neighbour.left = 8;
    neighbour.top = 2;
    neighbour.right = 12;
    neighbour.bottom = 6;
    neighbour.connected.push(0);
    let mut map = paint_minimal(&[room.clone(), neighbour]).expect("test map");
    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, Point::new(8, 4));
    let mut spawn = Vec::new();
    let loot = paint_laboratory(&mut d, &room, 0, &mut map, &doors, &mut spawn);
    Random::pop_generator();

    assert!((4..=5).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("LaboratoryRoom")));
    assert_eq!(
        loot.iter()
            .filter(|drop| drop.item.class_name == "AlchemyPage")
            .count(),
        2
    );
    assert_eq!(
        loot.iter()
            .filter(|drop| drop.item.class_name == "EnergyCrystal")
            .count(),
        1
    );
    assert_eq!(map.known_blobs.len(), 1);
    assert_eq!(map.known_blobs[0].class_name, "Alchemy");
    assert_eq!(map.known_blobs[0].cells.len(), 1);
    assert_eq!(
        map.known_heaps.iter().filter(|heap| heap.is_some()).count(),
        loot.len()
    );
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn crypt_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(35);
    Random::push_generator_seeded(105);
    let mut d = dungeon_from_run(run);
    d.depth = 11;
    let mut spawn = Vec::new();
    let loot = crypt_prize(&mut d, &mut spawn);
    Random::pop_generator();

    assert_eq!(loot.heap_type, "tomb");
    assert_eq!(loot.item.source.as_deref(), Some("CryptRoom"));
    assert!(loot.item.cursed);
    assert_eq!(loot.item.category, ItemCategory::Armor);
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn statue_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(36);
    Random::push_generator_seeded(106);
    let mut d = dungeon_from_run(run);
    d.depth = 8;
    let room = test_room("StatueRoom", 7, 7);
    let mut spawn = Vec::new();
    let loot = statue_weapon(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert_eq!(loot.heap_type, "statue");
    assert_eq!(loot.item.source.as_deref(), Some("StatueRoom"));
    assert!(!loot.item.cursed);
    assert_eq!(loot.item.category, ItemCategory::Weapon);
    assert!(loot.item.enchantment.is_some());
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn armory_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(37);
    Random::push_generator_seeded(107);
    let mut d = dungeon_from_run(run);
    d.depth = 13;
    let room = test_room("ArmoryRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("map");
    let entrance = crate::geom::Point::new(room.left, room.top + 2);
    let mut spawn = Vec::new();
    let loot = armory_prizes_on_map(&mut d, &room, &mut map, entrance, &mut spawn);
    Random::pop_generator();

    assert!((2..=3).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("ArmoryRoom")));
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "IronKey");
    assert_eq!(spawn[0].category, ItemCategory::Other);
}

#[test]
fn pool_room_pushes_invisibility() {
    Random::reset_generators();
    let run = init_run(38);
    Random::push_generator_seeded(108);
    let mut d = dungeon_from_run(run);
    d.depth = 6;
    let room = test_room("PoolRoom", 8, 8);
    let mut spawn = Vec::new();
    let loot = pool_prize(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert_eq!(loot.heap_type, "chest");
    assert_eq!(loot.item.source.as_deref(), Some("PoolRoom"));
    assert!(!loot.item.cursed);
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "PotionOfInvisibility");
    assert_eq!(spawn[0].category, ItemCategory::Potion);
}

#[test]
fn secret_runestone_pushes_liquid_flame() {
    Random::reset_generators();
    let run = init_run(39);
    Random::push_generator_seeded(109);
    let mut d = dungeon_from_run(run);
    d.depth = 10;
    let room = test_room("SecretRunestoneRoom", 8, 8);
    let mut spawn = Vec::new();
    let loot = secret_runestone(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert!((2..=3).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("SecretRunestoneRoom")));
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "PotionOfLiquidFlame");
    assert_eq!(spawn[0].category, ItemCategory::Potion);
}

#[test]
fn secret_laboratory_no_iron_key() {
    // Java SecretLaboratoryRoom extends SecretRoom with its own paint() that
    // never calls addItemToSpawn — reusing the lab prize body must not append
    // the LaboratoryRoom IronKey here.
    Random::reset_generators();
    let _run = init_run(40);
    Random::push_generator_seeded(110);
    let room = test_room("SecretLaboratoryRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("secret lab map");
    let loot = secret_laboratory(&room, &mut map);
    Random::pop_generator();

    assert!((2..=3).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("SecretLaboratoryRoom")));
    assert_eq!(
        map.known_heaps
            .iter()
            .flatten()
            .flat_map(|heap| &heap.items)
            .filter(|item| item.class_name == "EnergyCrystal")
            .count(),
        2
    );
}

#[test]
fn secret_larder_uses_pinned_food_energy_units() {
    let cases = [
        (4, vec!["ChargrilledMeat"]),
        (6, vec!["ChargrilledMeat", "ChargrilledMeat"]),
        (10, vec!["Pasty"]),
    ];

    for (depth, expected) in cases {
        Random::reset_generators();
        Random::push_generator_seeded(200 + i64::from(depth));
        let room = test_room("SecretLarderRoom", 8, 8);
        let mut map = paint_minimal(std::slice::from_ref(&room)).expect("secret larder map");

        let loot = secret_larder(depth, &room, &mut map);

        Random::pop_generator();
        assert_eq!(
            loot.iter()
                .map(|drop| drop.item.class_name.as_str())
                .collect::<Vec<_>>(),
            expected,
            "depth {depth} food composition"
        );
        assert_eq!(
            map.known_heaps.iter().flatten().count(),
            loot.len(),
            "depth {depth} heap count"
        );
    }
}
