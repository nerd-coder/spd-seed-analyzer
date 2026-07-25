//! Internal per-floor state and its public report projection.

use crate::items::model::{GeneratedItem, ItemProvenance, ShopStockRole};
use crate::report::{FloorMap, FloorReport, ItemEntry, ItemPredictionKind};
use crate::rooms::init_rooms::BuilderKind;

use super::Feeling;

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
        _ => match item.source.as_deref() {
            // The exact weapon depends on persistent generator state advanced by
            // runtime/player history before the room is painted.
            Some("SacrificeRoom") => ItemPredictionKind::Constrained,
            _ => ItemPredictionKind::Exact,
        },
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
    pub complete: bool,
    pub map: Option<FloorMap>,
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
        let mut items = Vec::new();
        let mut shop_items = Vec::new();
        let mut has_shop = false;
        for (index, item) in self.forced_items.iter().map(|item| (None, item)).chain(
            self.placed_items
                .iter()
                .enumerate()
                .map(|(index, item)| (Some(index), item)),
        ) {
            if index.is_some_and(|index| {
                self.runtime_sensitive_placed_items_from
                    .is_some_and(|boundary| index >= boundary)
            }) {
                continue;
            }
            if is_blacklisted(item) || is_runtime_sensitive_main_loot(item) {
                continue;
            }
            let prediction = prediction_kind(item);
            let shop_role = match item.provenance {
                ItemProvenance::Shop(role) => {
                    has_shop = true;
                    Some(role)
                }
                ItemProvenance::None => None,
            };
            // Bag presence and identity depend on inventory/limited-drop
            // history. A single conditional constraint is added below.
            if shop_role == Some(ShopStockRole::ChooseBag) {
                continue;
            }
            let constrained = prediction == ItemPredictionKind::Constrained;
            let full_title = item.title();
            let exact_name = if item.cursed {
                full_title
                    .strip_prefix("cursed ")
                    .unwrap_or(&full_title)
                    .to_string()
            } else {
                full_title
            };
            let tier = match shop_role {
                Some(ShopStockRole::DeckWeapon { tier } | ShopStockRole::DeckMissile { tier }) => {
                    Some(tier)
                }
                _ if constrained => crate::generator::weapon_tier_for_class(&item.class_name),
                _ => None,
            };
            let constrained_name = match shop_role {
                Some(ShopStockRole::DeckWeapon { .. }) => "weapon stock",
                Some(ShopStockRole::DeckMissile { .. }) => "missile weapon stock",
                Some(ShopStockRole::DeckRareWand) => "wand stock",
                Some(ShopStockRole::DeckRareRing) => "ring stock",
                Some(ShopStockRole::DeckRareArtifactOrRing) => "artifact or ring stock",
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
                level: reported_level(item, constrained, shop_role),
                cursed: Some(item.cursed),
                prediction,
                conditional_notes: if item.source.as_deref() == Some("SacrificeRoom") {
                    vec!["Parchment Scrap may alter the weapon's enchantment chance.".into()]
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
                level: None,
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
                level: None,
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
        FloorReport {
            depth: self.depth as u32,
            feeling: Some(self.feeling.as_str().to_string()),
            builder: self.builder.map(|builder| match builder {
                BuilderKind::Loop => "loop".to_string(),
                BuilderKind::FigureEight => "figure_eight".to_string(),
            }),
            rooms: self.rooms.clone(),
            items,
            quests: self.runtime_sensitive_quests_from.map_or_else(
                || self.quests.clone(),
                |boundary| self.quests[..boundary].to_vec(),
            ),
            map: if self.runtime_sensitive_placed_items_from.is_some() {
                None
            } else {
                self.map.clone().map(state_map::sanitize_public_map)
            },
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
