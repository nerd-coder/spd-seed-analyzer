//! Internal per-floor state and its public report projection.

use crate::items::model::GeneratedItem;
use crate::report::{FloorMap, FloorReport, ItemEntry, ItemPredictionKind};
use crate::rooms::init_rooms::BuilderKind;

use super::Feeling;

fn prediction_kind(item: &GeneratedItem) -> ItemPredictionKind {
    match item.source.as_deref() {
        // The exact weapon depends on persistent generator state advanced by
        // runtime/player history before the room is painted.
        Some("SacrificeRoom") => ItemPredictionKind::Constrained,
        _ => ItemPredictionKind::Exact,
    }
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
        for item in self.forced_items.iter().chain(&self.placed_items) {
            if is_blacklisted(item) {
                continue;
            }
            let prediction = prediction_kind(item);
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
            let tier = constrained
                .then(|| crate::generator::weapon_tier_for_class(&item.class_name))
                .flatten();
            items.push(ItemEntry {
                name: if constrained {
                    "weapon reward".to_string()
                } else {
                    exact_name
                },
                class_name: (!constrained).then(|| item.class_name.clone()),
                category: format!("{:?}", item.category).to_ascii_lowercase(),
                tier,
                level: (!constrained).then_some(item.level),
                cursed: Some(item.cursed),
                prediction,
                conditional_notes: if constrained {
                    vec!["Parchment Scrap may alter the weapon's enchantment chance.".into()]
                } else {
                    Vec::new()
                },
                source: item.source.clone(),
            });
        }
        FloorReport {
            depth: self.depth as u32,
            feeling: Some(self.feeling.as_str().to_string()),
            builder: self.builder.map(|builder| match builder {
                BuilderKind::Loop => "loop".to_string(),
                BuilderKind::FigureEight => "figure_eight".to_string(),
            }),
            rooms: self.rooms.clone(),
            items,
            quests: self.quests.clone(),
            map: self.map.clone().map(|mut map| {
                let mut sacrificial_cells = Vec::new();
                for heap in &mut map.heaps {
                    if heap.heap_type == "sacrificial" {
                        // The blob-held reward is runtime-history-sensitive. The
                        // public item list carries its stable constraints.
                        heap.items.clear();
                        sacrificial_cells.push(heap.cell);
                    }
                }
                for marker in &mut map.markers {
                    if marker.kind == crate::report::MapMarkerKind::Item
                        && sacrificial_cells.contains(&marker.cell)
                    {
                        marker.label = "Sacrifice reward".to_string();
                    }
                }
                map
            }),
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
