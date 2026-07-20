//! Dungeon run context (depth, seed, limited drops).

mod limited_drops;

pub use limited_drops::LimitedDrops;

use crate::generator::GeneratorState;
use crate::items::IdentityMaps;
use crate::items::model::GeneratedItem;
use crate::random::Random;
use crate::rooms::RoomRunState;

#[derive(Debug, Clone)]
pub struct DungeonState {
    pub seed: i64,
    pub depth: i32,
    pub branch: i32,
    pub challenges: i32,
    pub identities: IdentityMaps,
    pub rooms: RoomRunState,
    pub generator: GeneratorState,
    pub limited: LimitedDrops,
    /// Items forced onto the next level (`Level.itemsToSpawn`).
    pub items_to_spawn: Vec<GeneratedItem>,
}

impl DungeonState {
    pub fn seed_cur_depth(&self) -> i64 {
        seed_for_depth(self.seed, self.depth, self.branch)
    }

    pub fn shop_on_level(&self) -> bool {
        matches!(self.depth, 6 | 11 | 16)
    }

    pub fn boss_level(&self) -> bool {
        matches!(self.depth, 5 | 10 | 15 | 20 | 25)
    }

    pub fn pos_needed(&mut self) -> bool {
        let pos_left = 2 - (self.limited.strength_potions - (self.depth / 5) * 2);
        if pos_left <= 0 {
            return false;
        }
        let floor_this_set = self.depth % 5;
        let mut target = 2 - floor_this_set / 2;
        if floor_this_set % 2 == 1 && Random::int_max(2) == 0 {
            target -= 1;
        }
        target < pos_left
    }

    pub fn sou_needed(&mut self) -> bool {
        let sou_left = 3 - (self.limited.upgrade_scrolls - (self.depth / 5) * 3);
        if sou_left <= 0 {
            return false;
        }
        let floor_this_set = self.depth % 5;
        Random::int_max(5 - floor_this_set) < sou_left
    }

    pub fn as_needed(&mut self) -> bool {
        let as_left = 1 - (self.limited.arcane_styli - (self.depth / 5));
        if as_left <= 0 {
            return false;
        }
        let floor_this_set = self.depth % 5;
        Random::int_max(5 - floor_this_set) < as_left
    }

    pub fn ench_stone_needed(&mut self) -> bool {
        if self.limited.ench_stone {
            return false;
        }
        let region = 1 + self.depth / 5;
        if region > 1 {
            let mut floors_visited = self.depth - 5;
            if floors_visited > 4 {
                floors_visited -= 1;
            }
            return Random::int_max(9 - floors_visited) == 0;
        }
        false
    }

    pub fn int_stone_needed(&mut self) -> bool {
        self.depth < 5 && !self.limited.int_stone && Random::int_max(4 - self.depth) == 0
    }

    pub fn trinket_cata_needed(&mut self) -> bool {
        self.depth < 5 && !self.limited.trinket_cata && Random::int_max(4 - self.depth) == 0
    }

    pub fn lab_room_needed(&mut self) -> bool {
        let region = 1 + self.depth / 5;
        if region > self.limited.lab_room {
            let floor_this_region = self.depth % 5;
            if floor_this_region >= 4 || (floor_this_region == 3 && Random::int_max(2) == 0) {
                return true;
            }
        }
        false
    }
}

/// `Dungeon.seedForDepth`.
pub fn seed_for_depth(seed: i64, depth: i32, branch: i32) -> i64 {
    let look_ahead = depth + 30 * branch;
    Random::push_generator_seeded(seed);
    for _ in 0..look_ahead {
        Random::long();
    }
    let result = Random::long();
    Random::pop_generator();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn seed_for_depth_deterministic() {
        Random::reset_generators();
        let a = seed_for_depth(42, 1, 0);
        let b = seed_for_depth(42, 1, 0);
        assert_eq!(a, b);
        let c = seed_for_depth(42, 2, 0);
        assert_ne!(a, c);
    }
}
