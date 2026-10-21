//! `VaultLevel.createItems`: leftover `itemsToSpawn`, Darkness torches, and the
//! extra Potion of Healing beside the entrance.

use crate::dungeon::DungeonState;
use crate::geom::Point;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::terrain::{TerrainMap, EMPTY_SP, FURROWED_GRASS, GRASS, HIGH_GRASS};
use crate::random::Random;
use crate::rooms::room::Room;

use super::geometry::center;
use super::loot::is_shuffled_consumable;
use super::paint::{PaintFacts, HEAP};
use super::rooms::is_treasure;

const DARKNESS: i32 = 32;

struct Drops<'a> {
    map: &'a mut TerrainMap,
    rooms: &'a [Room],
    /// Java `level.rooms`, reshuffled in place by every `randomRoom`.
    order: Vec<usize>,
    entrance: usize,
    exit_cell: Option<usize>,
}

pub(super) fn create_items(
    map: &mut TerrainMap,
    rooms: &[Room],
    paint_order: &[usize],
    dungeon: &mut DungeonState,
    facts: &mut PaintFacts,
) {
    let entrance = rooms
        .iter()
        .position(|room| room.name == "VaultEntranceRoom")
        .expect("vault entrance room");
    // `exit()` falls back to the branch-entrance transition.
    let exit_cell = map.branch_entrances.first().copied();
    let mut drops = Drops {
        map,
        rooms,
        order: paint_order.to_vec(),
        entrance,
        exit_cell,
    };

    for item in std::mem::take(&mut dungeon.items_to_spawn) {
        let cell = drops.random_drop_cell();
        drops.drop(cell, item, facts);
    }

    if dungeon.challenges & DARKNESS != 0 {
        let seed = Random::long();
        Random::push_generator_seeded(seed);
        for _ in 0..2 {
            let cell = drops.random_drop_cell();
            drops.drop(
                cell,
                GeneratedItem::new("Torch", ItemCategory::Other),
                facts,
            );
        }
        Random::pop_generator();
    }

    let mut tries = 1000;
    let mut cell;
    loop {
        cell = drops.random_drop_cell();
        tries -= 1;
        if drops.beside_entrance(cell) || tries <= 0 {
            break;
        }
    }
    drops.drop(
        cell,
        GeneratedItem::new("PotionOfHealing", ItemCategory::Potion),
        facts,
    );
}

impl Drops<'_> {
    /// `RegularLevel.randomDropCell(StandardRoom.class)`.
    fn random_drop_cell(&mut self) -> Option<usize> {
        for _ in 0..100 {
            Random::shuffle_list(&mut self.order);
            // Every vault room but the final (Special) room is a StandardRoom.
            let ri = *self
                .order
                .iter()
                .find(|&&ri| self.rooms[ri].name != "VaultFinalRoom")?;
            if ri == self.entrance {
                continue;
            }
            let room = &self.rooms[ri];
            let p = room.random();
            let Some(cell) = self.map.point_to_cell(p.x, p.y) else {
                continue;
            };
            if self.map.passable[cell]
                && !self.map.is_solid(cell)
                && Some(cell) != self.exit_cell
                && !self.map.heap_occupied[cell]
                && can_place_item(room, p, self.map)
                && !self.map.mob_occupied[cell]
                && !self.map.trap_destroys_items[cell]
            {
                return Some(cell);
            }
        }
        None
    }

    /// `room(cell).connected.containsKey(roomEntrance)`.
    fn beside_entrance(&self, cell: Option<usize>) -> bool {
        let Some(cell) = cell else {
            return false;
        };
        let w = self.map.width as usize;
        let (x, y) = (
            (cell % w) as i32 + self.map.origin_x,
            (cell / w) as i32 + self.map.origin_y,
        );
        self.order
            .iter()
            .map(|&ri| &self.rooms[ri])
            .find(|room| x > room.left && y > room.top && x < room.right && y < room.bottom)
            .is_some_and(|room| room.connected.contains(&self.entrance))
    }

    fn drop(&mut self, cell: Option<usize>, item: GeneratedItem, facts: &mut PaintFacts) {
        let Some(cell) = cell else {
            return;
        };
        if is_shuffled_consumable(&item) {
            facts.uncertain.push(cell);
        }
        self.map.record_heap(cell, HEAP, item);
        if matches!(self.map.map[cell], HIGH_GRASS | FURROWED_GRASS) {
            self.map.map[cell] = GRASS;
        }
    }
}

/// Vault `Room.canPlaceItem` overrides; `Room.random()` is always `inside`.
fn can_place_item(room: &Room, p: Point, map: &TerrainMap) -> bool {
    match room.name.as_str() {
        name if is_treasure(name) => false,
        "VaultEnemyCenterRoom" | "VaultSimpleEnemyTreasureRoom" => {
            let c = center(room);
            (c.x - p.x).abs() > 2 && (c.y - p.y).abs() > 2
        }
        // Java precedence: `inside && x == left+1 || y == top+1 || ...`.
        "VaultAlternatingFireRoom" => {
            p.x == room.left + 1
                || p.y == room.top + 1
                || p.x == room.right - 1
                || p.y == room.bottom - 1
        }
        "VaultTokensRoom" => map
            .point_to_cell(p.x, p.y)
            .is_some_and(|cell| map.map[cell] != EMPTY_SP),
        _ => true,
    }
}
