use super::gardens::{garden_prizes, magic_well, secret_garden_prizes};
use super::pit_secrets::{
    pit_prizes, secret_chest_chasm, secret_maze_prize, secret_summoning_prize,
};
use super::trap_rooms::{
    magical_fire_prizes, sacrifice_prize, secret_honeypot, sentry_prize, toxic_gas_prizes,
    traps_prize,
};
use crate::geom::Point;
use crate::items::model::ItemCategory;
use crate::level::painter::DoorMap;
use crate::level::terrain::paint_minimal;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};

mod depth_one_parity;
mod forced_items;
mod placement_masks;
mod quest_npcs;
mod storage;

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
    Random::push_generator_seeded(1);
    let mut d = dungeon_from_run(run);
    d.depth = 11;
    let mut room = test_room("TrapsRoom", 7, 7);
    room.connected.push(1);
    let mut neighbour = Room::new(1, "TunnelRoom", RoomKind::Connection, 1, 16, 3, 10, 3, 10);
    neighbour.left = -4;
    neighbour.top = 2;
    neighbour.right = 0;
    neighbour.bottom = 6;
    neighbour.connected.push(0);
    let rooms = vec![room, neighbour];
    let mut map = paint_minimal(&rooms).expect("test map");
    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, Point::new(0, 3));
    let mut forced = crate::items::model::GeneratedItem::new("Pasty", ItemCategory::Food);
    forced.source = Some("forced".into());
    let mut spawn = vec![forced];
    let loot = traps_prize(&mut d, &rooms, 0, &mut map, &doors, &mut spawn);
    let tail = Random::peek_ints(4);
    Random::pop_generator();

    assert_eq!(loot.item.source.as_deref(), Some("TrapsRoom:forced"));
    assert_eq!(loot.item.class_name, "Pasty");
    assert_eq!(loot.heap_type, "chest");
    assert_eq!(map.heap_occupied.iter().filter(|&&cell| cell).count(), 1);
    assert_eq!(tail, [375063918, -1577561530, 1030305667, 2049706016]);
    assert_eq!(spawn.len(), 1, "queued Pasty was consumed by the room");
    assert_eq!(spawn[0].class_name, "PotionOfLevitation");
}

#[test]
fn magical_fire_drops_and_frost() {
    Random::reset_generators();
    let run = init_run(3);
    Random::push_generator_seeded(9);
    let mut d = dungeon_from_run(run);
    d.depth = 4;
    let mut room = test_room("MagicalFireRoom", 9, 9);
    room.connected.push(1);
    let mut neighbour = Room::new(1, "TunnelRoom", RoomKind::Connection, 1, 16, 3, 10, 3, 10);
    neighbour.left = -4;
    neighbour.top = 2;
    neighbour.right = 0;
    neighbour.bottom = 7;
    neighbour.connected.push(0);
    let rooms = vec![room, neighbour];
    let mut map = paint_minimal(&rooms).expect("test map");
    let mut doors = DoorMap::new();
    doors.insert_test_point(0, 1, Point::new(0, 4));
    let mut spawn = Vec::new();
    let loot = magical_fire_prizes(&mut d, &rooms, 0, &mut map, &doors, &mut spawn);
    let tail = Random::peek_ints(4);
    Random::pop_generator();

    assert!((3..=4).contains(&loot.len()));
    assert!(loot
        .iter()
        .all(|p| p.item.source.as_deref() == Some("MagicalFireRoom")));
    assert_eq!(
        spawn.last().map(|i| i.class_name.as_str()),
        Some("PotionOfFrost")
    );
    assert_eq!(
        map.heap_occupied
            .iter()
            .filter(|&&occupied| occupied)
            .count(),
        loot.len()
    );
    assert!(map.grass_allowed.iter().any(|&allowed| !allowed));
    assert!(map.character_allowed.iter().any(|&allowed| !allowed));
    assert_eq!(tail, [-1240710046, 1594262110, -1344810206, 1548219350]);
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
    let room = test_room("MagicWellRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("well map");
    let well = magic_well(&room, &mut map, &mut spawn);
    Random::pop_generator();
    assert!(well.is_empty(), "well water is a blob, not a heap");
    assert!(map.map.contains(&crate::level::terrain::WELL));

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
