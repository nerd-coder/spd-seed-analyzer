//! Public map sanitization for runtime-sensitive item facts.

use crate::items::model::{GeneratedItem, ShopStockRole};
use crate::report::{FloorMap, MapMarkerKind};

pub(super) fn reported_level(
    item: &GeneratedItem,
    constrained: bool,
    shop_role: Option<ShopStockRole>,
) -> Option<i32> {
    if shop_role == Some(ShopStockRole::DeckRareArtifactOrRing) {
        // Pinned case 2 does not call level(0). If the artifact deck is
        // exhausted, Generator falls back to a Ring with its randomized level.
        None
    } else if constrained && shop_role.is_some() {
        Some(0)
    } else {
        (!constrained).then_some(item.level)
    }
}

pub(super) fn sanitize_public_map(mut map: FloorMap) -> FloorMap {
    let runtime_sensitive_cells = map.runtime_sensitive_loot_cells.clone();
    map.heaps
        .retain(|heap| !runtime_sensitive_cells.contains(&heap.cell));
    map.mobs
        .retain(|mob| !runtime_sensitive_cells.contains(&mob.cell));
    map.markers
        .retain(|marker| !runtime_sensitive_cells.contains(&marker.cell));
    map.runtime_sensitive_loot_cells.clear();

    let constrained_cells = map.constrained_equipment_cells.clone();
    for heap in &mut map.heaps {
        if constrained_cells.contains(&heap.cell) {
            heap.items.clear();
        }
    }
    for marker in &mut map.markers {
        if marker.kind == MapMarkerKind::Item && constrained_cells.contains(&marker.cell) {
            marker.label = "Blacksmith room equipment".into();
        }
    }
    map.constrained_equipment_cells.clear();

    let for_sale_cells: Vec<_> = map
        .heaps
        .iter()
        .filter(|heap| heap.heap_type == "for_sale")
        .map(|heap| heap.cell)
        .collect();
    map.heaps.retain(|heap| heap.heap_type != "for_sale");
    map.markers
        .retain(|marker| !for_sale_cells.contains(&marker.cell));

    let mut sacrificial_cells = Vec::new();
    for heap in &mut map.heaps {
        if heap.heap_type == "sacrificial" {
            // The blob-held reward is runtime-history-sensitive. The public
            // item list carries its stable constraints.
            heap.items.clear();
            sacrificial_cells.push(heap.cell);
        }
    }
    for marker in &mut map.markers {
        if marker.kind == MapMarkerKind::Item && sacrificial_cells.contains(&marker.cell) {
            marker.label = "Sacrifice reward".to_string();
        }
    }
    map
}
