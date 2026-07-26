//! Headless level generation.

mod build;
mod create_items;
mod create_mobs;
mod map_facts;
mod maze;
mod painter;
pub mod patch;
mod quest_rewards;
mod room_public;
mod shop;
mod special_loot;
mod state;
mod terrain;
mod trinkets;

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::model::{ForcedDropRole, GeneratedItem, ItemCategory, ItemProvenance};
use crate::random::Random;
use crate::report::{FloorReport, MapProfile, MapTrinketProfile};

pub use create_items::PlacedLoot;
pub use state::LevelState;
pub use terrain::TerrainMap;
pub(crate) use terrain::{CUSTOM_DECO_EMPTY, DOOR, EMPTY_SP, ENTRANCE, ENTRANCE_SP, EXIT};

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

/// Level.create partial: forced drops → initRooms → build → minimal paint → createItems.
pub fn create_level_partial(dungeon: &mut DungeonState) -> LevelState {
    create_level_partial_with_profile(dungeon, false)
}

pub(crate) fn create_level_partial_with_profile(
    dungeon: &mut DungeonState,
    configured_profile: bool,
) -> LevelState {
    create_level_internal(dungeon, configured_profile, false)
}

fn create_level_layout_with_profile(
    dungeon: &mut DungeonState,
    configured_profile: bool,
) -> LevelState {
    create_level_internal(dungeon, configured_profile, true)
}

fn create_level_internal(
    dungeon: &mut DungeonState,
    configured_profile: bool,
    layout_only: bool,
) -> LevelState {
    let inherited_public_taint = dungeon.public_generation_tainted;
    let depth_seed = dungeon.seed_cur_depth();
    Random::push_generator_seeded(depth_seed);

    let mut forced = Vec::new();
    let mut feeling = Feeling::None;
    let mut items_to_spawn: Vec<GeneratedItem> = Vec::new();
    let mut runtime_sensitive_prebuild = false;
    let mut challenge_sensitive_upgrade_queue = false;

    // Forced drops + feelings only on RegularLevel (not boss / LastLevel).
    if dungeon.regular_level() {
        // HallsLevel.create queues these before RegularLevel/Level.create.
        if (21..=24).contains(&dungeon.depth) {
            for _ in 0..2 {
                let mut torch = GeneratedItem::new("Torch", ItemCategory::Other);
                torch.source = Some("forced".into());
                torch.provenance = ItemProvenance::Forced(ForcedDropRole::HallsTorch);
                items_to_spawn.push(torch.clone());
                forced.push(torch);
            }
        }
        let mut food = dungeon
            .generator
            .random_category(Category::Food, dungeon.depth);
        food.source = Some("forced".into());
        food.provenance = ItemProvenance::Forced(ForcedDropRole::BaseFood);
        // food goes to itemsToSpawn in Java Level.create
        items_to_spawn.push(food.clone());
        forced.push(food);

        if dungeon.pos_needed() {
            dungeon.limited.strength_potions += 1;
            let mut pot = GeneratedItem::new("PotionOfStrength", ItemCategory::Potion);
            pot.source = Some("forced".into());
            pot.provenance = ItemProvenance::Forced(ForcedDropRole::StrengthPotion);
            items_to_spawn.push(pot.clone());
            forced.push(pot);
        }
        if dungeon.sou_needed() {
            dungeon.limited.upgrade_scrolls += 1;
            challenge_sensitive_upgrade_queue = dungeon.limited.upgrade_scrolls % 2 == 0;
            if challenge_sensitive_upgrade_queue {
                dungeon.public_generation_tainted = true;
            }
            let mut sou = GeneratedItem::new("ScrollOfUpgrade", ItemCategory::Scroll);
            sou.source = Some("forced".into());
            sou.provenance = ItemProvenance::Forced(ForcedDropRole::UpgradeScroll {
                forbidden_runes_sensitive: challenge_sensitive_upgrade_queue,
            });
            items_to_spawn.push(sou.clone());
            forced.push(sou);
        }
        if dungeon.as_needed() {
            dungeon.limited.arcane_styli += 1;
            let mut st = GeneratedItem::new("Stylus", ItemCategory::Other);
            st.source = Some("forced".into());
            st.provenance = ItemProvenance::Forced(ForcedDropRole::ArcaneStylus);
            items_to_spawn.push(st.clone());
            forced.push(st);
        }
        if dungeon.ench_stone_needed() {
            dungeon.limited.ench_stone = true;
            let mut st = GeneratedItem::new("StoneOfEnchantment", ItemCategory::Stone);
            st.source = Some("forced".into());
            st.provenance = ItemProvenance::Forced(ForcedDropRole::EnchantmentStone);
            items_to_spawn.push(st.clone());
            forced.push(st);
        }
        if dungeon.int_stone_needed() {
            dungeon.limited.int_stone = true;
            let mut st = GeneratedItem::new("StoneOfIntuition", ItemCategory::Stone);
            st.source = Some("forced".into());
            st.provenance = ItemProvenance::Forced(ForcedDropRole::IntuitionStone);
            items_to_spawn.push(st.clone());
            forced.push(st);
        }
        if dungeon.trinket_cata_needed() {
            dungeon.limited.trinket_cata = true;
            let mut st = GeneratedItem::new("TrinketCatalyst", ItemCategory::Other);
            st.source = Some("forced".into());
            st.provenance = ItemProvenance::Forced(ForcedDropRole::TrinketCatalyst);
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
                    food2.provenance = ItemProvenance::Forced(ForcedDropRole::LargeFeelingFood);
                    items_to_spawn.push(food2.clone());
                    forced.push(food2);
                }
                5 => feeling = Feeling::Traps,
                6 => feeling = Feeling::Secrets,
                _ => {
                    runtime_sensitive_prebuild = !configured_profile;
                    dungeon.public_generation_tainted |= !configured_profile;
                    feeling = trinkets::override_default_feeling();
                }
            }
        }
    }

    let mut builder = None;
    let mut room_names = Vec::new();
    let mut room_bounds = Vec::new();
    let mut build_ok = false;
    let mut placed_items = Vec::new();
    // A prior floor's divergent population can taint persistent Generator
    // state, maps, and ordinary item identities. It does not taint this
    // floor's freshly seeded quest-selection stream. Keep the current-floor
    // boundary separate so later quest presence and invariant reward fields
    // remain public.
    let runtime_sensitive_quest_prebuild = runtime_sensitive_prebuild;
    runtime_sensitive_prebuild |= inherited_public_taint;
    let mut runtime_sensitive_placed_items_from = runtime_sensitive_prebuild.then_some(0);
    let mut runtime_sensitive_quests_from = runtime_sensitive_quest_prebuild.then_some(0);
    let mut floor_map = None;
    let mut layout_map = None;
    let mut quests = Vec::new();
    let mut quest_public_labels = Vec::new();
    let mut runtime_sensitive_map = runtime_sensitive_prebuild || challenge_sensitive_upgrade_queue;
    let mut runtime_sensitive_layout = runtime_sensitive_prebuild;
    let runtime_sensitive_feeling = runtime_sensitive_prebuild;
    let mut room_public_facts = Vec::new();
    let initial_forced_items = forced.clone();
    let mut pre_items_rng_probe = Vec::new();
    let mut pre_mobs_rng_probe = Vec::new();
    let mut pre_paint_rng_probe = Vec::new();

    // RegularLevel only — bosses + depth 26 LastLevel use dedicated layouts in SPD.
    if dungeon.regular_level() {
        let shop = dungeon.shop_on_level();
        let Some(mut floor) = build::regular_rooms(dungeon, feeling, shop) else {
            Random::pop_generator();
            return LevelState {
                depth: dungeon.depth,
                feeling,
                builder,
                rooms: room_names,
                room_bounds,
                build_ok,
                forced_items: forced,
                initial_forced_items,
                placed_items,
                runtime_sensitive_placed_items_from,
                runtime_sensitive_quests_from,
                quests,
                quest_public_labels,
                runtime_sensitive_map,
                runtime_sensitive_layout,
                runtime_sensitive_feeling,
                room_public_facts,
                complete: false,
                map: floor_map,
                layout_map,
                pre_items_rng_probe: Vec::new(),
                pre_mobs_rng_probe: Vec::new(),
                pre_paint_rng_probe: Vec::new(),
            };
        };
        builder = Some(floor.builder_kind);
        build_ok = true;

        let pending_quests = quest_rewards::take_pending(dungeon);
        placed_items.extend(pending_quests.items);
        quests.extend(pending_quests.summaries);
        quest_public_labels.extend(pending_quests.public_labels);

        if let (Some(wand1), Some(wand2)) = (
            dungeon.wandmaker.wand1.as_ref(),
            dungeon.wandmaker.wand2.as_ref(),
        ) {
            for wand in [wand1, wand2] {
                let mut persisted = wand.clone();
                persisted.provenance = crate::items::model::ItemProvenance::Quest(
                    crate::items::model::QuestRewardRole::WandmakerPersisted,
                );
                placed_items.push(persisted);
            }
        }

        // RegularPainter: nTraps() is rolled when constructing the painter,
        // before room shuffle / placeDoors / special paint.
        terrain::shift_rooms_for_painter(&mut floor.rooms, feeling == Feeling::Chasm);
        let n_traps = painter::n_traps(dungeon.depth);
        if matches!(dungeon.depth, 1..=4 | 6..=9 | 11..=14 | 16..=19 | 21) {
            pre_paint_rng_probe = Random::peek_ints(8);
        }

        let painted_map = if feeling == Feeling::Chasm {
            terrain::paint_minimal_with_chasm(&floor.rooms, true)
        } else {
            terrain::paint_minimal(&floor.rooms)
        };
        if let Some(mut map) = painted_map {
            // ShopRoom lazily generates stock when the builder first asks for
            // its minimum size; `regular_rooms` retains that exact inventory.
            placed_items.extend(floor.shop_items.clone());
            if floor.shop_items.iter().any(|item| {
                item.provenance
                    == crate::items::model::ItemProvenance::Shop(
                        crate::items::model::ShopStockRole::DeckRareArtifactOrRing,
                    )
            }) {
                runtime_sensitive_placed_items_from.get_or_insert(placed_items.len());
                runtime_sensitive_quests_from.get_or_insert(quests.len());
                runtime_sensitive_map = true;
                runtime_sensitive_layout = true;
                dungeon.public_generation_tainted = true;
            }

            // Special/secret room paint loot (before createItems; may consume itemsToSpawn).
            // Includes RegularPainter shuffle + placeDoors + door-type upgrades.
            let special = special_loot::special_room_loot(
                dungeon,
                &floor.rooms,
                &mut map,
                &mut items_to_spawn,
                &floor.shop_items,
                feeling,
            );
            let special_loot::SpecialPaintResult {
                loot: special_loot_items,
                mut doors,
                paint_order,
                first_sensitive_loot_index,
                room_public_facts: special_room_public_facts,
            } = special;
            room_public_facts.extend(special_room_public_facts);
            if let Some(first_sensitive_loot_index) = first_sensitive_loot_index {
                runtime_sensitive_placed_items_from
                    .get_or_insert(placed_items.len() + first_sensitive_loot_index);
                runtime_sensitive_quests_from.get_or_insert(quests.len());
                runtime_sensitive_map = true;
                dungeon.public_generation_tainted = true;
            }
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
                        // Room painters retain `:forced` when an arbitrary
                        // findPrizeItem pull consumes a pre-build queue item.
                        let consumed_forced = p
                            .item
                            .source
                            .as_deref()
                            .is_some_and(|source| source.ends_with(":forced"));
                        if consumed_forced
                            || matches!(
                                p.item.class_name.as_str(),
                                "TrinketCatalyst"
                                    | "PotionOfStrength"
                                    | "ScrollOfUpgrade"
                                    | "Stylus"
                                    | "StoneOfEnchantment"
                                    | "StoneOfIntuition"
                            )
                        {
                            forced.remove(pos);
                        }
                    }
                }
                // Garden plant entries are synthetic painter facts rather than
                // portable rewards. Blob-held well and SacrificeRoom facts
                // remain useful public analyzer results even though
                // Level.heaps does not contain them.
                if p.heap_type != "plant" {
                    placed_items.push(p.item);
                }
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

            layout_map = Some(
                map_facts::MapFacts::from_room_paint(&map)
                    .into_floor_map(&map, dungeon.depth, dungeon.branch, depth_seed)
                    .into_layout_only(),
            );

            if !layout_only {
                // RegularPainter shuffles the actual Java `rooms` ArrayList in
                // place. Later createMobs/createItems therefore observe painter
                // order, not the builder's original room order.
                let population_rooms: Vec<_> = paint_order
                    .iter()
                    .filter_map(|&index| floor.rooms.get(index).cloned())
                    .collect();

                if matches!(dungeon.depth, 1..=4 | 6..=9 | 11..=14 | 16..=19 | 21) {
                    pre_mobs_rng_probe = Random::peek_ints(8);
                }
                let spawned = quest_rewards::spawn_npcs(dungeon, &floor.rooms, &mut map);
                placed_items.extend(spawned.items);
                quests.extend(spawned.summaries);
                quest_public_labels.extend(spawned.public_labels);
                if spawned.wand_rng_tail_sensitive {
                    // The room and quest type were selected during initRooms,
                    // before painter callbacks. Even if painting makes the sampled
                    // wand identities unsafe, the public Wandmaker summary and the
                    // invariant two-wand reward contract remain valid. Only a
                    // pre-build divergence can invalidate the selection itself.
                    if !runtime_sensitive_quest_prebuild {
                        runtime_sensitive_quests_from = None;
                    }
                    runtime_sensitive_placed_items_from.get_or_insert(placed_items.len());
                    runtime_sensitive_map = true;
                    dungeon.public_generation_tainted = true;
                }

                let _ambient_mobs_consumed = if dungeon.regular_level() {
                    create_mobs::create_regular(
                        dungeon.depth,
                        feeling == Feeling::Large,
                        &population_rooms,
                        &mut map,
                    )
                } else {
                    false
                };

                if matches!(dungeon.depth, 1..=4 | 6..=9 | 11..=14 | 16..=19 | 21) {
                    pre_items_rng_probe = Random::peek_ints(8);
                }
                let loot = create_items::create_items_main(
                    dungeon,
                    &population_rooms,
                    &mut map,
                    feeling == Feeling::Large,
                    items_to_spawn,
                );
                let mut map_facts = map_facts::MapFacts::from_room_paint(&map);

                for created in loot {
                    map_facts.add_created_loot(&created, map.len());
                    let p = created.loot;
                    if matches!(
                        p.item.source.as_deref(),
                        Some("forced") | Some("items_to_spawn")
                    ) {
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
                floor_map =
                    Some(map_facts.into_floor_map(&map, dungeon.depth, dungeon.branch, depth_seed));
            }
        }

        room_names = floor
            .rooms
            .iter()
            .filter(|r| !r.is_empty())
            .map(|r| r.name.clone())
            .collect();
        room_bounds = floor
            .rooms
            .iter()
            .filter(|room| !room.is_empty())
            .map(|room| state::LevelRoomFact {
                class_name: room.name.clone(),
                left: room.left,
                top: room.top,
                right: room.right,
                bottom: room.bottom,
            })
            .collect();
        room_bounds.sort();
    }

    Random::pop_generator();

    LevelState {
        depth: dungeon.depth,
        feeling,
        builder,
        rooms: room_names,
        room_bounds,
        build_ok,
        forced_items: forced,
        initial_forced_items,
        placed_items,
        runtime_sensitive_placed_items_from,
        runtime_sensitive_quests_from,
        quests,
        quest_public_labels,
        runtime_sensitive_map,
        runtime_sensitive_layout,
        runtime_sensitive_feeling,
        room_public_facts,
        complete: build_ok,
        map: floor_map,
        layout_map,
        pre_items_rng_probe,
        pre_mobs_rng_probe,
        pre_paint_rng_probe,
    }
}

pub fn analyze_layouts_with_profile(
    dungeon: &mut DungeonState,
    max_floors: u32,
    profile: &MapProfile,
) -> Vec<FloorReport> {
    let mut floors = Vec::new();
    trinkets::reset(dungeon.seed);
    let max = max_floors.clamp(1, 26) as i32;
    for depth in 1..=max {
        dungeon.depth = depth;
        dungeon.branch = 0;
        trinkets::set_held(profile.trinket);
        let level = create_level_layout_with_profile(dungeon, true);
        floors.push(level.to_floor_report_with_map(true));
    }
    floors
}

pub fn analyze_floors(dungeon: &mut DungeonState, max_floors: u32) -> Vec<FloorReport> {
    analyze_floors_with_profile(dungeon, max_floors, None)
}

pub fn analyze_floors_with_profile(
    dungeon: &mut DungeonState,
    max_floors: u32,
    profile: Option<&MapProfile>,
) -> Vec<FloorReport> {
    let mut floors = Vec::new();
    trinkets::reset(dungeon.seed);
    let max = max_floors.clamp(1, 26) as i32;
    for depth in 1..=max {
        dungeon.depth = depth;
        dungeon.branch = 0;
        let trinket = profile
            .map(|profile| profile.trinket)
            .unwrap_or(MapTrinketProfile::NoMapAffectingTrinkets);
        trinkets::set_held(trinket);
        let configured = profile.is_some();
        let level = create_level_partial_with_profile(dungeon, configured);
        floors.push(level.to_floor_report_with_map(configured));
    }
    floors
}

#[cfg(test)]
#[path = "map_report_tests.rs"]
mod map_report_tests;
