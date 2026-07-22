//! Internal per-floor state and its public report projection.

use crate::items::model::GeneratedItem;
use crate::report::{FloorMap, FloorReport, ItemEntry};
use crate::rooms::init_rooms::BuilderKind;

use super::Feeling;

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

impl LevelState {
    pub fn to_floor_report(&self) -> FloorReport {
        let mut items = Vec::new();
        for item in self.forced_items.iter().chain(&self.placed_items) {
            if is_blacklisted(item) {
                continue;
            }
            let full_title = item.title();
            let name = if item.cursed {
                full_title
                    .strip_prefix("cursed ")
                    .unwrap_or(&full_title)
                    .to_string()
            } else {
                full_title
            };
            items.push(ItemEntry {
                name,
                class_name: Some(item.class_name.clone()),
                category: format!("{:?}", item.category).to_ascii_lowercase(),
                cursed: item.cursed,
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
            map: self.map.clone(),
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
