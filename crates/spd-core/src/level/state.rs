//! Internal per-floor state and its public report projection.

use crate::items::model::{GeneratedItem, ItemProvenance, QuestRewardRole, ShopStockRole};
use crate::report::{
    group_item_entries, FloorMap, FloorReport, ItemCondition, ItemDependencyCondition,
    ItemEnchantment, ItemEntry, ItemPredictionKind, ItemSpawnCondition, NumericRange, QuestReport,
};
use crate::rooms::init_rooms::BuilderKind;
use crate::trinkets::{ArtifactEvent, ArtifactEventAction, ArtifactKind};

use super::Feeling;

#[path = "state/forced_queue.rs"]
mod forced_queue;
use forced_queue::public_entries as forced_public_entries;

#[path = "state/conditions.rs"]
mod conditions;
use conditions::{item_conditions, item_conditions_typed, legacy_item_notes, parchment_condition};

#[path = "state/encounters.rs"]
mod encounters;

#[path = "state/baseline.rs"]
mod baseline;
use baseline::item_entry as baseline_item_entry;

#[path = "state/sources.rs"]
mod sources;
use sources::reported_source;

#[path = "state_map.rs"]
mod state_map;
use state_map::{guaranteed_appearances, reported_level};

fn prediction_kind(item: &GeneratedItem) -> ItemPredictionKind {
    match item.provenance {
        ItemProvenance::Shop(
            ShopStockRole::DeckWeapon { .. }
            | ShopStockRole::DeckMissile { .. }
            | ShopStockRole::ChooseBag
            | ShopStockRole::DeckRareWand
            | ShopStockRole::DeckRareRing
            | ShopStockRole::DeckRareArtifactOrRing,
        ) => ItemPredictionKind::Constrained,
        ItemProvenance::Quest(
            QuestRewardRole::GhostWeapon { .. }
            | QuestRewardRole::WandmakerWand
            | QuestRewardRole::ImpRing
            | QuestRewardRole::BlacksmithWeapon { .. }
            | QuestRewardRole::BlacksmithMissile { .. }
            | QuestRewardRole::BlacksmithArmor { .. }
            | QuestRewardRole::BlacksmithRoomWeapon { .. }
            | QuestRewardRole::BlacksmithRoomMissile { .. },
        ) => ItemPredictionKind::Constrained,
        ItemProvenance::Quest(
            QuestRewardRole::GhostArmor { .. } | QuestRewardRole::BlacksmithRoomArmor { .. },
        ) => ItemPredictionKind::Exact,
        ItemProvenance::Room(_) | ItemProvenance::Forced(_) => ItemPredictionKind::Constrained,
        _ => ItemPredictionKind::Exact,
    }
}

fn merge_identical_items(items: Vec<ItemEntry>) -> Vec<ItemEntry> {
    let mut merged: Vec<ItemEntry> = Vec::with_capacity(items.len());
    for item in items {
        let matching = item.class_name.is_some().then(|| {
            merged.iter_mut().find(|existing| {
                existing.name == item.name
                    && existing.class_name == item.class_name
                    && existing.candidate_classes == item.candidate_classes
                    && existing.category == item.category
                    && existing.tier == item.tier
                    && existing.tier_range == item.tier_range
                    && existing.level == item.level
                    && existing.level_range == item.level_range
                    && existing.cursed == item.cursed
                    && existing.enchantment == item.enchantment
                    && existing.prediction == item.prediction
                    && existing.spawn_conditions == item.spawn_conditions
                    && existing.conditions == item.conditions
                    && existing.source == item.source
            })
        });
        if let Some(Some(existing)) = matching {
            existing.quantity = existing.quantity.saturating_add(item.quantity);
        } else {
            merged.push(item);
        }
    }
    merged
}

pub(super) fn is_runtime_sensitive_main_loot(item: &GeneratedItem) -> bool {
    is_runtime_sensitive_loot_source(item.source.as_deref())
}

pub(super) fn is_runtime_sensitive_loot_source(source: Option<&str>) -> bool {
    source
        .and_then(|source| source.rsplit(':').next())
        .is_some_and(|origin| matches!(origin, "heap" | "mimic" | "golden_mimic"))
}

fn is_unpublished_main_loot(item: &GeneratedItem) -> bool {
    item.source
        .as_deref()
        .and_then(|source| source.rsplit(':').next())
        .is_some_and(|origin| matches!(origin, "mimic" | "golden_mimic"))
}

fn is_exact_floor_one_room_prize(
    depth: i32,
    runtime_sensitive_layout: bool,
    item: &GeneratedItem,
) -> bool {
    depth == 1
        && !runtime_sensitive_layout
        && (matches!(item.provenance, ItemProvenance::Room(_))
            || item.source.as_deref().is_some_and(|source| {
                source.ends_with(":forced")
                    // These painters predate the common Room provenance tag.
                    || matches!(
                        source,
                        "SacrificeRoom" | "CryptRoom" | "StatueRoom"
                    )
            }))
}

#[derive(Debug, Clone)]
pub struct LevelState {
    pub depth: i32,
    pub feeling: Feeling,
    pub builder: Option<BuilderKind>,
    pub rooms: Vec<String>,
    #[doc(hidden)]
    pub room_bounds: Vec<LevelRoomFact>,
    /// Pinned builder list order at the `RegularPainter.shuffle` boundary.
    #[doc(hidden)]
    pub pre_shuffle_room_bounds: Vec<LevelRoomFact>,
    pub build_ok: bool,
    pub forced_items: Vec<GeneratedItem>,
    /// Exact initial queue snapshot before room callbacks consume/reposition it.
    /// Internal parity evidence only; public output reports guaranteed spawns.
    #[doc(hidden)]
    pub initial_forced_items: Vec<GeneratedItem>,
    pub placed_items: Vec<GeneratedItem>,
    /// First placed-item index generated after a runtime-sensitive shop rare
    /// artifact call. Internal facts remain exact; public projection omits the
    /// tail because its RNG path can differ with artifact exhaustion/history.
    #[doc(hidden)]
    pub runtime_sensitive_placed_items_from: Option<usize>,
    /// First quest selected after the runtime-sensitive shop callback.
    #[doc(hidden)]
    pub runtime_sensitive_quests_from: Option<usize>,
    pub quests: Vec<QuestReport>,
    #[doc(hidden)]
    pub runtime_sensitive_map: bool,
    /// Builder and room metadata can depend on a pre-build player-state callback.
    #[doc(hidden)]
    pub runtime_sensitive_layout: bool,
    /// Room selection and the builder are fixed before room painting. Later
    /// paint callbacks and inherited deck taint may hide a map without
    /// invalidating these facts.
    #[doc(hidden)]
    pub runtime_sensitive_rooms: bool,
    /// The baseline feeling can be overridden by held trinkets before build.
    #[doc(hidden)]
    pub runtime_sensitive_feeling: bool,
    /// This replay uses the UI's declared fresh, no-history profile.
    #[doc(hidden)]
    pub baseline_projection: bool,
    #[doc(hidden)]
    pub room_public_facts: Vec<super::room_public::RoomPublicFact>,
    #[doc(hidden)]
    pub complete: bool,
    pub map: Option<FloorMap>,
    /// Snapshot after room/painter terrain, before NPC, mob, and item population.
    pub layout_map: Option<FloorMap>,
    /// Non-consuming parity probe at the `createItems` entry boundary.
    #[doc(hidden)]
    pub pre_items_rng_probe: Vec<i32>,
    /// Non-consuming parity probe at the `createMobs` entry boundary.
    #[doc(hidden)]
    pub pre_mobs_rng_probe: Vec<i32>,
    /// Non-consuming parity probe before `RegularPainter.paint`.
    #[doc(hidden)]
    pub pre_paint_rng_probe: Vec<i32>,
    /// Non-consuming main-RNG checkpoints after each regular-room paint callback.
    #[doc(hidden)]
    pub room_paint_rng_checkpoints: Vec<RoomPaintRngCheckpoint>,
    /// Non-consuming main-RNG checkpoint after `paintDoors`.
    #[doc(hidden)]
    pub post_doors_rng_probe: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomPaintRngCheckpoint {
    pub room: String,
    pub rng: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LevelRoomFact {
    pub class_name: String,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl LevelState {
    pub fn to_floor_report(&self) -> FloorReport {
        self.to_floor_report_with_map(false)
    }

    pub fn to_floor_report_with_map(&self, allow_map: bool) -> FloorReport {
        self.to_floor_report_with_context(allow_map, None)
    }

    pub(crate) fn to_floor_report_with_trinket_availability(
        &self,
        allow_map: bool,
        first_effective_trinket_depth: u32,
    ) -> FloorReport {
        self.to_floor_report_with_context(allow_map, Some(first_effective_trinket_depth))
    }

    fn to_floor_report_with_context(
        &self,
        allow_map: bool,
        first_effective_trinket_depth: Option<u32>,
    ) -> FloorReport {
        let mut items = forced_public_entries(self.depth, &self.initial_forced_items);
        let baseline_items: Vec<_> = if self.baseline_projection {
            self.placed_items
                .iter()
                .filter_map(baseline_item_entry)
                .filter(crate::report::is_baseline_highlight)
                .filter(|item| {
                    self.depth > 1
                        || !matches!(
                            item.source.as_deref(),
                            Some(
                                "PoolRoom:equipment"
                                    | "SuspiciousChestRoom:gold"
                                    | "SuspiciousChestRoom:mimic_reward"
                            )
                        )
                })
                .collect()
        } else {
            Vec::new()
        };
        let exact_floor_one_room_prize_indices: Vec<_> = if self.depth == 1
            && !self.runtime_sensitive_layout
        {
            self.placed_items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    is_exact_floor_one_room_prize(self.depth, self.runtime_sensitive_layout, item)
                })
                .map(|(index, _)| index)
                .collect()
        } else {
            Vec::new()
        };
        // A Floor 1 room can relocate a queued guaranteed spawn. Publish the
        // item at its reward source, rather than listing it twice or exposing
        // the private queue-consumption lifecycle.
        for &index in &exact_floor_one_room_prize_indices {
            let prize = &self.placed_items[index];
            items.retain(|entry| {
                !(entry.source.as_deref() == Some("guaranteed floor spawn")
                    && entry.class_name.as_deref() == Some(&prize.class_name))
            });
        }
        let mut shop_items = Vec::new();
        let mut has_shop = false;
        for (index, item) in self.placed_items.iter().enumerate() {
            let exact_floor_one_room_prize = exact_floor_one_room_prize_indices.contains(&index);
            let baseline_room_prize = self.baseline_projection
                && self.depth > 1
                && matches!(
                    item.source.as_deref(),
                    Some("SacrificeRoom" | "CryptRoom" | "StatueRoom")
                );
            let past_runtime_sensitive_boundary = self
                .runtime_sensitive_placed_items_from
                .is_some_and(|boundary| index >= boundary);
            // Supported floors contain at most one quest. A boundary at zero
            // means its selection is sensitive; a boundary after the first
            // summary means that quest and all of its reward entries were
            // already fixed before the divergent callback.
            let quest_past_runtime_sensitive_boundary =
                self.runtime_sensitive_quests_from == Some(0);
            // Shop stock is generated before the rare artifact constructor can
            // alter the remaining floor stream. Its fixed entries and public
            // deck constraints therefore remain safe even when that callback
            // suppresses the layout and later loot.
            if past_runtime_sensitive_boundary && !baseline_room_prize {
                match item.provenance {
                    ItemProvenance::Shop(_) => {}
                    ItemProvenance::Quest(_) if !quest_past_runtime_sensitive_boundary => {}
                    _ => continue,
                }
            }
            if (is_blacklisted(item) && !exact_floor_one_room_prize)
                || (is_unpublished_main_loot(item) && !exact_floor_one_room_prize)
            {
                continue;
            }
            // Outside Floor 1 Sacrifice is represented only by its static
            // room contract. Its sampled weapon depends on prior generator
            // history and remains internal for Java parity.
            if item.source.as_deref() == Some("SacrificeRoom")
                && !exact_floor_one_room_prize
                && !baseline_room_prize
            {
                continue;
            }
            if item.provenance == ItemProvenance::Quest(QuestRewardRole::WandmakerPersisted) {
                continue;
            }
            // Room contracts are emitted independently of the sampled exact
            // items so runtime-sensitive count/control flow cannot leak.
            if matches!(item.provenance, ItemProvenance::Room(_))
                && !exact_floor_one_room_prize
                && !baseline_room_prize
            {
                continue;
            }
            let artifact_conditional = item.artifact_conditional
                && item
                    .source
                    .as_deref()
                    .is_some_and(|source| source.rsplit(':').next() == Some("heap"));
            let imp_shop_conditional = item.source.as_deref() == Some("ImpShopRoom");
            let prediction = if artifact_conditional {
                ItemPredictionKind::Constrained
            } else {
                prediction_kind(item)
            };
            let shop_role = match item.provenance {
                ItemProvenance::Shop(role) => {
                    has_shop = true;
                    Some(role)
                }
                ItemProvenance::None
                | ItemProvenance::Quest(_)
                | ItemProvenance::Room(_)
                | ItemProvenance::Forced(_) => None,
            };
            // Bag presence and identity depend on inventory/limited-drop
            // history. A single conditional constraint is added below.
            if shop_role == Some(ShopStockRole::ChooseBag) {
                continue;
            }
            let constrained = prediction == ItemPredictionKind::Constrained;
            let quest_role = match item.provenance {
                ItemProvenance::Quest(role) => Some(role),
                _ => None,
            };
            let full_title = if quest_role.is_some() {
                let mut public_item = item.clone();
                public_item.enchantment = None;
                public_item.title()
            } else {
                item.title()
            };
            let exact_name = if exact_floor_one_room_prize && item.class_name == "Food" {
                "ration of food".to_string()
            } else if item.category == crate::items::model::ItemCategory::Gold {
                "gold".to_string()
            } else if item.cursed {
                full_title
                    .strip_prefix("cursed ")
                    .unwrap_or(&full_title)
                    .to_string()
            } else {
                full_title
            };
            let tier = match (shop_role, quest_role) {
                (
                    Some(ShopStockRole::DeckWeapon { tier } | ShopStockRole::DeckMissile { tier }),
                    _,
                ) => Some(tier),
                (
                    _,
                    Some(
                        QuestRewardRole::GhostWeapon { tier, .. }
                        | QuestRewardRole::GhostArmor { tier, .. },
                    ),
                ) => Some(tier),
                (
                    _,
                    Some(
                        QuestRewardRole::BlacksmithWeapon { .. }
                        | QuestRewardRole::BlacksmithMissile { .. }
                        | QuestRewardRole::BlacksmithArmor { .. },
                    ),
                ) => None,
                (
                    _,
                    Some(
                        QuestRewardRole::BlacksmithRoomWeapon { tier }
                        | QuestRewardRole::BlacksmithRoomMissile { tier }
                        | QuestRewardRole::BlacksmithRoomArmor { tier },
                    ),
                ) => Some(tier),
                _ if constrained => crate::generator::weapon_tier_for_class(&item.class_name),
                _ => None,
            };
            let constrained_name = match (shop_role, quest_role) {
                (Some(ShopStockRole::DeckWeapon { .. }), _) => "weapon stock",
                (Some(ShopStockRole::DeckMissile { .. }), _) => "missile weapon stock",
                (Some(ShopStockRole::DeckRareWand), _) => "wand stock",
                (Some(ShopStockRole::DeckRareRing), _) => "ring stock",
                (Some(ShopStockRole::DeckRareArtifactOrRing), _) => "artifact or ring",
                (_, Some(QuestRewardRole::GhostWeapon { .. })) => "Ghost weapon reward",
                (_, Some(QuestRewardRole::GhostArmor { .. })) => "Ghost armor reward",
                (_, Some(QuestRewardRole::WandmakerWand)) => "wand reward",
                (_, Some(QuestRewardRole::BlacksmithWeapon { .. })) => "Blacksmith weapon option",
                (_, Some(QuestRewardRole::BlacksmithMissile { .. })) => "Blacksmith missile option",
                (_, Some(QuestRewardRole::BlacksmithArmor { .. })) => "Blacksmith armor option",
                (_, Some(QuestRewardRole::BlacksmithRoomWeapon { .. })) => "Blacksmith room weapon",
                (_, Some(QuestRewardRole::BlacksmithRoomMissile { .. })) => {
                    "Blacksmith room missile weapon"
                }
                (_, Some(QuestRewardRole::BlacksmithRoomArmor { .. })) => "Blacksmith room armor",
                (_, Some(QuestRewardRole::ImpRing)) => "ring reward",
                _ => "weapon reward",
            };
            let ghost_weapon = match quest_role {
                Some(QuestRewardRole::GhostWeapon {
                    minimum_parchment_level,
                    ..
                }) => Some(minimum_parchment_level),
                _ => None,
            };
            let ghost_enchantment_condition = match quest_role {
                Some(
                    QuestRewardRole::GhostWeapon {
                        minimum_parchment_level,
                        ..
                    }
                    | QuestRewardRole::GhostArmor {
                        minimum_parchment_level,
                        ..
                    },
                ) => Some(minimum_parchment_level),
                _ => None,
            };
            let minimum_parchment_level = ghost_enchantment_condition.flatten();
            let reachable_parchment_level = minimum_parchment_level.filter(|_| {
                first_effective_trinket_depth
                    .is_none_or(|first_depth| self.depth as u32 >= first_depth)
            });
            let potential_enchantment_is_reachable =
                minimum_parchment_level.is_none() || reachable_parchment_level.is_some();
            let blacksmith_smith_option = matches!(
                quest_role,
                Some(
                    QuestRewardRole::BlacksmithWeapon { .. }
                        | QuestRewardRole::BlacksmithMissile { .. }
                        | QuestRewardRole::BlacksmithArmor { .. }
                )
            );
            let prediction = if exact_floor_one_room_prize
                || ghost_weapon.is_some_and(|_| item.candidate_classes.len() == 1)
            {
                ItemPredictionKind::Exact
            } else if baseline_room_prize {
                ItemPredictionKind::Baseline
            } else {
                prediction
            };
            let constrained = prediction == ItemPredictionKind::Constrained;
            let candidate_classes = ghost_weapon
                .filter(|_| item.candidate_classes.len() > 1)
                .map(|_| item.candidate_classes.clone())
                .unwrap_or_else(|| {
                    if artifact_conditional {
                        vec![item.class_name.clone()]
                    } else {
                        Vec::new()
                    }
                });
            let mut conditions = item_conditions_typed(quest_role, imp_shop_conditional);
            if exact_floor_one_room_prize
                && item.source.as_deref().is_some_and(|source| {
                    source
                        .rsplit(':')
                        .next()
                        .is_some_and(|tail| tail.contains("mimic"))
                })
            {
                conditions.push(ItemCondition::Runtime {
                    state_id: "defeat_mimic".into(),
                });
            }
            let entry = ItemEntry {
                name: if constrained {
                    constrained_name.to_string()
                } else {
                    exact_name
                },
                quantity: item.quantity.max(1),
                class_name: (!constrained).then(|| item.class_name.clone()),
                candidate_classes,
                category: if shop_role == Some(ShopStockRole::DeckRareArtifactOrRing) {
                    "other".into()
                } else {
                    format!("{:?}", item.category).to_ascii_lowercase()
                },
                tier,
                tier_range: blacksmith_smith_option.then_some(NumericRange { min: 3, max: 5 }),
                level: match quest_role {
                    Some(QuestRewardRole::WandmakerWand) => None,
                    Some(
                        QuestRewardRole::BlacksmithWeapon { .. }
                        | QuestRewardRole::BlacksmithMissile { .. }
                        | QuestRewardRole::BlacksmithArmor { .. },
                    ) => None,
                    Some(QuestRewardRole::ImpRing) => None,
                    Some(_) => Some(item.level),
                    None if artifact_conditional => Some(item.level),
                    None => reported_level(item, constrained, shop_role),
                },
                level_range: if quest_role == Some(QuestRewardRole::WandmakerWand) {
                    Some(NumericRange { min: 1, max: 3 })
                } else if quest_role == Some(QuestRewardRole::ImpRing) {
                    Some(NumericRange { min: 2, max: 4 })
                } else {
                    blacksmith_smith_option.then_some(NumericRange { min: 0, max: 3 })
                },
                cursed: if matches!(
                    quest_role,
                    Some(
                        QuestRewardRole::BlacksmithRoomWeapon { .. }
                            | QuestRewardRole::BlacksmithRoomMissile { .. }
                            | QuestRewardRole::BlacksmithRoomArmor { .. }
                    )
                ) {
                    None
                } else {
                    Some(item.cursed)
                },
                enchantment: (!blacksmith_smith_option && potential_enchantment_is_reachable)
                    .then(|| item.potential_enchantment.clone())
                    .flatten()
                    .map(|enchantment_type| ItemEnchantment {
                        enchantment_type,
                        conditions: reachable_parchment_level
                            .map(|level| parchment_condition(self.depth as u32, level))
                            .into_iter()
                            .collect(),
                    }),
                prediction,
                spawn_conditions: item_conditions(artifact_conditional),
                conditions,
                notes: legacy_item_notes(
                    quest_role,
                    blacksmith_smith_option,
                    imp_shop_conditional,
                    reachable_parchment_level,
                    item.potential_enchantment.as_deref(),
                ),
                source: reported_source(item, exact_floor_one_room_prize),
            };
            if shop_role.is_some() {
                shop_items.push(entry);
            } else {
                items.push(entry);
            }
        }
        if has_shop {
            let shop_source = if self.depth == 20 {
                "ImpShopRoom"
            } else {
                "ShopRoom"
            };
            shop_items.push(ItemEntry {
                name: "inventory-dependent bag stock".into(),
                quantity: 1,
                class_name: None,
                candidate_classes: Vec::new(),
                category: "other".into(),
                tier: None,
                tier_range: None,
                level: None,
                level_range: None,
                cursed: None,
                enchantment: None,
                prediction: ItemPredictionKind::Constrained,
                spawn_conditions: Vec::new(),
                conditions: vec![
                    ItemCondition::Inventory {
                        requirement_id: "bag_stock".into(),
                    },
                    ItemCondition::Runtime {
                        state_id: "limited_drop_history".into(),
                    },
                ],
                notes: {
                    let mut notes = vec!["A bag may be offered; its presence and identity depend on inventory and prior limited drops.".into()];
                    if self.depth == 20 { notes.push("Appears only if the Ambitious Imp quest was completed before this shop is spawned.".into()); }
                    notes
                },
                source: Some(shop_source.into()),
            });
            shop_items.push(ItemEntry {
                name: "Hourglass sand stock".into(),
                quantity: 1,
                class_name: Some("SandBag".into()),
                candidate_classes: Vec::new(),
                category: "other".into(),
                tier: None,
                tier_range: None,
                level: None,
                level_range: None,
                cursed: None,
                enchantment: None,
                prediction: ItemPredictionKind::Constrained,
                spawn_conditions: vec![ItemSpawnCondition {
                    all_of: vec![ItemDependencyCondition::Artifact {
                        events: vec![ArtifactEvent {
                            before_depth: self.depth as u32,
                            action: ArtifactEventAction::Obtained {
                                artifact: ArtifactKind::TimekeepersHourglass,
                            },
                        }],
                    }],
                }],
                conditions: if self.depth == 20 {
                    vec![ItemCondition::Quest { quest_id: "ambitious_imp".into(), depth: Some(self.depth as u32) }]
                } else {
                    Vec::new()
                },
                notes: if self.depth == 20 { vec!["Appears only if the Ambitious Imp quest was completed before this shop is spawned.".into()] } else { Vec::new() },
                source: Some(shop_source.into()),
            });
        }
        // The pinned isolated shuffle order is not public: bag/sand list size
        // and artifact constructor/fallback RNG can change its permutation.
        shop_items.sort_by(|a, b| {
            (&a.name, &a.class_name, &a.category).cmp(&(&b.name, &b.class_name, &b.category))
        });
        items.extend(shop_items);
        // These are room-class contracts, not painter/map facts. Once the
        // room selection is known, a later runtime-sensitive paint callback
        // cannot make its guaranteed reward contract disappear.
        if !self.runtime_sensitive_rooms {
            for fact in &self.room_public_facts {
                items.extend(fact.entries().into_iter().filter(|entry| {
                    !self.placed_items.iter().any(|item| {
                        let concrete_floor_one_prize = is_exact_floor_one_room_prize(
                            self.depth,
                            self.runtime_sensitive_layout,
                            item,
                        );
                        if !concrete_floor_one_prize {
                            return false;
                        }
                        let Some(source) = reported_source(item, true) else {
                            return false;
                        };
                        let room = source.split(':').next().unwrap_or(&source);
                        let entry_room = entry
                            .source
                            .as_deref()
                            .map(|source| source.split(':').next().unwrap_or(source));
                        if Some(room) != entry_room {
                            return false;
                        }
                        entry.class_name.as_deref().map_or_else(
                            || {
                                entry.source.as_deref() == Some(source.as_str())
                                    || entry.name == "single room reward source"
                            },
                            |class_name| class_name == item.class_name,
                        )
                    })
                }));
            }
        }
        let mut items = merge_identical_items(items);
        for baseline in baseline_items {
            let already_projected = items.iter().any(|item| {
                item.class_name == baseline.class_name
                    && item.source == baseline.source
                    && item.level == baseline.level
                    && item.cursed == baseline.cursed
                    && item.enchantment == baseline.enchantment
            });
            if !already_projected {
                items.push(baseline);
            }
        }
        let exact_map = allow_map
            .then(|| self.layout_map.clone())
            .flatten()
            .filter(|_| !self.runtime_sensitive_layout);
        let assumed_map = (allow_map && self.runtime_sensitive_layout)
            .then(|| self.layout_map.clone())
            .flatten();
        let guaranteed_appearances =
            guaranteed_appearances(&self.rooms, !self.runtime_sensitive_rooms);
        let initial_encounters = encounters::initial_encounters(
            self.depth,
            self.runtime_sensitive_layout,
            self.map.as_ref(),
            &self.placed_items,
        );

        FloorReport {
            depth: self.depth as u32,
            feeling: (!self.runtime_sensitive_feeling).then(|| self.feeling.as_str().to_string()),
            builder: (!self.runtime_sensitive_rooms)
                .then(|| {
                    self.builder.map(|builder| match builder {
                        BuilderKind::Loop => "loop".to_string(),
                        BuilderKind::FigureEight => "figure_eight".to_string(),
                    })
                })
                .flatten(),
            rooms: if self.runtime_sensitive_rooms {
                Vec::new()
            } else {
                self.rooms.clone()
            },
            possible_rooms: Vec::new(),
            guaranteed_appearances,
            initial_encounters,
            items: group_item_entries(items),
            quests: self.quests[..self
                .runtime_sensitive_quests_from
                .unwrap_or(self.quests.len())]
                .to_vec(),
            // Public maps are painter-complete floor layouts, captured before
            // NPC, mob, and item population. Final entity maps remain internal.
            map: exact_map,
            assumed_map,
        }
    }
}

fn is_blacklisted(item: &GeneratedItem) -> bool {
    matches!(
        item.class_name.as_str(),
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

#[cfg(test)]
#[path = "state/tests.rs"]
mod tests;
