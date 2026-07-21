//! NPC quest generation (Ghost / Wandmaker / Imp / …).

mod ghost;

pub use ghost::{try_spawn_ghost, GhostQuestState, GhostType};
