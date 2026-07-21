//! Headless level generation.

mod create_items;
mod terrain;

use crate::builders::{self, BuilderParams};
use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::random::Random;
use crate::report::{FloorReport, ItemEntry};
use crate::rooms::init_rooms::{self, BuilderKind};
use crate::rooms::room::clear_all_connections;

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
}

impl LevelState {
    pub fn to_floor_report(&self) -> FloorReport {
        let mut items: Vec<ItemEntry> = Vec::new();
        for it in self.forced_items.iter().chain(self.placed_items.iter()) {
            if is_blacklisted(it) {
                continue;
            }
            items.push(ItemEntry {
                name: it.title(),
                category: format!("{:?}", it.category).to_ascii_lowercase(),
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

    if !dungeon.boss_level() && dungeon.branch == 0 {
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

    if !dungeon.boss_level() && dungeon.branch == 0 && dungeon.depth <= 26 {
        let lab_needed = dungeon.lab_room_needed();
        let mut floor = init_rooms::init_rooms_regular(
            dungeon.depth,
            feeling,
            dungeon.shop_on_level(),
            lab_needed,
            &mut dungeon.limited.lab_room,
            &mut dungeon.rooms.specials,
            &mut dungeon.rooms.secrets,
            &mut dungeon.rooms.region_secrets,
            &mut dungeon.rooms.pit_needed_depth,
        );
        builder = Some(floor.builder_kind);
        room_names = floor.rooms.iter().map(|r| r.name.clone()).collect();

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
                let loot = create_items::create_items_main(
                    dungeon,
                    &floor.rooms,
                    &map,
                    feeling == Feeling::Large,
                    items_to_spawn,
                );
                // forced items already listed; avoid double-counting forced from items_to_spawn
                for p in loot {
                    if p.item.source.as_deref() == Some("forced") {
                        // already in forced_items
                        continue;
                    }
                    let mut item = p.item;
                    if item.source.is_none() {
                        item.source = Some(p.heap_type.into());
                    } else if p.heap_type != "heap" {
                        item.source = Some(format!("{}:{}", p.heap_type, item.source.as_deref().unwrap_or("")));
                    }
                    placed_items.push(item);
                }
            }
        }

        // refresh room names after connection rooms added
        room_names = floor
            .rooms
            .iter()
            .filter(|r| !r.is_empty() || r.kind != crate::rooms::types::RoomKind::Connection)
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
        quests: Vec::new(),
        complete: build_ok,
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
