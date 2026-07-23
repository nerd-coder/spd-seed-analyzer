//! Exact `SecretSummoningRoom.paint` trap field and center heap cell.

use crate::level::terrain::{TerrainMap, SECRET_TRAP, WALL};
use crate::rooms::room::Room;

pub(super) fn paint(map: &mut TerrainMap, room: &Room) -> usize {
    for y in room.top..=room.bottom {
        for x in room.left..=room.right {
            let Some(cell) = map.point_to_cell(x, y) else {
                continue;
            };
            let interior = x > room.left && x < room.right && y > room.top && y < room.bottom;
            map.map[cell] = if interior { SECRET_TRAP } else { WALL };
            if interior {
                // TrapMechanism is absent in a fresh run, so every summoning
                // trap stays hidden and the reveal loop consumes no RNG.
                map.trap_names[cell] = Some("SummoningTrap");
            }
        }
    }

    let center = room.as_rect().center_room();
    map.point_to_cell(center.x, center.y)
        .expect("SecretSummoningRoom center is inside the map")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use crate::rooms::types::RoomKind;

    #[test]
    fn paints_hidden_trap_field_and_consumes_center_jitter() {
        let mut room = Room::new(0, "SecretSummoningRoom", RoomKind::Secret, 1, 1, 5, 8, 5, 8);
        room.left = 1;
        room.top = 1;
        room.right = 5;
        room.bottom = 6;
        let mut map = crate::level::terrain::paint_minimal(std::slice::from_ref(&room))
            .expect("summoning map");

        Random::reset_generators();
        Random::push_generator_seeded(0x5A11);
        let expected_y = 3 + Random::int_max(2);
        let expected_tail = Random::int();
        Random::pop_generator();

        Random::push_generator_seeded(0x5A11);
        let heap_cell = paint(&mut map, &room);
        let actual_tail = Random::int();
        Random::pop_generator();

        assert_eq!(
            heap_cell,
            map.point_to_cell(3, expected_y).expect("expected center")
        );
        assert_eq!(actual_tail, expected_tail);
        assert_eq!(
            map.trap_names
                .iter()
                .filter(|name| **name == Some("SummoningTrap"))
                .count(),
            12
        );
        assert_eq!(
            map.map.iter().filter(|&&tile| tile == SECRET_TRAP).count(),
            12
        );
    }
}
