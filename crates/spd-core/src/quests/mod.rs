//! NPC quest generation (Ghost / Wandmaker / Imp / …).

mod ghost;
mod imp;
mod wandmaker;

pub use ghost::{try_spawn_ghost, GhostQuestState, GhostType};
pub use imp::{take_pending as take_imp_pending, try_spawn as try_spawn_imp, ImpQuestState};
pub use wandmaker::{
    try_spawn_room as try_spawn_wandmaker_room, try_spawn_wandmaker, WandmakerQuestState,
    WandmakerQuestType,
};
