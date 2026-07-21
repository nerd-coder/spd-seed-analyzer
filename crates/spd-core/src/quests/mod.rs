//! NPC quest generation (Ghost / Wandmaker / Imp / …).

mod ghost;
mod wandmaker;

pub use ghost::{try_spawn_ghost, GhostQuestState, GhostType};
pub use wandmaker::{
    try_spawn_room as try_spawn_wandmaker_room, try_spawn_wandmaker, WandmakerQuestState,
    WandmakerQuestType,
};
