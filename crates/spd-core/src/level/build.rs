//! RegularLevel outer/inner build retry orchestration.

use crate::builders;
use crate::dungeon::DungeonState;
use crate::rooms::init_rooms::{self, FloorRooms};
use crate::rooms::types::RoomKind;

use super::{shop, Feeling};

const BUILD_RETRY_LIMIT: u32 = 10_000;

/// `Level.create` retries the entire `RegularLevel.build`, while the nested
/// builder retry reuses one shuffled initRooms list and one builder instance.
pub(super) fn regular_rooms(
    dungeon: &mut DungeonState,
    feeling: Feeling,
    shop: bool,
) -> Option<FloorRooms> {
    for _ in 0..BUILD_RETRY_LIMIT {
        let mut floor = init_rooms::init_rooms_regular(
            dungeon.depth,
            feeling,
            shop,
            &mut dungeon.limited.lab_room,
            &mut dungeon.rooms.specials,
            &mut dungeon.rooms.secrets,
            &mut dungeon.rooms.region_secrets,
            &mut dungeon.rooms.pit_needed_depth,
            &mut dungeon.wandmaker,
            &mut dungeon.blacksmith,
            &mut dungeon.imp,
            &mut dungeon.generator,
        );
        let mut shop_items = None;
        if !builders::build_rooms(
            &mut floor.rooms,
            floor.builder_kind,
            floor.curve_intensity,
            floor.curve_offset,
            dungeon.depth,
            BUILD_RETRY_LIMIT,
            &mut |shop_room| {
                if shop_items.is_none() {
                    let items = shop::generate_items(dungeon);
                    // ShopRoom.spacesNeeded ignores sandbags, adds four stable
                    // sandbag slots, then one shopkeeper slot.
                    let spaces_needed = items.len() + 5;
                    let min_size = ((spaces_needed as f64).sqrt() as i32 + 3).max(7);
                    shop_room.min_w = min_size;
                    shop_room.min_h = min_size;
                    shop_items = Some(items);
                }
            },
        ) {
            continue;
        }
        floor.shop_items = shop_items.unwrap_or_default();

        // RegularPainter returns false only for a disconnected SpecialRoom.
        // Preflight it so the whole build can be retried. A malformed failed
        // layout omits any partial room-paint burns before Java encounters the
        // room; valid builder output never takes this path.
        if !painter_accepts(&floor.rooms) {
            continue;
        }
        return Some(floor);
    }
    None
}

fn painter_accepts(rooms: &[crate::rooms::room::Room]) -> bool {
    !rooms.iter().any(|room| {
        matches!(room.kind, RoomKind::Special | RoomKind::Shop)
            && !room.is_empty()
            && room.connected.is_empty()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::room::Room;

    fn placed(kind: RoomKind) -> Room {
        let mut room = Room::new(0, "Room", kind, 1, 1, 5, 5, 5, 5);
        room.resize(4, 4);
        room
    }

    #[test]
    fn painter_retry_gate_matches_special_room_hierarchy() {
        assert!(!painter_accepts(&[placed(RoomKind::Special)]));
        assert!(!painter_accepts(&[placed(RoomKind::Shop)]));
        assert!(painter_accepts(&[placed(RoomKind::Secret)]));

        let mut special = placed(RoomKind::Special);
        special.connected.push(1);
        assert!(painter_accepts(&[special]));
    }
}
