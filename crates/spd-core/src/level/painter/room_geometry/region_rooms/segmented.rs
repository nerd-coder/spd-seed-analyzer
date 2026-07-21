//! Prison `SegmentedRoom` recursive wall layout.

use crate::geom::Rect;
use crate::level::terrain::{TerrainMap, EMPTY, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::super::DoorMap;
use super::{door_points, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, EMPTY);

    // SPD clears door cells before recursion so its wall-adjacency checks see
    // openings at external connections.
    for door in door_points(room, room_index, doors) {
        set(map, door.x, door.y, EMPTY);
    }
    create_walls(
        map,
        Rect {
            left: room.left + 1,
            top: room.top + 1,
            right: room.right - 1,
            bottom: room.bottom - 1,
        },
    );
}

fn create_walls(map: &mut TerrainMap, area: Rect) {
    if (area.raw_width() + 1).max(area.raw_height() + 1) < 5
        || (area.raw_width() + 1).min(area.raw_height() + 1) < 3
    {
        return;
    }

    let split_vertical = area.raw_width() > area.raw_height()
        || (area.raw_width() == area.raw_height() && Random::int_max(2) == 0);
    let mut tries = 10;
    if split_vertical {
        loop {
            let split_x = Random::int_range_inclusive(area.left + 2, area.right - 2);
            if terrain_at(map, split_x, area.top - 1) == Some(WALL)
                && terrain_at(map, split_x, area.bottom + 1) == Some(WALL)
            {
                fill_rect(map, split_x, area.top, split_x, area.bottom, WALL);
                let space_top = Random::int_range_inclusive(area.top, area.bottom - 1);
                set(map, split_x, space_top, EMPTY);
                set(map, split_x, space_top + 1, EMPTY);
                create_walls(
                    map,
                    Rect {
                        left: area.left,
                        top: area.top,
                        right: split_x - 1,
                        bottom: area.bottom,
                    },
                );
                create_walls(
                    map,
                    Rect {
                        left: split_x + 1,
                        top: area.top,
                        right: area.right,
                        bottom: area.bottom,
                    },
                );
                break;
            }
            tries -= 1;
            if tries == 0 {
                break;
            }
        }
    } else {
        loop {
            let split_y = Random::int_range_inclusive(area.top + 2, area.bottom - 2);
            if terrain_at(map, area.left - 1, split_y) == Some(WALL)
                && terrain_at(map, area.right + 1, split_y) == Some(WALL)
            {
                fill_rect(map, area.left, split_y, area.right, split_y, WALL);
                let space_left = Random::int_range_inclusive(area.left, area.right - 1);
                set(map, space_left, split_y, EMPTY);
                set(map, space_left + 1, split_y, EMPTY);
                create_walls(
                    map,
                    Rect {
                        left: area.left,
                        top: area.top,
                        right: area.right,
                        bottom: split_y - 1,
                    },
                );
                create_walls(
                    map,
                    Rect {
                        left: area.left,
                        top: split_y + 1,
                        right: area.right,
                        bottom: area.bottom,
                    },
                );
                break;
            }
            tries -= 1;
            if tries == 0 {
                break;
            }
        }
    }
}
