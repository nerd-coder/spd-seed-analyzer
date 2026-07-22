use super::gardens::{garden_prizes, magic_well, secret_garden_prizes};
use super::pit_secrets::{
    pit_prizes, secret_chest_chasm, secret_maze_prize, secret_summoning_prize,
};
use super::secret_rooms::{secret_laboratory, secret_runestone};
use super::special_rooms::{
    armory_prizes, crypt_prize, laboratory_prizes, library_prizes, pool_prize, runestone_prizes,
    statue_weapon, treasury_prizes,
};
use super::trap_rooms::{
    magical_fire_prizes, sacrifice_prize, secret_honeypot, sentry_prize, toxic_gas_prizes,
    traps_prize,
};
use crate::items::model::ItemCategory;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};

fn test_room(name: &str, w: i32, h: i32) -> Room {
    let mut r = Room::new(0, name, RoomKind::Special, 1, 3, w, w + 2, h, h + 2);
    r.left = 0;
    r.top = 0;
    r.right = w;
    r.bottom = h;
    r
}

#[test]
fn sacrifice_prize_is_cursed_weapon() {
    Random::reset_generators();
    let run = init_run(42);
    Random::push_generator_seeded(12345);
    let mut d = dungeon_from_run(run);
    d.depth = 6;
    let room = test_room("SacrificeRoom", 7, 7);
    let loot = sacrifice_prize(&mut d, &[room], 0);
    Random::pop_generator();

    assert_eq!(loot.item.source.as_deref(), Some("SacrificeRoom"));
    assert_eq!(loot.heap_type, "sacrificial");
    assert!(loot.item.cursed);
    assert_eq!(loot.item.category, ItemCategory::Weapon);
    // uncursed weapons get a free upgrade before the curse is forced
    assert!(loot.item.level >= 1 || loot.item.enchantment.is_some());
}

#[test]
fn sentry_prize_deterministic() {
    Random::reset_generators();
    let run = init_run(7);
    Random::push_generator_seeded(777);
    let mut d = dungeon_from_run(run.clone());
    d.depth = 8;
    let mut spawn = Vec::new();
    let a = sentry_prize(&mut d, &mut spawn);
    Random::pop_generator();

    Random::reset_generators();
    Random::push_generator_seeded(777);
    let mut d2 = dungeon_from_run(run);
    d2.depth = 8;
    let mut spawn2 = Vec::new();
    let b = sentry_prize(&mut d2, &mut spawn2);
    Random::pop_generator();

    assert_eq!(a.item.class_name, b.item.class_name);
    assert_eq!(a.item.level, b.item.level);
    assert!(!a.item.cursed);
    assert_eq!(a.item.source.as_deref(), Some("SentryRoom"));
    assert_eq!(spawn.len(), 1);
    assert_eq!(spawn[0].class_name, "PotionOfHaste");
}

#[test]
fn traps_prize_adds_levitation() {
    Random::reset_generators();
    let run = init_run(99);
    Random::push_generator_seeded(55);
    let mut d = dungeon_from_run(run);
    d.depth = 11;
    let mut spawn = Vec::new();
    let loot = traps_prize(&mut d, &mut spawn);
    Random::pop_generator();

    assert_eq!(loot.item.source.as_deref(), Some("TrapsRoom"));
    assert_eq!(loot.heap_type, "chest");
    assert!(!loot.item.cursed);
    assert_eq!(spawn[0].class_name, "PotionOfLevitation");
}

#[test]
fn magical_fire_drops_and_frost() {
    Random::reset_generators();
    let run = init_run(3);
    Random::push_generator_seeded(9);
    let mut d = dungeon_from_run(run);
    d.depth = 4;
    let room = test_room("MagicalFireRoom", 9, 9);
    let mut spawn = Vec::new();
    let loot = magical_fire_prizes(&mut d, &room, &mut spawn);
    Random::pop_generator();

    assert!((3..=4).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("MagicalFireRoom")));
    assert_eq!(
        spawn.last().map(|i| i.class_name.as_str()),
        Some("PotionOfFrost")
    );
}

#[test]
fn secret_honeypot_has_pot_honey_bomb() {
    Random::reset_generators();
    Random::push_generator_seeded(1);
    let room = test_room("SecretHoneypotRoom", 7, 7);
    let loot = secret_honeypot(&room);
    Random::pop_generator();

    assert_eq!(loot.len(), 3);
    assert_eq!(loot[0].item.class_name, "ShatteredPot");
    assert_eq!(loot[1].item.class_name, "Honeypot");
    assert!(
        loot[2].item.class_name == "Bomb" || loot[2].item.class_name == "DoubleBomb",
        "got {}",
        loot[2].item.class_name
    );
}

#[test]
fn toxic_gas_burns_layout_and_adds_purity() {
    Random::reset_generators();
    let run = init_run(12);
    Random::push_generator_seeded(88);
    let mut d = dungeon_from_run(run);
    d.depth = 7;
    let room = test_room("ToxicGasRoom", 9, 9);
    let mut spawn = Vec::new();
    let loot = toxic_gas_prizes(&mut d, &room, &mut spawn);
    Random::pop_generator();

    // skeleton gold + 2 chests
    assert_eq!(loot.len(), 3);
    assert_eq!(loot[0].heap_type, "skeleton");
    assert_eq!(loot[1].heap_type, "chest");
    assert_eq!(loot[2].heap_type, "chest");
    assert_eq!(
        spawn.last().map(|i| i.class_name.as_str()),
        Some("PotionOfPurity")
    );
}

#[test]
fn secret_chest_chasm_four_locked_plus_levitation() {
    Random::reset_generators();
    let run = init_run(21);
    Random::push_generator_seeded(404);
    let mut d = dungeon_from_run(run);
    d.depth = 9;
    let mut spawn = Vec::new();
    let loot = secret_chest_chasm(&mut d, &mut spawn);
    Random::pop_generator();

    let chests: Vec<_> = loot
        .iter()
        .filter(|p| p.heap_type == "locked_chest")
        .collect();
    assert_eq!(chests.len(), 4);
    assert!(chests
        .iter()
        .all(|p| p.item.source.as_deref() == Some("SecretChestChasmRoom")));
    assert_eq!(
        spawn.last().map(|i| i.class_name.as_str()),
        Some("PotionOfLevitation")
    );
}

#[test]
fn secret_summoning_skeleton_random() {
    Random::reset_generators();
    let run = init_run(5);
    Random::push_generator_seeded(11);
    let mut d = dungeon_from_run(run);
    d.depth = 6;
    let loot = secret_summoning_prize(&mut d);
    Random::pop_generator();

    assert_eq!(loot.heap_type, "skeleton");
    assert_eq!(loot.item.source.as_deref(), Some("SecretSummoningRoom"));
    assert!(!loot.item.class_name.is_empty());
}

#[test]
fn secret_maze_uncursed_equip() {
    Random::reset_generators();
    let run = init_run(8);
    Random::push_generator_seeded(33);
    let mut d = dungeon_from_run(run);
    d.depth = 10;
    let loot = secret_maze_prize(&mut d);
    Random::pop_generator();

    assert_eq!(loot.heap_type, "chest");
    assert!(!loot.item.cursed);
    assert_eq!(loot.item.source.as_deref(), Some("SecretMazeRoom"));
    assert!(matches!(
        loot.item.category,
        ItemCategory::Weapon | ItemCategory::Armor
    ));
}

#[test]
fn pit_room_skeleton_and_crystal_key() {
    Random::reset_generators();
    let run = init_run(15);
    Random::push_generator_seeded(66);
    let mut d = dungeon_from_run(run);
    d.depth = 12;
    let mut spawn = Vec::new();
    let loot = pit_prizes(&mut d, &mut spawn);
    Random::pop_generator();

    assert!((2..=3).contains(&loot.len())); // main + 1–2 side prizes
    assert!(loot.iter().all(|p| p.heap_type == "skeleton"));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("PitRoom")));
    assert_eq!(
        spawn.last().map(|i| i.class_name.as_str()),
        Some("CrystalKey")
    );
}

#[test]
fn garden_and_well_prizes() {
    Random::reset_generators();
    Random::push_generator_seeded(2);
    let room = test_room("GardenRoom", 7, 7);
    let mut spawn = Vec::new();
    let garden = garden_prizes(&room, &mut spawn);
    Random::pop_generator();
    assert_eq!(spawn[0].class_name, "IronKey");
    // bushes roll may yield 0–2 plants
    assert!(garden.len() <= 2);
    assert!(garden
        .iter()
        .all(|p| p.heap_type == "plant" && p.item.source.as_deref() == Some("GardenRoom")));

    Random::reset_generators();
    Random::push_generator_seeded(3);
    let well = magic_well(&mut spawn);
    Random::pop_generator();
    assert_eq!(well.len(), 1);
    assert!(
        well[0].item.class_name == "WaterOfAwareness" || well[0].item.class_name == "WaterOfHealth"
    );
    assert_eq!(well[0].heap_type, "well");

    Random::reset_generators();
    Random::push_generator_seeded(4);
    let room = test_room("SecretGardenRoom", 8, 8);
    let secret = secret_garden_prizes(&room);
    Random::pop_generator();
    assert_eq!(secret.len(), 4);
    assert_eq!(secret[0].item.class_name, "StarflowerSeed");
    assert!(secret
        .iter()
        .all(|p| p.item.source.as_deref() == Some("SecretGardenRoom")));
}

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
    let loot = treasury_prizes(&mut d, &room, &mut spawn);
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

    // placed prize itself is unchanged by the push
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

    // placed prize itself is unchanged by the push
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
    let mut spawn = Vec::new();
    let loot = armory_prizes(&mut d, &room, &mut spawn);
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
