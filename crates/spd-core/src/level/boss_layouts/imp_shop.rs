//! Pinned `ImpShopRoom.spawnShop` projection on the `CityBossLevel` map.
//!
//! The room's stock is generated during level creation, but the shopkeeper
//! and FOR_SALE heaps only appear once the Imp quest is completed and the
//! Dwarf King is defeated, so every published cell is marked uncertain.

use crate::items::model::GeneratedItem;
use crate::level::map_facts::heap_item;
use crate::level::mob_state;
use crate::report::{FloorMap, MapHeap, MapMob};
use spd_objects::MobId;

/// `impShop.set(end.left+3, end.top+12, end.left+11, end.top+20)`, inclusive.
const LEFT: i32 = 3;
const TOP: i32 = 12;
const RIGHT: i32 = 11;
const BOTTOM: i32 = 20;
/// `ImpShopRoom.center()` (odd extents, no jitter) holds the build pedestal.
const KEEPER: (i32, i32) = (7, 16);
/// `ImpShopRoom.entrance()` without connections: `((left+right)/2 + 1, bottom-1)`.
const ENTRANCE: (i32, i32) = ((LEFT + RIGHT) / 2 + 1, BOTTOM - 1);

pub(crate) fn place(floor: &mut FloorMap, stock: &[GeneratedItem]) {
    let width = floor.width as i32;
    let cell = |(x, y): (i32, i32)| (x + y * width) as u32;
    // `spawnShop` grows the room downward while it cannot hold the stock plus
    // four sandbags and the shopkeeper.
    let mut bottom = BOTTOM;
    while stock.len() as i32 + 5 >= 7 * (bottom - TOP - 1) {
        bottom += 1;
    }

    let keeper = cell(KEEPER);
    floor.mobs.push(MapMob {
        cell: keeper,
        mob: MobId::ImpShopkeeper,
        state: mob_state::initial_state("ImpShopkeeper"),
        items: Vec::new(),
    });
    floor.uncertain_cells.push(keeper);

    for (point, item) in clockwise_cells(bottom).zip(stock) {
        let cell = cell(point);
        floor.heaps.push(MapHeap {
            cell,
            heap_type: "for_sale".into(),
            items: vec![heap_item(item)],
        });
        floor.uncertain_cells.push(cell);
    }
    floor.uncertain_cells.sort_unstable();
    floor.uncertain_cells.dedup();
}

/// `ShopRoom.placeItems` clockwise rings, starting beside the entrance.
fn clockwise_cells(bottom: i32) -> impl Iterator<Item = (i32, i32)> {
    let (width, height) = (RIGHT - LEFT + 1, bottom - TOP + 1);
    let mut entry = (ENTRANCE.0 - 1, ENTRANCE.1);
    let mut current = entry;
    let mut inset = 1;
    std::iter::from_fn(move || {
        step(&mut current, inset, bottom);
        if current == entry {
            if entry.1 == TOP + inset {
                entry.1 += 1;
            } else if entry.1 == bottom - inset {
                entry.1 -= 1;
            }
            if entry.0 == LEFT + inset {
                entry.0 += 1;
            } else if entry.0 == RIGHT - inset {
                entry.0 -= 1;
            }
            inset += 1;
            if inset > (width.min(height) - 3) / 2 {
                return None;
            }
            current = entry;
            step(&mut current, inset, bottom);
        }
        Some(current)
    })
}

fn step(point: &mut (i32, i32), inset: i32, bottom: i32) {
    if point.0 == LEFT + inset && point.1 != TOP + inset {
        point.1 -= 1;
    } else if point.1 == TOP + inset && point.0 != RIGHT - inset {
        point.0 += 1;
    } else if point.0 == RIGHT - inset && point.1 != bottom - inset {
        point.1 += 1;
    } else {
        point.0 -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imp_shop_fills_rings_from_the_entrance_inset() {
        let cells: Vec<_> = clockwise_cells(BOTTOM).collect();
        assert_eq!(&cells[..3], &[(6, 19), (5, 19), (4, 19)]);
        assert_eq!(cells[3], (4, 18));
        // Each ring skips its own entry inset.
        assert_eq!(cells.len(), 23 + 15 + 7);
        assert!(!cells.contains(&KEEPER));
    }
}
