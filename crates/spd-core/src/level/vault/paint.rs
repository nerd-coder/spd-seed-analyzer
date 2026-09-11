//! RegularPainter room shuffle, placeDoors, and vault `paint()` dispatch.

use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::level::painter::{place_doors_for_room, DoorMap, DoorType};
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

use super::geometry;
use super::loot::{self, VaultGen, VaultMob};
use super::{combat, entrance, final_room, hallway, hazards, tokens, treasure};

pub(super) struct PaintCtx<'a> {
    pub map: &'a mut TerrainMap,
    pub rooms: &'a [Room],
    pub doors: &'a mut DoorMap,
    pub dungeon: &'a mut DungeonState,
    pub gen: &'a mut VaultGen,
}

impl PaintCtx<'_> {
    pub(super) fn room(&self, ri: usize) -> &Room {
        &self.rooms[ri]
    }

    pub(super) fn width(&self) -> i32 {
        self.map.width
    }

    pub(super) fn cell(&self, x: i32, y: i32) -> usize {
        geometry::cell(self.map, x, y)
    }

    pub(super) fn point_cell(&self, p: Point) -> usize {
        self.cell(p.x, p.y)
    }

    pub(super) fn door_points(&self, ri: usize) -> Vec<Point> {
        geometry::door_points(self.rooms, ri, self.doors)
    }

    pub(super) fn set_regular_doors(&mut self, ri: usize) {
        let connected = self.rooms[ri].connected.clone();
        for other in connected {
            if let Some(door) = self.doors.get_mut(ri, other) {
                door.set(DoorType::Regular);
            }
        }
    }

    pub(super) fn occupy_heap(&mut self, cell: usize) {
        geometry::occupy_heap(self.map, cell);
    }

    pub(super) fn occupy_mob(&mut self, cell: usize) {
        geometry::occupy_mob(self.map, cell);
    }

    pub(super) fn create_mob(&mut self) -> VaultMob {
        self.gen.create_mob()
    }

    pub(super) fn return_mob(&mut self, mob: VaultMob) {
        self.gen.return_mob(mob);
    }

    pub(super) fn create_equipment(&mut self, tier: usize) {
        let _ = self.gen.create_equipment(self.dungeon, tier);
    }

    pub(super) fn create_consumable(&mut self, tier: usize) {
        let _ = self.gen.create_consumable(tier);
    }

    pub(super) fn find_prize_any(&mut self) {
        let _ = loot::find_prize_any(&mut self.dungeon.items_to_spawn);
    }

    pub(super) fn find_t2_solve_or_consumable(&mut self, fallback_tier: usize) {
        if self
            .gen
            .find_t2_solve(&mut self.dungeon.items_to_spawn)
            .is_none()
        {
            self.create_consumable(fallback_tier);
        }
    }

    pub(super) fn find_t3_solve_or_consumable(&mut self, fallback_tier: usize) {
        if self
            .gen
            .find_t3_solve(&mut self.dungeon.items_to_spawn)
            .is_none()
        {
            self.create_consumable(fallback_tier);
        }
    }

    pub(super) fn add_item_to_spawn(
        &mut self,
        class_name: &str,
        category: crate::items::model::ItemCategory,
    ) {
        self.dungeon
            .items_to_spawn
            .push(crate::items::model::GeneratedItem::new(
                class_name, category,
            ));
    }

    pub(super) fn create_t2_mob(&mut self) -> VaultMob {
        self.gen.create_t2_mob()
    }

    pub(super) fn entrance_door(&self, ri: usize) -> Point {
        self.door_points(ri)
            .into_iter()
            .next()
            .expect("vault treasure rooms have one door")
    }

    pub(super) fn occupy_random_rect(
        &mut self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        skip: &[usize],
    ) -> usize {
        loop {
            let p = *Random::element(&geometry::rect_points(left, top, right, bottom))
                .expect("treasure rect");
            let cell = self.point_cell(p);
            if !skip.contains(&cell) {
                self.occupy_heap(cell);
                return cell;
            }
        }
    }
}

pub(super) fn paint_rooms(
    map: &mut TerrainMap,
    rooms: &[Room],
    doors: &mut DoorMap,
    dungeon: &mut DungeonState,
    gen: &mut VaultGen,
) -> Vec<usize> {
    let mut order: Vec<usize> = (0..rooms.len()).collect();
    Random::shuffle_list(&mut order);
    for &ri in &order {
        place_doors_for_room(rooms, ri, doors);
        let mut ctx = PaintCtx {
            map,
            rooms,
            doors,
            dungeon,
            gen,
        };
        paint_one(&mut ctx, ri);
    }
    order
}

fn paint_one(ctx: &mut PaintCtx<'_>, ri: usize) {
    match ctx.rooms[ri].name.as_str() {
        "VaultEntranceRoom" => entrance::paint(ctx, ri),
        "VaultTokensRoom" => tokens::paint(ctx, ri),
        "VaultFinalRoom" => final_room::paint(ctx, ri),
        "VaultHallwayRoom" => hallway::paint(ctx, ri),
        "VaultRingRoom" => combat::paint_ring(ctx, ri),
        "VaultRingsRoom" => combat::paint_rings(ctx, ri),
        "VaultLongRingsRoom" => combat::paint_long_rings(ctx, ri),
        "VaultEnemyCenterRoom" => combat::paint_enemy_center(ctx, ri),
        "VaultQuadrantsRoom" => combat::paint_quadrants(ctx, ri),
        "VaultSimpleEnemyTreasureRoom" => combat::paint_simple_enemy(ctx, ri),
        "VaultCrossRoom" => hazards::paint_cross(ctx, ri),
        "VaultCircleRoom" => hazards::paint_circle(ctx, ri),
        "VaultLasersRoom" => hazards::paint_lasers(ctx, ri),
        "VaultAlternatingFireRoom" => hazards::paint_alternating_fire(ctx, ri),
        "VaultFlamePathRoom" => treasure::paint_flame_path(ctx, ri),
        "VaultLaserTreasureRoom" => treasure::paint_laser(ctx, ri),
        "VaultCircleScanTreasureRoom" => treasure::paint_circle_scan(ctx, ri),
        "VaultSingleEnemyTreasureRoom" => treasure::paint_single_enemy(ctx, ri),
        "VaultBookcaseTreasureRoom" => treasure::paint_bookcase(ctx, ri),
        "VaultFlamesTreasureRoom" => treasure::paint_flames(ctx, ri),
        "VaultManyScansRoom" => treasure::paint_many_scans(ctx, ri),
        "VaultMultipleEnemyTreasureRoom" => treasure::paint_multiple_enemy(ctx, ri),
        "VaultHardLaserTreasureRoom" => treasure::paint_hard_laser(ctx, ri),
        _ => {}
    }
}
