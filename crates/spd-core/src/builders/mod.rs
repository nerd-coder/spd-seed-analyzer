//! Level builders (Loop / Figure-Eight).

mod place;
mod regular;

pub use place::{angle_between_rooms, find_free_space, place_room};
pub use regular::BuilderParams;

use crate::rooms::init_rooms::BuilderKind;
use crate::rooms::room::{clear_all_connections, Room};

/// Place rooms with the selected builder. Clears connections and retries until success
/// or `max_tries` failures (returns false).
pub fn build_rooms(
    rooms: &mut Vec<Room>,
    kind: BuilderKind,
    intensity: f32,
    offset: f32,
    depth: i32,
    max_tries: u32,
) -> bool {
    let mut params = BuilderParams::default();
    params.curve_exponent = 2;
    params.curve_intensity = intensity % 1.0;
    params.curve_offset = offset % 0.5;

    for _ in 0..max_tries {
        clear_all_connections(rooms);
        for r in rooms.iter_mut() {
            r.set_empty();
        }
        // Re-assign ids after possible connection room appends from failed tries
        rooms.retain(|r| r.kind != crate::rooms::types::RoomKind::Connection);
        for (i, r) in rooms.iter_mut().enumerate() {
            r.id = i;
        }

        let ok = match kind {
            BuilderKind::Loop => regular::build_loop(rooms, &params, depth).is_some(),
            BuilderKind::FigureEight => {
                // Full figure-eight not complete; use loop placement with same params
                // (still produces a connected map for createItems development)
                regular::build_loop(rooms, &params, depth).is_some()
            }
        };
        if ok {
            return true;
        }
    }
    false
}
