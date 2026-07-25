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
            if is_blacklisted(item) || is_runtime_sensitive_main_loot(item) {
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
                let runtime_sensitive_cells = map.runtime_sensitive_loot_cells.clone();
                map.heaps
                    .retain(|heap| !runtime_sensitive_cells.contains(&heap.cell));
                map.mobs
                    .retain(|mob| !runtime_sensitive_cells.contains(&mob.cell));
                map.markers
                    .retain(|marker| !runtime_sensitive_cells.contains(&marker.cell));
                map.runtime_sensitive_loot_cells.clear();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::model::ItemCategory;
    use crate::report::{MapHeap, MapHeapItem, MapMarker, MapMarkerKind, MapMob};

    fn item(class_name: &str, source: &str) -> GeneratedItem {
        let mut item = GeneratedItem::new(class_name, ItemCategory::Potion);
        item.source = Some(source.into());
        item
    }

    #[test]
    fn public_projection_omits_runtime_sensitive_main_loot_and_map_cells() {
        let runtime_cells = vec![7, 9];
        let floor = LevelState {
            depth: 3,
            feeling: Feeling::None,
            builder: None,
            rooms: vec![],
            room_bounds: vec![],
            build_ok: true,
            forced_items: vec![item("PotionOfStrength", "forced")],
            placed_items: vec![
                item("PotionOfHealing", "chest:heap"),
                item("PotionOfMindVision", "mimic:mimic"),
                item("PotionOfHaste", "golden_mimic:golden_mimic"),
            ],
            quests: vec![],
            complete: true,
            map: Some(FloorMap {
                width: 4,
                height: 4,
                tileset: "sewers".into(),
                tiles: vec![0; 16],
                tile_variance: vec![0; 16],
                discoverable: vec![true; 16],
                markers: vec![
                    MapMarker {
                        cell: 7,
                        kind: MapMarkerKind::Item,
                        label: "Potion of Healing".into(),
                    },
                    MapMarker {
                        cell: 9,
                        kind: MapMarkerKind::Mob,
                        label: "Mimic".into(),
                    },
                    MapMarker {
                        cell: 12,
                        kind: MapMarkerKind::Item,
                        label: "Potion of Strength".into(),
                    },
                ],
                heaps: vec![
                    MapHeap {
                        cell: 7,
                        heap_type: "chest".into(),
                        items: vec![MapHeapItem {
                            class_name: "PotionOfHealing".into(),
                            quantity: 1,
                            level: 0,
                            cursed: false,
                        }],
                    },
                    MapHeap {
                        cell: 12,
                        heap_type: "heap".into(),
                        items: vec![MapHeapItem {
                            class_name: "PotionOfStrength".into(),
                            quantity: 1,
                            level: 0,
                            cursed: false,
                        }],
                    },
                ],
                mobs: vec![MapMob {
                    cell: 9,
                    class_name: "Mimic".into(),
                }],
                transitions: vec![],
                traps: vec![],
                plants: vec![],
                blobs: vec![],
                runtime_sensitive_loot_cells: runtime_cells,
            }),
            pre_items_rng_probe: vec![],
            pre_mobs_rng_probe: vec![],
            pre_paint_rng_probe: vec![],
        };

        let report = floor.to_floor_report();
        assert_eq!(report.items.len(), 1);
        assert_eq!(
            report.items[0].class_name.as_deref(),
            Some("PotionOfStrength")
        );
        let map = report.map.expect("map");
        assert_eq!(
            map.heaps.iter().map(|heap| heap.cell).collect::<Vec<_>>(),
            [12]
        );
        assert!(map.mobs.is_empty());
        assert_eq!(
            map.markers
                .iter()
                .map(|marker| marker.cell)
                .collect::<Vec<_>>(),
            [12]
        );
        assert!(map.runtime_sensitive_loot_cells.is_empty());

        let json = serde_json::to_string(&map).expect("serialize public map");
        assert!(!json.contains("PotionOfHealing"));
        assert!(!json.contains("Mimic"));
        assert!(!json.contains("runtime_sensitive_loot_cells"));
    }

    #[test]
    fn main_loot_classification_uses_source_provenance() {
        assert!(is_runtime_sensitive_main_loot(&item("Anything", "heap")));
        assert!(is_runtime_sensitive_main_loot(&item(
            "Anything",
            "locked_chest:heap"
        )));
        assert!(is_runtime_sensitive_main_loot(&item(
            "Anything",
            "mimic:mimic"
        )));
        assert!(!is_runtime_sensitive_main_loot(&item("Mimic", "forced")));
    }
}
