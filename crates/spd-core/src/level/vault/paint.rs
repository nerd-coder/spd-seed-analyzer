//! RegularPainter room shuffle, placeDoors, and vault `paint()` dispatch.

use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::level::painter::{place_doors_for_room, DoorMap, DoorType};
use crate::level::terrain::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

use crate::items::model::{GeneratedItem, ItemCategory};

use super::geometry;
use super::loot::{self, VaultGen};
use super::mobs::VaultMob;
use super::sentries::SentrySpec;
use super::{combat, entrance, final_room, hallway, hazards, tokens, treasure};

pub(super) const HEAP: &str = "heap";
pub(super) const CHEST: &str = "chest";

/// Placement facts the public map needs beyond `TerrainMap`.
#[derive(Debug, Default)]
pub(super) struct PaintFacts {
    /// Mobs the room sets to `WANDERING`; others keep their class state.
    pub wandering: Vec<usize>,
    /// Heaps whose identity is not seed-determined: Imp rewards (route) and
    /// JDK-shuffled consumables (per run).
    pub uncertain: Vec<usize>,
    /// Subset of `uncertain` that varies every run (unseeded shuffle).
    pub per_run: Vec<usize>,
    pub sentries: Vec<SentrySpec>,
}

pub(super) struct PaintCtx<'a> {
    pub map: &'a mut TerrainMap,
    pub rooms: &'a [Room],
    pub doors: &'a mut DoorMap,
    pub dungeon: &'a mut DungeonState,
    pub gen: &'a mut VaultGen,
    pub facts: &'a mut PaintFacts,
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

    /// `level.drop(item, cell).type = heap_type`; a second drop joins the heap.
    pub(super) fn drop(&mut self, cell: usize, heap_type: &'static str, item: GeneratedItem) {
        if loot::is_shuffled_consumable(&item) {
            self.facts.uncertain.push(cell);
            self.facts.per_run.push(cell);
        }
        self.map.record_heap(cell, heap_type, item);
    }

    pub(super) fn drop_new(&mut self, cell: usize, class_name: &str) {
        self.drop(
            cell,
            HEAP,
            GeneratedItem::new(class_name, ItemCategory::Other),
        );
    }

    pub(super) fn place_npc(&mut self, cell: usize, class_name: &'static str) {
        geometry::occupy_mob(self.map, cell, class_name);
    }

    pub(super) fn place_sentry(&mut self, spec: SentrySpec) {
        self.place_npc(spec.cell, "VaultSentry");
        self.facts.sentries.push(spec);
    }

    pub(super) fn place_enemy(&mut self, cell: usize, mob: VaultMob, wandering: bool) {
        geometry::occupy_mob(self.map, cell, mob.class_name());
        if wandering {
            self.facts.wandering.push(cell);
        }
    }

    pub(super) fn create_mob(&mut self) -> VaultMob {
        self.gen.create_mob()
    }

    pub(super) fn return_mob(&mut self, mob: VaultMob) {
        self.gen.return_mob(mob);
    }

    pub(super) fn return_mobs(&mut self, mobs: Vec<VaultMob>) {
        for mob in mobs {
            self.return_mob(mob);
        }
    }

    pub(super) fn next_to_entrance(&self, ri: usize) -> bool {
        self.rooms[ri]
            .connected
            .iter()
            .any(|&other| self.rooms[other].is_entrance())
    }

    /// `createMob` until the spawn is legal. Rejects are not returned yet.
    /// Near an entrance only tier 1 (`Skeleton`, `Dm100`) is kept.
    pub(super) fn create_spawn_mob(
        &mut self,
        near_entrance: bool,
        reject_large: bool,
    ) -> (VaultMob, Vec<VaultMob>) {
        let mut returned = Vec::new();
        let enemy = loop {
            let enemy = self.create_mob();
            if (reject_large && enemy.large()) || (near_entrance && enemy.tier() != 1) {
                returned.push(enemy);
                continue;
            }
            break enemy;
        };
        (enemy, returned)
    }

    pub(super) fn create_equipment(&mut self, tier: usize) -> GeneratedItem {
        self.gen.create_equipment(self.dungeon, tier)
    }

    pub(super) fn create_consumable(&mut self, tier: usize) -> GeneratedItem {
        self.gen.create_consumable(tier)
    }

    pub(super) fn find_prize_any(&mut self) -> Option<GeneratedItem> {
        loot::find_prize_any(&mut self.dungeon.items_to_spawn)
    }

    pub(super) fn find_prize_equipable(&mut self) -> Option<GeneratedItem> {
        loot::find_prize_equipable(&mut self.dungeon.items_to_spawn)
    }

    pub(super) fn find_t2_solve_or_consumable(&mut self, fallback_tier: usize) -> GeneratedItem {
        match self.gen.find_t2_solve(&mut self.dungeon.items_to_spawn) {
            Some(item) => item,
            None => self.create_consumable(fallback_tier),
        }
    }

    pub(super) fn find_t3_solve_or_consumable(&mut self, fallback_tier: usize) -> GeneratedItem {
        match self.gen.find_t3_solve(&mut self.dungeon.items_to_spawn) {
            Some(item) => item,
            None => self.create_consumable(fallback_tier),
        }
    }

    pub(super) fn add_item_to_spawn(&mut self, class_name: &str, category: ItemCategory) {
        self.dungeon
            .items_to_spawn
            .push(GeneratedItem::new(class_name, category));
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

    /// `level.pointToCell(Random.element(rect.getPoints()))`.
    pub(super) fn random_rect_cell(&self, rect: (i32, i32, i32, i32)) -> usize {
        let (left, top, right, bottom) = rect;
        let p = *Random::element(&geometry::rect_points(left, top, right, bottom))
            .expect("treasure rect");
        self.point_cell(p)
    }

    /// Redraws `random_rect_cell` while `level.heaps.get(cell) != null`.
    pub(super) fn random_free_rect_cell(&self, rect: (i32, i32, i32, i32)) -> usize {
        loop {
            let cell = self.random_rect_cell(rect);
            if !self.map.heap_occupied[cell] {
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
    facts: &mut PaintFacts,
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
            facts,
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
