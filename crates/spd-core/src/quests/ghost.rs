//! Port of `Ghost.Quest.spawn` (Sad Ghost, sewer floors).
//!
//! Called from `SewerLevel.createMobs` before regular mob placement. We only
//! port reward generation + spawn chance; placement uses simplified solid /
//! openSpace flags from the minimal painter (may desync createItems RNG).

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::level::TerrainMap;
use crate::random::Random;
use crate::rooms::room::Room;

const ARMOR_TIER_CHANCES: &[f32] = &[0., 0., 10., 6., 3., 1.];
const WEAPON_TIER_CHANCES: &[f32] = &[0., 0., 10., 6., 3., 1.];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhostType {
    FetidRat = 1,
    GnollTrickster = 2,
    GreatCrab = 3,
}

impl GhostType {
    pub fn as_str(self) -> &'static str {
        match self {
            GhostType::FetidRat => "Fetid Rat",
            GhostType::GnollTrickster => "Gnoll Trickster",
            GhostType::GreatCrab => "Great Crab",
        }
    }

    fn from_depth(depth: i32) -> Self {
        match depth {
            2 => GhostType::FetidRat,
            3 => GhostType::GnollTrickster,
            _ => GhostType::GreatCrab, // depth 4 (and any later sewer)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GhostQuestState {
    pub spawned: bool,
    pub quest_type: Option<GhostType>,
    pub depth: i32,
}

#[derive(Debug, Clone)]
pub struct GhostSpawnResult {
    pub quest_type: GhostType,
    pub weapon: GeneratedItem,
    pub armor: GeneratedItem,
    pub summary: String,
}

/// `Ghost.Quest.spawn(level, roomExit)` — sewer depths only (1–4).
///
/// Placement burns `Room.random()` until a non-solid open cell ≠ exit.
/// Reward generation advances the weapon generator deck.
pub fn try_spawn_ghost(
    dungeon: &mut DungeonState,
    exit_room: &Room,
    map: &TerrainMap,
) -> Option<GhostSpawnResult> {
    // Only SewerLevel calls this (depths 1–4). Depth 1 never spawns.
    if dungeon.depth < 1 || dungeon.depth > 4 {
        return None;
    }
    if dungeon.ghost.spawned {
        return None;
    }
    if dungeon.depth <= 1 {
        return None;
    }
    // Random.Int(5 - depth) == 0
    if Random::int_max(5 - dungeon.depth) != 0 {
        return None;
    }

    // Placement loop (consumes RNG; openSpace is approximate).
    let exit_cell = exit_center_cell(exit_room, map);
    let mut placed = false;
    for _ in 0..2000 {
        let p = exit_room.random();
        let Some(cell) = map.point_to_cell(p.x, p.y) else {
            continue;
        };
        if Some(cell) == exit_cell {
            continue;
        }
        if map.is_solid(cell) {
            continue;
        }
        if !map.is_open_space(cell) {
            continue;
        }
        placed = true;
        break;
    }
    if !placed {
        // Extremely rare with valid exit rooms; still spawn rewards so analysis
        // reports the quest (placement desync already possible from painter).
        let _ = exit_room.random();
    }

    dungeon.ghost.spawned = true;
    let quest_type = GhostType::from_depth(dungeon.depth);
    dungeon.ghost.quest_type = Some(quest_type);
    dungeon.ghost.depth = dungeon.depth;

    let (weapon, armor) = generate_rewards(dungeon);

    let summary = format!(
        "Sad Ghost ({}) — {} / {}",
        quest_type.as_str(),
        weapon.title(),
        armor.title()
    );

    Some(GhostSpawnResult {
        quest_type,
        weapon,
        armor,
        summary,
    })
}

fn generate_rewards(dungeon: &mut DungeonState) -> (GeneratedItem, GeneratedItem) {
    // 50% t2, 30% t3, 15% t4, 5% t5
    let armor_tier = Random::chances(ARMOR_TIER_CHANCES).max(2) as usize;
    let mut armor = match armor_tier {
        2 => GeneratedItem::new("LeatherArmor", ItemCategory::Armor),
        3 => GeneratedItem::new("MailArmor", ItemCategory::Armor),
        4 => GeneratedItem::new("ScaleArmor", ItemCategory::Armor),
        _ => GeneratedItem::new("PlateArmor", ItemCategory::Armor),
    };

    let wep_tier = Random::chances(WEAPON_TIER_CHANCES).max(2) as usize;
    // Generator.random(wepTiers[wepTier - 1])
    let wep_cat = match wep_tier {
        2 => Category::WepT2,
        3 => Category::WepT3,
        4 => Category::WepT4,
        _ => Category::WepT5,
    };
    let mut weapon = dungeon.generator.random_category(wep_cat, dungeon.depth);

    // Clear weapon's starting properties from Item.random()
    weapon.level = 0;
    weapon.enchantment = None;
    weapon.cursed = false;

    // 50%:+0, 30%:+1, 15%:+2, 5%:+3
    let item_level_roll = Random::float();
    let item_level = if item_level_roll < 0.5 {
        0
    } else if item_level_roll < 0.8 {
        1
    } else if item_level_roll < 0.95 {
        2
    } else {
        3
    };
    weapon.level = item_level;
    armor.level = item_level;

    // Always generate enchant+glyph first so the roll count is constant.
    let enchant = enchants::random_weapon_enchant(None).to_string();
    let glyph = enchants::random_armor_glyph(None).to_string();
    // 20% base chance (no ParchmentScrap trinket)
    let enchant_roll = Random::float();
    if enchant_roll <= 0.2 {
        weapon.enchantment = Some(enchant);
        armor.enchantment = Some(glyph);
    }

    weapon.source = Some("Ghost.Quest".into());
    armor.source = Some("Ghost.Quest".into());

    (weapon, armor)
}

fn exit_center_cell(room: &Room, map: &TerrainMap) -> Option<usize> {
    let cx = (room.left + room.right) / 2;
    let cy = (room.top + room.bottom) / 2;
    map.point_to_cell(cx, cy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::{dungeon_from_run, init_run};

    #[test]
    fn ghost_rewards_deterministic() {
        Random::reset_generators();
        let run = init_run(42);
        Random::push_generator_seeded(999);
        let (w1, a1) = {
            let mut d = dungeon_from_run(run.clone());
            d.depth = 4;
            generate_rewards(&mut d)
        };
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(999);
        let (w2, a2) = {
            let mut d = dungeon_from_run(run);
            d.depth = 4;
            generate_rewards(&mut d)
        };
        Random::pop_generator();

        assert_eq!(w1.class_name, w2.class_name);
        assert_eq!(w1.level, w2.level);
        assert_eq!(a1.class_name, a2.class_name);
        assert_eq!(a1.level, a2.level);
    }

    #[test]
    fn depth4_always_attempts_when_not_spawned() {
        // int_max(1) is always 0
        Random::reset_generators();
        for _ in 0..20 {
            assert_eq!(Random::int_max(1), 0);
        }
    }
}
