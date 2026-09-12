//! SecretLaboratory, SecretLibrary, and CrystalPath Exotic Crystals conversion.

use super::super::crystal_path;
use super::super::secret_rooms::{secret_laboratory, secret_library};
use super::test_room;
use crate::geom::Point;
use crate::items::exotic::{exotic_to_regular, regular_to_exotic};
use crate::level::painter::DoorMap;
use crate::level::terrain::{paint_minimal, EMPTY_SP};
use crate::level::trinkets::{self, set_held};
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};
use crate::trinkets::{MapProfile, TrinketEvent, TrinketEventAction, TrinketKind};

struct ClearHeldOnDrop;

impl Drop for ClearHeldOnDrop {
    fn drop(&mut self) {
        set_held(None);
    }
}

fn hold_exotic_crystals(level: Option<u8>) -> ClearHeldOnDrop {
    match level {
        None => {
            trinkets::reset(1);
            set_held(None);
        }
        Some(level) => {
            let profile = MapProfile {
                trinket_events: vec![TrinketEvent {
                    before_depth: 1,
                    action: TrinketEventAction::Acquired {
                        trinket: TrinketKind::ExoticCrystals,
                        min_upgrades: Some(level),
                    },
                }],
                ..MapProfile::default()
            };
            trinkets::reset(1);
            set_held(profile.held_at(1));
        }
    }
    ClearHeldOnDrop
}

fn assert_same_or_exotic(regular: &str, actual: &str) {
    if actual == regular {
        assert!(
            regular_to_exotic(regular).is_some(),
            "expected a regular consumable, got {regular}"
        );
        return;
    }
    assert_eq!(exotic_to_regular(actual), Some(regular));
}

fn secret_lab_classes(seed: i64) -> Vec<String> {
    Random::reset_generators();
    let _run = init_run(40);
    Random::push_generator_seeded(seed);
    let room = test_room("SecretLaboratoryRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("secret lab map");
    let loot = secret_laboratory(&room, &mut map);
    Random::pop_generator();
    loot.into_iter().map(|drop| drop.item.class_name).collect()
}

fn secret_library_classes(seed: i64) -> Vec<String> {
    Random::reset_generators();
    Random::push_generator_seeded(seed);
    let room = test_room("SecretLibraryRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("library map");
    map.map.fill(EMPTY_SP);
    let loot = secret_library(&room, &mut map);
    Random::pop_generator();
    loot.into_iter().map(|drop| drop.item.class_name).collect()
}

fn crystal_path_classes(seed: i64) -> Vec<String> {
    Random::reset_generators();
    let run = init_run(41);
    Random::push_generator_seeded(seed);
    let mut dungeon = dungeon_from_run(run);
    dungeon.depth = 1;
    let mut room = test_room("CrystalPathRoom", 8, 8);
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
    doors.insert_test_point(0, 1, Point::new(0, 4));
    let mut spawn = Vec::new();
    let loot = crystal_path::paint(&mut dungeon, &rooms, 0, &mut map, &doors, &mut spawn);
    Random::pop_generator();
    loot.into_iter().map(|drop| drop.item.class_name).collect()
}

#[test]
fn secret_laboratory_stays_regular_without_trinket() {
    let _guard = hold_exotic_crystals(None);
    let classes = secret_lab_classes(110);
    assert!((2..=3).contains(&classes.len()));
    assert!(classes
        .iter()
        .all(|class_name| regular_to_exotic(class_name).is_some()));
}

#[test]
fn secret_laboratory_converts_when_exotic_crystals_are_held() {
    let mut swapped = 0;
    for seed in 110..140 {
        let _none = hold_exotic_crystals(None);
        let regular = secret_lab_classes(seed);
        drop(_none);
        let _held = hold_exotic_crystals(Some(3));
        let converted = secret_lab_classes(seed);
        assert_eq!(regular.len(), converted.len());
        for (base, actual) in regular.iter().zip(&converted) {
            assert_same_or_exotic(base, actual);
            if actual != base {
                swapped += 1;
            }
        }
    }
    assert!(swapped > 0);
}

#[test]
fn secret_library_converts_when_exotic_crystals_are_held() {
    let mut swapped = 0;
    for seed in 0x051E_C1A8..0x051E_C1A8 + 30 {
        let _none = hold_exotic_crystals(None);
        let regular = secret_library_classes(seed);
        drop(_none);
        let _held = hold_exotic_crystals(Some(0));
        let converted = secret_library_classes(seed);
        assert_eq!(regular.len(), converted.len());
        for (base, actual) in regular.iter().zip(&converted) {
            assert_same_or_exotic(base, actual);
            if actual != base {
                swapped += 1;
            }
        }
    }
    assert!(swapped > 0);
}

#[test]
fn crystal_path_converts_when_exotic_crystals_are_held() {
    let mut swapped = 0;
    for seed in 111..141 {
        let _none = hold_exotic_crystals(None);
        let regular = crystal_path_classes(seed);
        drop(_none);
        let _held = hold_exotic_crystals(Some(3));
        let converted = crystal_path_classes(seed);
        assert_eq!(regular.len(), converted.len());
        for (base, actual) in regular.iter().zip(&converted) {
            assert_same_or_exotic(base, actual);
            if actual != base {
                swapped += 1;
            }
        }
    }
    assert!(swapped > 0);
}
