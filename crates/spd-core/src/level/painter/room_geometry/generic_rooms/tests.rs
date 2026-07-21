use super::*;
use crate::level::painter::DoorMap;
use crate::level::terrain::{
    self, BOOKSHELF, CHASM, EMPTY_SP, GRASS, HIGH_GRASS, PEDESTAL, SECRET_TRAP, WALL, WATER,
};
use crate::random::Random;
use crate::rooms::types::RoomKind;
use crate::run::{dungeon_from_run, init_run};

fn room(name: &str, width: i32, height: i32, size_factor: i32) -> Room {
    let mut room = Room::new(
        0,
        name,
        RoomKind::Standard,
        size_factor,
        16,
        width,
        width,
        height,
        height,
    );
    room.left = 1;
    room.top = 1;
    room.right = width;
    room.bottom = height;
    room
}

fn paint_room(
    name: &str,
    width: i32,
    height: i32,
    size_factor: i32,
) -> (TerrainMap, StandardPaintResult) {
    Random::reset_generators();
    let run = init_run(19);
    let mut dungeon = dungeon_from_run(run);
    dungeon.depth = 8;
    let room = room(name, width, height, size_factor);
    let mut map = terrain::paint_minimal(std::slice::from_ref(&room)).expect("map");
    Random::push_generator_seeded(0x5EED);
    let result = paint(
        &mut map,
        &room,
        0,
        &DoorMap::new(),
        &mut dungeon.generator,
        dungeon.depth,
    )
    .expect("supported room");
    Random::pop_generator();
    (map, result)
}

#[test]
fn plants_paints_grass_and_blocks_plant_cells_from_item_drops() {
    let (map, _) = paint_room("PlantsRoom", 9, 8, 1);
    assert!(map.map.contains(&HIGH_GRASS));
    assert_eq!(
        map.item_allowed.iter().filter(|&&allowed| !allowed).count(),
        2
    );
}

#[test]
fn aquarium_paints_water_and_blocks_it_from_item_drops() {
    let (map, _) = paint_room("AquariumRoom", 10, 10, 1);
    let water = map.map.iter().filter(|&&terrain| terrain == WATER).count();
    let blocked_water = map
        .map
        .iter()
        .zip(&map.item_allowed)
        .filter(|(terrain, allowed)| **terrain == WATER && !**allowed)
        .count();
    assert_eq!(water, 16);
    assert_eq!(blocked_water, water);
}

#[test]
fn grassy_grave_paints_a_grass_interior() {
    let (map, _) = paint_room("GrassyGraveRoom", 8, 10, 1);
    let room_floor = map.map.iter().filter(|&&terrain| terrain == GRASS).count();
    assert_eq!(room_floor, 48);
    assert!(map.map.iter().filter(|&&terrain| terrain == WALL).count() > 32);
}

#[test]
fn platform_and_fissure_paint_both_floor_and_chasm() {
    let (platform, _) = paint_room("PlatformRoom", 14, 14, 3);
    assert!(platform.map.contains(&CHASM));
    assert!(platform.map.contains(&EMPTY_SP));

    let (fissure, _) = paint_room("FissureRoom", 10, 10, 1);
    assert!(fissure.map.contains(&CHASM));
    assert!(fissure.map.contains(&terrain::EMPTY));
}

#[test]
fn striped_and_study_paint_their_structural_tiles() {
    let (striped, _) = paint_room("StripedRoom", 9, 9, 1);
    assert!(striped.map.contains(&HIGH_GRASS));
    assert!(striped.map.contains(&EMPTY_SP));

    let (study, result) = paint_room("StudyRoom", 12, 12, 2);
    assert!(study.map.contains(&BOOKSHELF));
    assert_eq!(
        study
            .map
            .iter()
            .filter(|&&terrain| terrain == PEDESTAL)
            .count(),
        1
    );
    let center = result.center_loot.expect("study center");
    let cell = study
        .point_to_cell(center.x, center.y)
        .expect("center cell");
    assert!(!study.item_allowed[cell]);
}

#[test]
fn suspicious_chest_defers_center_until_after_prize_rng() {
    let (map, result) = paint_room("SuspiciousChestRoom", 7, 8, 1);
    assert!(result.center_loot.is_none());
    assert!(!map.map.contains(&PEDESTAL));
}

#[test]
fn minefield_places_expected_hidden_explosive_mines() {
    let (map, _) = paint_room("MinefieldRoom", 9, 9, 1);
    let mines: Vec<_> = map
        .map
        .iter()
        .enumerate()
        .filter(|(_, terrain)| **terrain == SECRET_TRAP)
        .map(|(cell, _)| cell)
        .collect();
    assert_eq!(mines.len(), 6);
    assert!(mines.iter().all(|&cell| {
        map.trap_names[cell] == Some("ExplosiveTrap") && map.trap_destroys_items[cell]
    }));
}
