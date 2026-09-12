//! MineEntrance transition paint (`ENTRANCE_SP` plus `EMPTY_SP` neighbours).

use crate::level::terrain::{TerrainMap, EMPTY_SP, ENTRANCE_SP, WALL};
use crate::rooms::room::Room;

pub(super) fn paint(map: &mut TerrainMap, room: &Room, depth: i32) {
    let entrance = loop {
        let point = room.random_margin(3);
        let cell = map.point_to_cell(point.x, point.y).unwrap();
        if map.mob_occupied[cell] {
            continue;
        }
        let w = map.width as isize;
        let valid = [-w - 1, -w, -w + 1, -1, 0, 1, w - 1, w, w + 1]
            .iter()
            .any(|offset| map.map[(cell as isize + offset) as usize] != WALL);
        if valid || (room.height() == 7 && room.width() == 7) {
            break point;
        }
    };
    let cell = map.point_to_cell(entrance.x, entrance.y).unwrap();
    map.map[cell] = ENTRANCE_SP;
    let w = map.width as isize;
    for offset in [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1] {
        map.map[(cell as isize + offset) as usize] = EMPTY_SP;
    }
    map.branch_entrances.push(cell);
    map.record_custom_tile(
        "QuestExit",
        "environment/custom_tiles/caves_quest.png",
        (entrance.x - 1, entrance.y - 1, 3, 3),
        vec![8, 9, 10, 16, 17, 18, 24, 25, 26],
    );
    let _ = depth;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::terrain;
    use crate::random::Random;
    use crate::rooms::types::RoomKind;

    #[test]
    fn paints_entrance_sp_and_empty_sp_neighbours() {
        Random::push_generator_seeded(1);
        let mut room = Room::new(0, "MineEntrance", RoomKind::Entrance, 1, 16, 7, 10, 7, 10);
        room.left = 1;
        room.top = 1;
        room.right = 7;
        room.bottom = 7;
        let mut map = terrain::paint_minimal(&[room.clone()]).unwrap();
        paint(&mut map, &room, 12);
        Random::pop_generator();

        assert_eq!(map.branch_entrances, vec![map.point_to_cell(4, 4).unwrap()]);
        let cell = map.branch_entrances[0];
        assert_eq!(map.map[cell], ENTRANCE_SP);
        let w = map.width as isize;
        for offset in [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1] {
            assert_eq!(map.map[(cell as isize + offset) as usize], EMPTY_SP);
        }
    }
}
