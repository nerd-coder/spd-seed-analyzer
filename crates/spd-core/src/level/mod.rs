//! Headless level generation (partial — forced drops, feelings, room selection).

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::random::Random;
use crate::report::{FloorReport, ItemEntry};
use crate::rooms::init_rooms::{self, BuilderKind, FloorRooms};

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
    /// Forced drops rolled at the start of `Level.create` (food, SoU, etc.).
    pub forced_items: Vec<GeneratedItem>,
    /// Placeholder until full room/item placement is ported.
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
                source: it.source.clone().or_else(|| Some("forced".into())),
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

/// Partial `Level.create()`:
/// 1. Forced drops + feeling
/// 2. For regular floors: builder selection + `initRooms` + shuffle
///
/// Geometry build / paint / createItems not yet implemented.
pub fn create_level_partial(dungeon: &mut DungeonState) -> LevelState {
    let depth_seed = dungeon.seed_cur_depth();
    Random::push_generator_seeded(depth_seed);

    let mut forced = Vec::new();
    let mut feeling = Feeling::None;

    if !dungeon.boss_level() && dungeon.branch == 0 {
        let mut food = dungeon
            .generator
            .random_category(Category::Food, dungeon.depth);
        food.source = Some("forced".into());
        forced.push(food);

        if dungeon.pos_needed() {
            dungeon.limited.strength_potions += 1;
            let mut pot = GeneratedItem::new("PotionOfStrength", ItemCategory::Potion);
            pot.source = Some("forced".into());
            forced.push(pot);
        }
        if dungeon.sou_needed() {
            dungeon.limited.upgrade_scrolls += 1;
            let mut sou = GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll);
            sou.source = Some("forced".into());
            forced.push(sou);
        }
        if dungeon.as_needed() {
            dungeon.limited.arcane_styli += 1;
            let mut st = GeneratedItem::new("Stylus", ItemCategory::Other);
            st.source = Some("forced".into());
            forced.push(st);
        }
        if dungeon.ench_stone_needed() {
            dungeon.limited.ench_stone = true;
            let mut st = GeneratedItem::new("StoneOfEnchantment", ItemCategory::Stone);
            st.source = Some("forced".into());
            forced.push(st);
        }
        if dungeon.int_stone_needed() {
            dungeon.limited.int_stone = true;
            let mut st = GeneratedItem::new("StoneOfIntuition", ItemCategory::Stone);
            st.source = Some("forced".into());
            forced.push(st);
        }
        if dungeon.trinket_cata_needed() {
            dungeon.limited.trinket_cata = true;
            let mut st = GeneratedItem::new("TrinketCatalyst", ItemCategory::Other);
            st.source = Some("forced".into());
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

    // Regular main-path floors only (sewers-style counts for all regions for now)
    if !dungeon.boss_level() && dungeon.branch == 0 && dungeon.depth <= 26 {
        let lab_needed = dungeon.lab_room_needed();
        let floor: FloorRooms = init_rooms::init_rooms_regular(
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
        room_names = floor.rooms.into_iter().map(|r| r.name).collect();
    }

    Random::pop_generator();

    LevelState {
        depth: dungeon.depth,
        feeling,
        builder,
        rooms: room_names,
        forced_items: forced,
        placed_items: Vec::new(),
        quests: Vec::new(),
        complete: false,
    }
}

/// Analyze floors 1..=max_floors with partial levelgen.
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
