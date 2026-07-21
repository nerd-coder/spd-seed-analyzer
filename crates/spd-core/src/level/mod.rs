//! Headless level generation.

mod build;
mod create_items;
mod maze;
mod painter;
pub mod patch;
mod shop;
mod special_loot;
mod terrain;

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::quests;
use crate::random::Random;
use crate::report::{FloorReport, ItemEntry, MapMarker, MapMarkerKind};
use crate::rooms::init_rooms::BuilderKind;
use crate::rooms::types::RoomKind;

pub use create_items::PlacedLoot;
pub use terrain::TerrainMap;
pub(crate) use terrain::{ENTRANCE, ENTRANCE_SP, EXIT};

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
        let Some(floor) = build::regular_rooms(dungeon, feeling, shop, lab_needed) else {
            Random::pop_generator();
            return LevelState {
                depth: dungeon.depth,
                feeling,
                builder,
                rooms: room_names,
                build_ok,
                forced_items: forced,
                placed_items,
                quests,
                complete: false,
                map: floor_map,
            };
        };
        builder = Some(floor.builder_kind);
        build_ok = true;

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

        // RegularPainter: nTraps() is rolled when constructing the painter,
        // before room shuffle / placeDoors / special paint.
        let n_traps = painter::n_traps(dungeon.depth);

        let painted_map = if feeling == Feeling::Chasm {
            terrain::paint_minimal_with_chasm(&floor.rooms, true)
        } else {
            terrain::paint_minimal(&floor.rooms)
        };
        if let Some(mut map) = painted_map {
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
            // Includes RegularPainter shuffle + placeDoors + door-type upgrades.
            let special = special_loot::special_room_loot(
                dungeon,
                &floor.rooms,
                &mut map,
                &mut items_to_spawn,
                feeling,
            );
            let special_loot::SpecialPaintResult {
                loot: special_loot_items,
                mut doors,
                paint_order,
            } = special;
            for p in special_loot_items {
                // Drop matching forced clones when a prize was pulled from itemsToSpawn.
                if p.item
                    .source
                    .as_deref()
                    .is_some_and(|s| s.contains("Room") || s.contains("Secret"))
                {
                    if let Some(pos) = forced.iter().position(|f| {
                        f.class_name == p.item.class_name && f.source.as_deref() == Some("forced")
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

            // paintDoors: mergeRooms + hidden-door Float/Graph + terrain.
            painter::paint_doors(
                &mut map,
                &floor.rooms,
                &paint_order,
                dungeon.depth,
                feeling,
                &mut doors,
            );

            // Water / grass / traps / decorate on a separate generator.
            painter::paint_water_grass_traps(
                &mut map,
                &floor.rooms,
                &paint_order,
                &doors,
                dungeon.depth,
                feeling,
                n_traps,
            );

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
            let mut map_markers: Vec<MapMarker> = map
                .known_mobs
                .iter()
                .enumerate()
                .filter_map(|(cell, &label)| {
                    label.map(|label| MapMarker {
                        cell: cell as u32,
                        kind: MapMarkerKind::Mob,
                        label: label.to_string(),
                    })
                })
                .collect();

            for created in loot {
                let p = created.loot;
                if let Some(cell) = created.cell.filter(|&cell| cell < map.len()) {
                    let (kind, label) = match p.heap_type {
                        "mimic" => (MapMarkerKind::Mob, "Mimic".to_string()),
                        "golden_mimic" => (MapMarkerKind::Mob, "Golden Mimic".to_string()),
                        _ => (MapMarkerKind::Item, p.item.title()),
                    };
                    map_markers.push(MapMarker {
                        cell: cell as u32,
                        kind,
                        label,
                    });
                }
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

            // Some room painters know a heap cell but not which generated item belongs to it.
            // Keep that distinction explicit and do not fabricate identity-to-cell pairings.
            for (cell, &occupied) in map.heap_occupied.iter().enumerate() {
                if occupied
                    && cell < map.len()
                    && !map_markers.iter().any(|marker| marker.cell == cell as u32)
                {
                    map_markers.push(MapMarker {
                        cell: cell as u32,
                        kind: MapMarkerKind::Item,
                        label: "Room loot".to_string(),
                    });
                }
            }
            map_markers.sort_by_key(|marker| marker.cell);

            floor_map = Some(crate::report::FloorMap {
                width: map.width as u32,
                height: map.height as u32,
                tileset: terrain::tileset_for_depth(dungeon.depth).to_string(),
                tiles: map.map.iter().map(|&t| t as u16).collect(),
                tile_variance: tile_variance(map.len(), depth_seed),
                markers: map_markers,
            });
        }

        room_names = floor
            .rooms
            .iter()
            .filter(|r| !r.is_empty())
            .map(|r| r.name.clone())
            .collect();
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

/// Pinned `DungeonTileSheet.setupVariance`: a fresh seedCurDepth generator,
/// intentionally isolated from the active level-generation RNG.
fn tile_variance(len: usize, depth_seed: i64) -> Vec<u8> {
    Random::push_generator_seeded(depth_seed);
    let variance = (0..len).map(|_| Random::int_max(100) as u8).collect();
    Random::pop_generator();
    variance
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

#[cfg(test)]
mod map_report_tests {
    use super::*;

    #[test]
    fn tile_variance_is_deterministic_and_does_not_consume_level_rng() {
        Random::reset_generators();
        Random::push_generator_seeded(77);
        let first = Random::int();
        let variance = tile_variance(8, 1234);
        let after_variance = Random::int();
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(77);
        assert_eq!(Random::int(), first);
        assert_eq!(Random::int(), after_variance);
        Random::pop_generator();

        assert_eq!(variance, tile_variance(8, 1234));
        assert!(variance.iter().all(|&value| value < 100));
    }
}
