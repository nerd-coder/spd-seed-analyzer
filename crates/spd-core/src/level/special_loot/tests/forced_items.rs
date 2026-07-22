//! Focused coverage for room-paint additions to `itemsToSpawn`.

use super::super::secret_rooms::{secret_laboratory, secret_runestone};
use super::super::special_rooms::{
    armory_prizes_on_map, crypt_prize, laboratory_prizes, library_prizes, pool_prize,
    runestone_prizes, statue_weapon, treasury_prizes_on_map,
};
use super::test_room;
use crate::items::model::ItemCategory;
use crate::level::terrain::paint_minimal;
use crate::random::Random;
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
fn laboratory_room_pushes_iron_key() {
    Random::reset_generators();
    let run = init_run(34);
    Random::push_generator_seeded(104);
    let mut d = dungeon_from_run(run);
    d.depth = 12;
    let room = test_room("LaboratoryRoom", 8, 8);
    let mut spawn = Vec::new();
    let loot = laboratory_prizes(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert!((1..=2).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("LaboratoryRoom")));
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
    let run = init_run(40);
    Random::push_generator_seeded(110);
    let mut d = dungeon_from_run(run);
    d.depth = 11;
    let room = test_room("SecretLaboratoryRoom", 8, 8);
    let mut spawn = Vec::new();
    let loot = secret_laboratory(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert!(!loot.is_empty());
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("SecretLaboratoryRoom")));
    assert!(spawn.is_empty());
}
