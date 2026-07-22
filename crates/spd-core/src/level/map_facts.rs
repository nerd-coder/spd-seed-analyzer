//! Structured renderer facts projected from room paint and `createItems`.

use crate::items::model::GeneratedItem;
use crate::level::create_items::CreatedLoot;
use crate::level::painter;
use crate::level::terrain::{self, TerrainMap};
use crate::random::Random;
use crate::report::{
    FloorMap, MapHeap, MapHeapItem, MapMarker, MapMarkerKind, MapMob, MapTransition, MapTrap,
};

pub(super) struct MapFacts {
    pub heaps: Vec<MapHeap>,
    pub mobs: Vec<MapMob>,
    pub markers: Vec<MapMarker>,
}

impl MapFacts {
    pub fn from_room_paint(map: &TerrainMap) -> Self {
        let mut heaps = Vec::new();
        let mut mobs = Vec::new();
        let mut markers = Vec::new();

        for (cell, heap) in map.known_heaps.iter().enumerate() {
            let Some(heap) = heap else { continue };
            if let Some(item) = heap.items.first() {
                markers.push(MapMarker {
                    cell: cell as u32,
                    kind: MapMarkerKind::Item,
                    label: item.title(),
                });
            }
            heaps.push(MapHeap {
                cell: cell as u32,
                heap_type: heap.heap_type.to_string(),
                items: heap.items.iter().map(heap_item).collect(),
            });
        }
        // Retain the legacy marker projection for room families whose item-to-cell
        // association is not yet modeled. Covered rooms (including Armory) use
        // the exact structured heap above and never fall back to this label.
        for (cell, &occupied) in map.heap_occupied.iter().enumerate() {
            if occupied && map.known_heaps[cell].is_none() {
                markers.push(MapMarker {
                    cell: cell as u32,
                    kind: MapMarkerKind::Item,
                    label: "Room loot".to_string(),
                });
            }
        }
        for (cell, &class_name) in map.known_mobs.iter().enumerate() {
            let Some(class_name) = class_name else {
                continue;
            };
            markers.push(MapMarker {
                cell: cell as u32,
                kind: MapMarkerKind::Mob,
                label: class_name.to_string(),
            });
            mobs.push(MapMob {
                cell: cell as u32,
                class_name: class_name.to_string(),
            });
        }

        Self {
            heaps,
            mobs,
            markers,
        }
    }

    pub fn add_created_loot(&mut self, created: &CreatedLoot, map_len: usize) {
        let Some(cell) = created.cell.filter(|&cell| cell < map_len) else {
            return;
        };
        let loot = &created.loot;
        match loot.heap_type {
            "mimic" | "golden_mimic" => {
                let class_name = if loot.heap_type == "mimic" {
                    "Mimic"
                } else {
                    "GoldenMimic"
                };
                self.mobs.push(MapMob {
                    cell: cell as u32,
                    class_name: class_name.to_string(),
                });
                self.markers.push(MapMarker {
                    cell: cell as u32,
                    kind: MapMarkerKind::Mob,
                    label: if class_name == "GoldenMimic" {
                        "Golden Mimic".to_string()
                    } else {
                        class_name.to_string()
                    },
                });
            }
            heap_type => {
                self.heaps.push(MapHeap {
                    cell: cell as u32,
                    heap_type: heap_type.to_string(),
                    items: vec![heap_item(&loot.item)],
                });
                self.markers.push(MapMarker {
                    cell: cell as u32,
                    kind: MapMarkerKind::Item,
                    label: loot.item.title(),
                });
            }
        }
    }

    fn sort(&mut self) {
        self.heaps.sort_by_key(|heap| heap.cell);
        self.mobs.sort_by(|left, right| {
            (left.cell, &left.class_name).cmp(&(right.cell, &right.class_name))
        });
        self.markers.sort_by_key(|marker| marker.cell);
    }

    pub fn into_floor_map(
        mut self,
        map: &TerrainMap,
        depth: i32,
        branch: i32,
        depth_seed: i64,
    ) -> FloorMap {
        self.sort();
        FloorMap {
            width: map.width as u32,
            height: map.height as u32,
            tileset: terrain::tileset_for_depth(depth).to_string(),
            tiles: map.map.iter().map(|&tile| tile as u16).collect(),
            tile_variance: tile_variance(map.len(), depth_seed),
            discoverable: discoverable(&map.map, map.width),
            markers: self.markers,
            heaps: self.heaps,
            mobs: self.mobs,
            transitions: transitions(map, depth, branch),
            traps: traps(map),
            plants: Vec::new(),
            blobs: Vec::new(),
        }
    }
}

fn heap_item(item: &GeneratedItem) -> MapHeapItem {
    MapHeapItem {
        class_name: item.class_name.clone(),
        quantity: item.quantity,
        level: item.level,
        cursed: item.cursed,
    }
}

fn transitions(map: &TerrainMap, depth: i32, branch: i32) -> Vec<MapTransition> {
    let mut transitions = Vec::new();
    for (cell, &tile) in map.map.iter().enumerate() {
        let (transition_type, dest_depth, dest_type) = match tile {
            terrain::EXIT => (
                "REGULAR_EXIT",
                depth + 1,
                Some("REGULAR_ENTRANCE".to_string()),
            ),
            terrain::ENTRANCE | terrain::ENTRANCE_SP if depth == 1 => ("SURFACE", 0, None),
            terrain::ENTRANCE | terrain::ENTRANCE_SP => (
                "REGULAR_ENTRANCE",
                depth - 1,
                Some("REGULAR_EXIT".to_string()),
            ),
            _ => continue,
        };
        let x = cell as u32 % map.width as u32;
        let y = cell as u32 / map.width as u32;
        transitions.push(MapTransition {
            cell: cell as u32,
            transition_type: transition_type.to_string(),
            left: x,
            top: y,
            right: x,
            bottom: y,
            dest_depth,
            dest_branch: branch,
            dest_type,
        });
    }
    transitions.sort_by(|left, right| {
        (left.cell, &left.transition_type).cmp(&(right.cell, &right.transition_type))
    });
    transitions
}

fn traps(map: &TerrainMap) -> Vec<MapTrap> {
    map.trap_names
        .iter()
        .enumerate()
        .filter_map(|(cell, &class_name)| {
            class_name.map(|class_name| {
                let metadata = painter::trap_metadata(class_name);
                MapTrap {
                    cell: cell as u32,
                    class_name: class_name.to_string(),
                    visible: map.map[cell] == terrain::TRAP,
                    active: true,
                    color: metadata.map(|metadata| metadata.color).unwrap_or_default(),
                    shape: metadata.map(|metadata| metadata.shape).unwrap_or_default(),
                }
            })
        })
        .collect()
}

/// Pinned `DungeonTileSheet.setupVariance`: a fresh seedCurDepth generator,
/// intentionally isolated from the active level-generation RNG.
pub(super) fn tile_variance(len: usize, depth_seed: i64) -> Vec<u8> {
    Random::push_generator_seeded(depth_seed);
    let variance = (0..len).map(|_| Random::int_max(100) as u8).collect();
    Random::pop_generator();
    variance
}

/// Pinned `Level.cleanWalls()`: a cell is discoverable when its linear
/// `PathFinder.NEIGHBOURS9` window contains a non-wall terrain cell.
fn discoverable(tiles: &[i32], width: i32) -> Vec<bool> {
    let offsets = [
        -width - 1,
        -width,
        -width + 1,
        -1,
        0,
        1,
        width - 1,
        width,
        width + 1,
    ];
    (0..tiles.len())
        .map(|cell| {
            offsets.iter().any(|offset| {
                let neighbour = cell as i32 + offset;
                neighbour >= 0
                    && (neighbour as usize) < tiles.len()
                    && !matches!(
                        tiles[neighbour as usize],
                        terrain::WALL | terrain::WALL_DECO
                    )
            })
        })
        .collect()
}
