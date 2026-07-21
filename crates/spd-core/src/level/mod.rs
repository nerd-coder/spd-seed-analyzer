//! Headless level generation.

mod create_items;
mod shop;
mod special_loot;
mod terrain;

use crate::builders::{self, BuilderParams};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::quests;
use crate::random::Random;
use crate::report::{FloorReport, ItemEntry};
use crate::rooms::init_rooms::{self, BuilderKind};
use crate::rooms::room::clear_all_connections;
use crate::rooms::types::RoomKind;

pub use create_items::PlacedLoot;
pub use terrain::TerrainMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feeling {
    None,
    Chasm,
    Water,
    Grass,
    Dark,
    Large,
    Traps,
    Secrets,
}

impl Feeling {
    pub fn as_str(self) -> &'static str {
        match self {
            Feeling::None => "none",
            Feeling::Chasm => "chasm",
            Feeling::Water => "water",
            Feeling::Grass => "grass",
            Feeling::Dark => "dark",
            Feeling::Large => "large",
            Feeling::Traps => "traps",
            Feeling::Secrets => "secrets",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LevelState {
    pub depth: i32,
    pub feeling: Feeling,
    pub builder: Option<BuilderKind>,
    pub rooms: Vec<String>,
    pub build_ok: bool,
    pub forced_items: Vec<GeneratedItem>,
    pub placed_items: Vec<GeneratedItem>,
    pub quests: Vec<String>,
    pub complete: bool,
    pub map: Option<crate::report::FloorMap>,
}

impl LevelState {
    pub fn to_floor_report(&self) -> FloorReport {
        let mut items: Vec<ItemEntry> = Vec::new();
        for it in self.forced_items.iter().chain(self.placed_items.iter()) {
            if is_blacklisted(it) {
                continue;
            }
            // Title includes a "cursed " prefix; report that as a structured flag
            // and keep `name` free of it for chip-based UI.
            let full_title = it.title();
            let name = if it.cursed {
                full_title
                    .strip_prefix("cursed ")
                    .unwrap_or(&full_title)
                    .to_string()
            } else {
                full_title
            };
            items.push(ItemEntry {
                name,
                class_name: Some(it.class_name.clone()),
                category: format!("{:?}", it.category).to_ascii_lowercase(),
                cursed: it.cursed,
                source: it.source.clone(),
            });
        }
        FloorReport {
            depth: self.depth as u32,
            feeling: Some(self.feeling.as_str().to_string()),
            builder: self.builder.map(|b| match b {
                BuilderKind::Loop => "loop".to_string(),
                BuilderKind::FigureEight => "figure_eight".to_string(),
            }),
            rooms: self.rooms.clone(),
            items,
            quests: self.quests.clone(),
            map: self.map.clone(),
        }
    }
}

fn is_blacklisted(it: &GeneratedItem) -> bool {
    matches!(
        it.class_name.as_str(),
        "Gold"
            | "Dewdrop"
            | "IronKey"
            | "GoldenKey"
            | "CrystalKey"
            | "EnergyCrystal"
            | "CorpseDust"
            | "Embers"
            | "CeremonialCandle"
            | "Pickaxe"
    )
}

/// Level.create partial: forced drops → initRooms → build → minimal paint → createItems.
pub fn create_level_partial(dungeon: &mut DungeonState) -> LevelState {
    let depth_seed = dungeon.seed_cur_depth();
    Random::push_generator_seeded(depth_seed);

    let mut forced = Vec::new();
    let mut feeling = Feeling::None;
    let mut items_to_spawn: Vec<GeneratedItem> = Vec::new();

    // Forced drops + feelings only on RegularLevel (not boss / LastLevel).
    if dungeon.regular_level() {
        let mut food = dungeon
            .generator
            .random_category(Category::Food, dungeon.depth);
        food.source = Some("forced".into());
        // food goes to itemsToSpawn in Java Level.create
        items_to_spawn.push(food.clone());
        forced.push(food);

        if dungeon.pos_needed() {
            dungeon.limited.strength_potions += 1;
            let mut pot = GeneratedItem::new("PotionOfStrength", ItemCategory::Potion);
            pot.source = Some("forced".into());
            items_to_spawn.push(pot.clone());
            forced.push(pot);
        }
        if dungeon.sou_needed() {
            dungeon.limited.upgrade_scrolls += 1;
            let mut sou = GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll);
            sou.source = Some("forced".into());
            items_to_spawn.push(sou.clone());
            forced.push(sou);
        }
        if dungeon.as_needed() {
            dungeon.limited.arcane_styli += 1;
            let mut st = GeneratedItem::new("Stylus", ItemCategory::Other);
            st.source = Some("forced".into());
            items_to_spawn.push(st.clone());
            forced.push(st);
        }
        if dungeon.ench_stone_needed() {
            dungeon.limited.ench_stone = true;
            let mut st = GeneratedItem::new("StoneOfEnchantment", ItemCategory::Stone);
            st.source = Some("forced".into());
            items_to_spawn.push(st.clone());
            forced.push(st);
        }
        if dungeon.int_stone_needed() {
            dungeon.limited.int_stone = true;
            let mut st = GeneratedItem::new("StoneOfIntuition", ItemCategory::Stone);
            st.source = Some("forced".into());
            items_to_spawn.push(st.clone());
            forced.push(st);
        }
        if dungeon.trinket_cata_needed() {
            dungeon.limited.trinket_cata = true;
            let mut st = GeneratedItem::new("TrinketCatalyst", ItemCategory::Other);
            st.source = Some("forced".into());
            items_to_spawn.push(st.clone());
            forced.push(st);
        }

        if dungeon.depth > 1 {
            match Random::int_max(14) {
                0 => feeling = Feeling::Chasm,
                1 => feeling = Feeling::Water,
                2 => feeling = Feeling::Grass,
                3 => feeling = Feeling::Dark,
                4 => {
                    feeling = Feeling::Large;
                    let mut food2 = dungeon
                        .generator
                        .random_category(Category::Food, dungeon.depth);
                    food2.source = Some("forced".into());
                    items_to_spawn.push(food2.clone());
                    forced.push(food2);
                }
                5 => feeling = Feeling::Traps,
                6 => feeling = Feeling::Secrets,
                _ => {
                    let _ = Random::float();
                    let _ = Random::float();
                    feeling = Feeling::None;
                }
            }
        }
    }

    let mut builder = None;
    let mut room_names = Vec::new();
    let mut build_ok = false;
    let mut placed_items = Vec::new();
    let mut floor_map = None;
    let mut quests = Vec::new();

    // RegularLevel only — bosses + depth 26 LastLevel use dedicated layouts in SPD.
    if dungeon.regular_level() {
        let lab_needed = dungeon.lab_room_needed();
        let shop = dungeon.shop_on_level();
        let depth = dungeon.depth;
        let mut floor = init_rooms::init_rooms_regular(
            depth,
            feeling,
            shop,
            lab_needed,
            &mut dungeon.limited.lab_room,
            &mut dungeon.rooms.specials,
            &mut dungeon.rooms.secrets,
            &mut dungeon.rooms.region_secrets,
            &mut dungeon.rooms.pit_needed_depth,
            &mut dungeon.wandmaker,
            &mut dungeon.blacksmith,
            &mut dungeon.imp,
            &mut dungeon.generator,
        );
        builder = Some(floor.builder_kind);

        // Blacksmith.Quest generates smithRewards during initRooms (before shuffle/build).
        if let Some(bs) = quests::take_blacksmith_pending(&mut dungeon.blacksmith) {
            quests.push(bs.summary.clone());
            for mut reward in bs.rewards {
                // Apply stored enchant/glyph for display (SPD keeps them separate
                // until the player picks the smith option — still useful in report).
                if reward.category == crate::items::model::ItemCategory::Weapon {
                    if let Some(ref e) = bs.smith_enchant {
                        reward.enchantment = Some(e.clone());
                    }
                } else if reward.category == crate::items::model::ItemCategory::Armor {
                    if let Some(ref g) = bs.smith_glyph {
                        reward.enchantment = Some(g.clone());
                    }
                }
                placed_items.push(reward);
            }
        }

        // Imp.Quest generates its ring during initRooms (before shuffle/build).
        if let Some(imp) = quests::take_imp_pending(&mut dungeon.imp) {
            quests.push(imp.summary.clone());
            placed_items.push(imp.reward);
        }

        // Inner build retry loop (same rooms, clear connections)
        build_ok = builders::build_rooms(
            &mut floor.rooms,
            floor.builder_kind,
            floor.curve_intensity,
            floor.curve_offset,
            dungeon.depth,
            50,
        );

        if build_ok {
            if let Some(map) = terrain::paint_minimal(&floor.rooms) {
                floor_map = Some(crate::report::FloorMap {
                    width: map.width as u32,
                    height: map.height as u32,
                    tileset: terrain::tileset_for_depth(dungeon.depth).to_string(),
                    tiles: map.map.iter().map(|&t| t as u16).collect(),
                });

                // Shop stock: in SPD generated during setSize (mid-build). We run
                // after build / before other special paint so Generator still
                // advances before createItems (timing approximate).
                if floor
                    .rooms
                    .iter()
                    .any(|r| r.kind == RoomKind::Shop && !r.is_empty())
                {
                    for item in shop::generate_items(dungeon) {
                        placed_items.push(item);
                    }
                }

                // Special/secret room paint loot (before createItems; may consume itemsToSpawn).
                let special =
                    special_loot::special_room_loot(dungeon, &floor.rooms, &mut items_to_spawn);
                for p in special {
                    // Drop matching forced clones when a prize was pulled from itemsToSpawn.
                    if p.item
                        .source
                        .as_deref()
                        .is_some_and(|s| s.contains("Room") || s.contains("Secret"))
                    {
                        if let Some(pos) = forced.iter().position(|f| {
                            f.class_name == p.item.class_name
                                && f.source.as_deref() == Some("forced")
                        }) {
                            // Only remove once per prize consumption of unique forced types.
                            if matches!(
                                p.item.class_name.as_str(),
                                "TrinketCatalyst"
                                    | "PotionOfStrength"
                                    | "ScrollOfUpgrade"
                                    | "Stylus"
                                    | "StoneOfEnchantment"
                                    | "StoneOfIntuition"
                            ) {
                                forced.remove(pos);
                            }
                        }
                    }
                    placed_items.push(p.item);
                }

                // createMobs subset: Ghost (sewers) / Wandmaker (prison) before createItems.
                // Full mob placement still not ported — createItems RNG remains approximate.
                if let Some(exit) = floor.rooms.iter().find(|r| r.is_exit() && !r.is_empty()) {
                    if let Some(ghost) = quests::try_spawn_ghost(dungeon, exit, &map) {
                        quests.push(ghost.summary.clone());
                        placed_items.push(ghost.weapon);
                        placed_items.push(ghost.armor);
                    }
                }
                if let Some(entrance) = floor
                    .rooms
                    .iter()
                    .find(|r| r.is_entrance() && !r.is_empty())
                {
                    if let Some(wm) = quests::try_spawn_wandmaker(dungeon, entrance, &map) {
                        quests.push(wm.summary.clone());
                        placed_items.push(wm.wand1);
                        placed_items.push(wm.wand2);
                    }
                }

                let loot = create_items::create_items_main(
                    dungeon,
                    &floor.rooms,
                    &map,
                    feeling == Feeling::Large,
                    items_to_spawn,
                );
                for p in loot {
                    if p.item.source.as_deref() == Some("forced") {
                        // Room paint may add to itemsToSpawn (e.g. Storage → PotionOfLiquidFlame).
                        // Keep those in the report if not already listed under forced.
                        if !forced.iter().any(|f| f.class_name == p.item.class_name) {
                            forced.push(p.item);
                        }
                        continue;
                    }
                    let mut item = p.item;
                    if item.source.is_none() {
                        item.source = Some(p.heap_type.into());
                    } else if p.heap_type != "heap" {
                        item.source = Some(format!(
                            "{}:{}",
                            p.heap_type,
                            item.source.as_deref().unwrap_or("")
                        ));
                    }
                    placed_items.push(item);
                }
            }
        }

        room_names = floor
            .rooms
            .iter()
            .filter(|r| !r.is_empty())
            .map(|r| r.name.clone())
            .collect();
        let _ = clear_all_connections;
        let _ = BuilderParams::default();
    }

    Random::pop_generator();

    LevelState {
        depth: dungeon.depth,
        feeling,
        builder,
        rooms: room_names,
        build_ok,
        forced_items: forced,
        placed_items,
        quests,
        complete: build_ok,
        map: floor_map,
    }
}

pub fn analyze_floors(dungeon: &mut DungeonState, max_floors: u32) -> Vec<FloorReport> {
    let mut floors = Vec::new();
    let max = max_floors.clamp(1, 26) as i32;
    for depth in 1..=max {
        dungeon.depth = depth;
        dungeon.branch = 0;
        let level = create_level_partial(dungeon);
        floors.push(level.to_floor_report());
    }
    floors
}
