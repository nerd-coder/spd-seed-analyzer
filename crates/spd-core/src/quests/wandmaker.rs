//! Port of `Wandmaker.Quest` (Old Wandmaker, prison floors 7–9).
//!
//! - `spawnRoom` runs at the end of `PrisonLevel.initRooms` (before shuffle)
//! - `spawnWandmaker` runs at the start of `PrisonLevel.createMobs`
//!
//! Quest-room paint is approximate (see `special_loot` for MassGrave loot).
//! Placement uses simplified entrance-room flags (same caveats as Ghost).

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::GeneratedItem;
use crate::level::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::{RoomKind, RoomSpec};

/// 1 = corpse dust, 2 = elemental embers, 3 = rotberry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WandmakerQuestType {
    CorpseDust = 1,
    ElementalEmbers = 2,
    Rotberry = 3,
}

impl WandmakerQuestType {
    pub fn as_str(self) -> &'static str {
        match self {
            WandmakerQuestType::CorpseDust => "Corpse Dust",
            WandmakerQuestType::ElementalEmbers => "Elemental Embers",
            WandmakerQuestType::Rotberry => "Rotberry",
        }
    }

    pub fn room_name(self) -> &'static str {
        match self {
            WandmakerQuestType::CorpseDust => "MassGraveRoom",
            WandmakerQuestType::ElementalEmbers => "RitualSiteRoom",
            WandmakerQuestType::Rotberry => "RotGardenRoom",
        }
    }

    fn from_int(v: i32) -> Self {
        match v {
            1 => WandmakerQuestType::CorpseDust,
            2 => WandmakerQuestType::ElementalEmbers,
            _ => WandmakerQuestType::Rotberry,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WandmakerQuestState {
    pub spawned: bool,
    /// 0 = undecided; 1–3 once type is chosen (when room is added).
    pub quest_type: i32,
    /// Set true on the floor where `spawnRoom` adds the quest room.
    pub quest_room_spawned: bool,
    pub depth: i32,
}

#[derive(Debug, Clone)]
pub struct WandmakerSpawnResult {
    pub quest_type: WandmakerQuestType,
    pub wand1: GeneratedItem,
    pub wand2: GeneratedItem,
    pub summary: String,
}

/// `Wandmaker.Quest.spawnRoom(rooms)` — prison only; call before room shuffle.
///
/// SPD: `!spawned && (type != 0 || (depth > 6 && Random.Int(10 - depth) == 0))`.
/// Depth 6 never rolls; depth 9 always succeeds if not yet spawned.
pub fn try_spawn_room(
    wandmaker: &mut WandmakerQuestState,
    depth: i32,
    specs: &mut Vec<RoomSpec>,
) -> bool {
    wandmaker.quest_room_spawned = false;

    // Only PrisonLevel (depths 6–9) calls this. Depth 6 never passes `depth > 6`.
    if !(6..=9).contains(&depth) {
        return false;
    }
    if wandmaker.spawned {
        return false;
    }

    let type_ready = wandmaker.quest_type != 0;
    let roll_ok = depth > 6 && Random::int_max(10 - depth) == 0;
    if !type_ready && !roll_ok {
        return false;
    }

    if wandmaker.quest_type == 0 {
        wandmaker.quest_type = Random::int_max(3) + 1;
    }
    let qtype = WandmakerQuestType::from_int(wandmaker.quest_type);
    specs.push(quest_room_spec(qtype));
    wandmaker.quest_room_spawned = true;
    true
}

fn quest_room_spec(qtype: WandmakerQuestType) -> RoomSpec {
    match qtype {
        // SpecialRoom subclasses — maxConnections = 1
        WandmakerQuestType::CorpseDust | WandmakerQuestType::Rotberry => RoomSpec {
            name: qtype.room_name().into(),
            kind: RoomKind::Special,
            size_factor: 1,
            max_connections: 1,
        },
        // RitualSiteRoom extends StandardRoom
        WandmakerQuestType::ElementalEmbers => RoomSpec {
            name: qtype.room_name().into(),
            kind: RoomKind::Standard,
            size_factor: 1,
            max_connections: 16,
        },
    }
}

/// `Wandmaker.Quest.spawnWandmaker(level, roomEntrance)` — after paint, before createItems.
pub fn try_spawn_wandmaker(
    dungeon: &mut DungeonState,
    entrance: &Room,
    map: &TerrainMap,
) -> Option<WandmakerSpawnResult> {
    if !dungeon.wandmaker.quest_room_spawned {
        return None;
    }
    dungeon.wandmaker.quest_room_spawned = false;

    // Placement loop on entrance (consumes RNG; terrain flags approximate).
    place_wandmaker(entrance, map);

    dungeon.wandmaker.spawned = true;
    dungeon.wandmaker.depth = dungeon.depth;
    let quest_type = WandmakerQuestType::from_int(dungeon.wandmaker.quest_type);

    let (wand1, wand2) = generate_wands(dungeon);

    let summary = format!(
        "Old Wandmaker ({}) — {} / {}",
        quest_type.as_str(),
        wand1.title(),
        wand2.title()
    );

    Some(WandmakerSpawnResult {
        quest_type,
        wand1,
        wand2,
        summary,
    })
}

fn place_wandmaker(room: &Room, map: &TerrainMap) {
    let entrance_cell = entrance_center_cell(room, map);
    let mut tries = 0;
    let mut dist = 2;
    for _ in 0..2000 {
        if tries > 30 && dist > 0 {
            tries = 0;
            dist -= 1;
        }
        let p = room.random_margin(dist);
        tries += 1;
        let Some(cell) = map.point_to_cell(p.x, p.y) else {
            continue;
        };
        if Some(cell) == entrance_cell {
            continue;
        }
        if map.is_solid(cell) {
            continue;
        }
        // Reject EMPTY_SP and non-passable (approx passable = !solid for painted tiles)
        if !map.passable.get(cell).copied().unwrap_or(false) {
            continue;
        }
        // Door-neighbour check: burn NEIGHBOURS4 style — if any door adjacent, reject
        if has_adjacent_door(map, cell) {
            continue;
        }
        // Valid placement
        return;
    }
}

fn has_adjacent_door(map: &TerrainMap, cell: usize) -> bool {
    let w = map.width as usize;
    let x = cell % w;
    let y = cell / w;
    for (dx, dy) in [(0i32, -1), (1, 0), (0, 1), (-1, 0)] {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= map.width || ny >= map.height {
            continue;
        }
        let ncell = (nx + ny * map.width) as usize;
        // Terrain.DOOR / OPEN_DOOR / LOCKED_DOOR
        if matches!(map.map.get(ncell).copied().unwrap_or(0), 5 | 6 | 10) {
            return true;
        }
    }
    false
}

fn generate_wands(dungeon: &mut DungeonState) -> (GeneratedItem, GeneratedItem) {
    let mut wand1 = dungeon
        .generator
        .random_category(Category::Wand, dungeon.depth);
    wand1.cursed = false;
    upgrade_wand(&mut wand1);

    let mut wand2 = dungeon
        .generator
        .random_category(Category::Wand, dungeon.depth);
    let mut to_undo: Vec<String> = Vec::new();
    while wand2.class_name == wand1.class_name {
        to_undo.push(wand2.class_name.clone());
        wand2 = dungeon
            .generator
            .random_category(Category::Wand, dungeon.depth);
    }
    for class_name in &to_undo {
        dungeon.generator.undo_drop(class_name);
    }
    wand2.cursed = false;
    upgrade_wand(&mut wand2);

    wand1.source = Some("Wandmaker.Quest".into());
    wand2.source = Some("Wandmaker.Quest".into());

    (wand1, wand2)
}

/// `Wand.upgrade()` after clearing curse: level++, `Random.Int(3)==0` curse clear.
fn upgrade_wand(wand: &mut GeneratedItem) {
    wand.level += 1;
    if Random::int_max(3) == 0 {
        wand.cursed = false;
    }
}

fn entrance_center_cell(room: &Room, map: &TerrainMap) -> Option<usize> {
    let cx = (room.left + room.right) / 2;
    let cy = (room.top + room.bottom) / 2;
    map.point_to_cell(cx, cy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::{dungeon_from_run, init_run};

    #[test]
    fn wand_rewards_deterministic() {
        Random::reset_generators();
        let run = init_run(42);
        Random::push_generator_seeded(12345);
        let (w1a, w1b) = {
            let mut d = dungeon_from_run(run.clone());
            d.depth = 9;
            generate_wands(&mut d)
        };
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(12345);
        let (w2a, w2b) = {
            let mut d = dungeon_from_run(run);
            d.depth = 9;
            generate_wands(&mut d)
        };
        Random::pop_generator();

        assert_eq!(w1a.class_name, w2a.class_name);
        assert_eq!(w1a.level, w2a.level);
        assert_eq!(w1b.class_name, w2b.class_name);
        assert_ne!(w1a.class_name, w1b.class_name);
    }

    #[test]
    fn depth9_always_spawns_room_when_not_spawned() {
        Random::reset_generators();
        let mut wm = WandmakerQuestState::default();
        let mut specs = Vec::new();
        // Int(1) is always 0
        assert!(try_spawn_room(&mut wm, 9, &mut specs));
        assert_eq!(specs.len(), 1);
        assert!(wm.quest_room_spawned);
        assert!((1..=3).contains(&wm.quest_type));
    }

    #[test]
    fn depth6_never_spawns_room() {
        Random::reset_generators();
        let mut wm = WandmakerQuestState::default();
        let mut specs = Vec::new();
        // depth > 6 is false — no roll
        assert!(!try_spawn_room(&mut wm, 6, &mut specs));
        assert!(specs.is_empty());
    }
}
