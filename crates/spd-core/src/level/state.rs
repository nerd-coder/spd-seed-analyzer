//! Internal per-floor state and its public report projection.

use crate::items::model::{GeneratedItem, ItemProvenance, QuestRewardRole, ShopStockRole};
use crate::report::{FloorMap, FloorReport, ItemEntry, ItemPredictionKind};
use crate::rooms::init_rooms::BuilderKind;

use super::Feeling;

#[path = "state/forced_queue.rs"]
mod forced_queue;
use forced_queue::public_entries as forced_public_entries;

#[path = "state_map.rs"]
mod state_map;
use state_map::reported_level;

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
            | QuestRewardRole::BlacksmithRoomWeapon { .. }
            | QuestRewardRole::BlacksmithRoomMissile { .. },
        ) => ItemPredictionKind::Constrained,
        ItemProvenance::Quest(
            QuestRewardRole::GhostArmor { .. }
            | QuestRewardRole::BlacksmithWeapon { .. }
            | QuestRewardRole::BlacksmithMissile { .. }
            | QuestRewardRole::BlacksmithArmor { .. }
            | QuestRewardRole::BlacksmithRoomArmor { .. },
        ) => ItemPredictionKind::Exact,
        ItemProvenance::Room(_) | ItemProvenance::Forced(_) => ItemPredictionKind::Constrained,
        _ => ItemPredictionKind::Exact,
    }
}

pub(super) fn is_runtime_sensitive_main_loot(item: &GeneratedItem) -> bool {
    is_runtime_sensitive_loot_source(item.source.as_deref())
}

pub(super) fn is_runtime_sensitive_loot_source(source: Option<&str>) -> bool {
    source
        .and_then(|source| source.rsplit(':').next())
        .is_some_and(|origin| matches!(origin, "heap" | "mimic" | "golden_mimic"))
}

#[derive(Debug, Clone)]
pub struct LevelState {
    pub depth: i32,
    pub feeling: Feeling,
    pub builder: Option<BuilderKind>,
    pub rooms: Vec<String>,
    #[doc(hidden)]
    pub room_bounds: Vec<LevelRoomFact>,
    pub build_ok: bool,
    pub forced_items: Vec<GeneratedItem>,
    /// Exact initial queue snapshot before room callbacks consume/reposition it.
    /// Internal parity evidence only; public output uses static queue contracts.
    #[doc(hidden)]
    pub initial_forced_items: Vec<GeneratedItem>,
    pub placed_items: Vec<GeneratedItem>,
    /// First placed-item index generated after a runtime-sensitive shop rare
    /// artifact call. Internal facts remain exact; public projection omits the
    /// tail because its RNG path can differ with artifact exhaustion/history.
    #[doc(hidden)]
    pub runtime_sensitive_placed_items_from: Option<usize>,
    /// First quest summary selected after the runtime-sensitive shop callback.
    #[doc(hidden)]
    pub runtime_sensitive_quests_from: Option<usize>,
    pub quests: Vec<String>,
    /// Public-safe label for reward-bearing quest summaries. `None` means the
    /// exact internal summary is safe to expose.
    #[doc(hidden)]
    pub quest_public_labels: Vec<Option<String>>,
    #[doc(hidden)]
    pub runtime_sensitive_map: bool,
    /// Builder and room metadata can depend on a pre-build player-state callback.
    #[doc(hidden)]
    pub runtime_sensitive_layout: bool,
    /// The baseline feeling can be overridden by held trinkets before build.
    #[doc(hidden)]
    pub runtime_sensitive_feeling: bool,
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
        let mut items = forced_public_entries(self.depth, &self.initial_forced_items);
        let mut shop_items = Vec::new();
        let mut has_shop = false;
        for (index, item) in self.placed_items.iter().enumerate() {
            if self
                .runtime_sensitive_placed_items_from
                .is_some_and(|boundary| index >= boundary)
            {
                continue;
            }
            if is_blacklisted(item) || is_runtime_sensitive_main_loot(item) {
                continue;
            }
            // Sacrifice is represented only by its static room contract. The
            // sampled weapon remains internal for Java parity.
            if item.source.as_deref() == Some("SacrificeRoom") {
                continue;
            }
            if item.provenance == ItemProvenance::Quest(QuestRewardRole::WandmakerPersisted) {
                continue;
            }
            // Room contracts are emitted independently of the sampled exact
            // items so runtime-sensitive count/control flow cannot leak.
            if matches!(item.provenance, ItemProvenance::Room(_)) {
                continue;
            }
            let prediction = prediction_kind(item);
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
            let exact_name = if item.cursed {
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
                        QuestRewardRole::GhostWeapon { tier }
                        | QuestRewardRole::GhostArmor { tier },
                    ),
                ) => Some(tier),
                (
                    _,
                    Some(
                        QuestRewardRole::BlacksmithWeapon { tier }
                        | QuestRewardRole::BlacksmithMissile { tier }
                        | QuestRewardRole::BlacksmithArmor { tier },
                    ),
                ) => Some(tier),
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
                (Some(ShopStockRole::DeckRareArtifactOrRing), _) => "artifact or ring stock",
                (_, Some(QuestRewardRole::GhostWeapon { .. })) => "Ghost weapon reward",
                (_, Some(QuestRewardRole::GhostArmor { .. })) => "Ghost armor reward",
                (_, Some(QuestRewardRole::WandmakerWand)) => "Wandmaker wand reward",
                (_, Some(QuestRewardRole::BlacksmithWeapon { .. })) => "Blacksmith weapon option",
                (_, Some(QuestRewardRole::BlacksmithMissile { .. })) => "Blacksmith missile option",
                (_, Some(QuestRewardRole::BlacksmithArmor { .. })) => "Blacksmith armor option",
                (_, Some(QuestRewardRole::BlacksmithRoomWeapon { .. })) => "Blacksmith room weapon",
                (_, Some(QuestRewardRole::BlacksmithRoomMissile { .. })) => {
                    "Blacksmith room missile weapon"
                }
                (_, Some(QuestRewardRole::BlacksmithRoomArmor { .. })) => "Blacksmith room armor",
                (_, Some(QuestRewardRole::ImpRing)) => "Imp ring reward",
                _ => "weapon reward",
            };
            let entry = ItemEntry {
                name: if constrained {
                    constrained_name.to_string()
                } else {
                    exact_name
                },
                class_name: (!constrained).then(|| item.class_name.clone()),
                category: if shop_role == Some(ShopStockRole::DeckRareArtifactOrRing) {
                    "other".into()
                } else {
                    format!("{:?}", item.category).to_ascii_lowercase()
                },
                tier,
                tier_range: None,
                level: match quest_role {
                    Some(QuestRewardRole::WandmakerWand) => None,
                    Some(_) => Some(item.level),
                    None => reported_level(item, constrained, shop_role),
                },
                level_range: None,
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
                prediction,
                conditional_notes: if matches!(
                    quest_role,
                    Some(
                        QuestRewardRole::GhostWeapon { .. }
                            | QuestRewardRole::GhostArmor { .. }
                            | QuestRewardRole::BlacksmithWeapon { .. }
                            | QuestRewardRole::BlacksmithMissile { .. }
                            | QuestRewardRole::BlacksmithArmor { .. }
                            | QuestRewardRole::BlacksmithRoomWeapon { .. }
                            | QuestRewardRole::BlacksmithRoomMissile { .. }
                            | QuestRewardRole::BlacksmithRoomArmor { .. }
                    )
                ) {
                    vec![
                        "Parchment Scrap may alter the reward's enchantment or glyph chance."
                            .into(),
                    ]
                } else if quest_role == Some(QuestRewardRole::WandmakerWand) {
                    vec!["The two reward wands are distinct, uncursed, and each receives one upgrade; concrete identities and levels depend on prior wand history.".into()]
                } else if quest_role == Some(QuestRewardRole::ImpRing) {
                    vec!["The concrete ring identity depends on prior ring history; the reported level is stable after two upgrades and the reward is forced cursed.".into()]
                } else {
                    Vec::new()
                },
                source: item.source.clone(),
            };
            if shop_role.is_some() {
                shop_items.push(entry);
            } else {
                items.push(entry);
            }
        }
        if has_shop {
            shop_items.push(ItemEntry {
                name: "inventory-dependent bag stock".into(),
                class_name: None,
                category: "other".into(),
                tier: None,
                tier_range: None,
                level: None,
                level_range: None,
                cursed: None,
                prediction: ItemPredictionKind::Constrained,
                conditional_notes: vec![
                    "A bag may be offered; its presence and identity depend on inventory and prior limited drops.".into(),
                ],
                source: Some("ShopRoom".into()),
            });
            shop_items.push(ItemEntry {
                name: "Hourglass sand stock".into(),
                class_name: None,
                category: "other".into(),
                tier: None,
                tier_range: None,
                level: None,
                level_range: None,
                cursed: None,
                prediction: ItemPredictionKind::Constrained,
                conditional_notes: vec![
                    "Sandbags may be offered depending on the hero's Timekeeper's Hourglass state; presence and quantity are not asserted.".into(),
                ],
                source: Some("ShopRoom".into()),
            });
        }
        // The pinned isolated shuffle order is not public: bag/sand list size
        // and artifact constructor/fallback RNG can change its permutation.
        shop_items.sort_by(|a, b| {
            (&a.name, &a.class_name, &a.category).cmp(&(&b.name, &b.class_name, &b.category))
        });
        items.extend(shop_items);
        if !self.runtime_sensitive_layout {
            for fact in &self.room_public_facts {
                items.extend(fact.entries());
            }
        }
        FloorReport {
            depth: self.depth as u32,
            feeling: (!self.runtime_sensitive_feeling).then(|| self.feeling.as_str().to_string()),
            builder: (!self.runtime_sensitive_layout)
                .then(|| {
                    self.builder.map(|builder| match builder {
                        BuilderKind::Loop => "loop".to_string(),
                        BuilderKind::FigureEight => "figure_eight".to_string(),
                    })
                })
                .flatten(),
            rooms: if self.runtime_sensitive_layout {
                Vec::new()
            } else {
                self.rooms.clone()
            },
            items,
            quests: self.quests[..self
                .runtime_sensitive_quests_from
                .unwrap_or(self.quests.len())]
                .iter()
                .enumerate()
                .map(|(index, exact)| {
                    self.quest_public_labels
                        .get(index)
                        .and_then(|label| label.as_deref())
                        .unwrap_or(exact)
                        .to_string()
                })
                .collect(),
            // Public maps are painter-complete floor layouts, captured before
            // NPC, mob, and item population. Final entity maps remain internal.
            map: allow_map
                .then(|| self.layout_map.clone())
                .flatten()
                .filter(|_| !self.runtime_sensitive_layout),
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
