//! City `SegmentedLibraryRoom` recursive bookshelf layout.

use crate::geom::Rect;
use crate::level::terrain::{TerrainMap, BOOKSHELF, EMPTY_SP, WALL};
use crate::random::Random;
use crate::rooms::room::Room;

use super::super::super::DoorMap;
use super::{door_points, draw_inside, fill_margin, fill_rect, fill_room, set, terrain_at};

pub(super) fn paint(map: &mut TerrainMap, room: &Room, room_index: usize, doors: &DoorMap) {
    fill_room(map, room, WALL);
    fill_margin(map, room, 1, BOOKSHELF);
    fill_margin(map, room, 2, EMPTY_SP);
    for door in door_points(room, room_index, doors) {
        draw_inside(map, room, door, 2, EMPTY_SP);
    }
    create_walls(
        map,
        Rect {
            left: room.left + 2,
            top: room.top + 2,
            right: room.right - 2,
            bottom: room.bottom - 2,
        },
    );
}

fn create_walls(map: &mut TerrainMap, area: Rect) {
    if (area.raw_width() + 1).max(area.raw_height() + 1) < 4
        || (area.raw_width() + 1).min(area.raw_height() + 1) < 3
    {
        return;
    }

    let vertical = area.raw_width() > area.raw_height()
        || (area.raw_width() == area.raw_height() && Random::int_max(2) == 0);
    let mut tries = 10;
    if vertical {
        loop {
            let split_x = Random::int_range_inclusive(area.left + 2, area.right - 2);
            if terrain_at(map, split_x, area.top - 1) == Some(BOOKSHELF)
                && terrain_at(map, split_x, area.bottom + 1) == Some(BOOKSHELF)
            {
                fill_rect(map, split_x, area.top, split_x, area.bottom, BOOKSHELF);
                let space_top = Random::int_range_inclusive(area.top, area.bottom - 1);
                set(map, split_x, space_top, EMPTY_SP);
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
            if terrain_at(map, area.left - 1, split_y) == Some(BOOKSHELF)
                && terrain_at(map, area.right + 1, split_y) == Some(BOOKSHELF)
            {
                fill_rect(map, area.left, split_y, area.right, split_y, BOOKSHELF);
                let space_left = Random::int_range_inclusive(area.left, area.right - 1);
                set(map, space_left, split_y, EMPTY_SP);
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
