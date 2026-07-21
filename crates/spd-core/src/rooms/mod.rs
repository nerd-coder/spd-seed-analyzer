//! Room deck init and floor room selection.

mod dimensions;
pub mod init_rooms;
pub mod room;
pub mod secret;
pub mod special;
pub mod standard;
pub mod types;

use secret::init_for_run as secret_init;
use special::init_for_run as special_init;

/// Run-level room deck state after `initForRun`.
#[derive(Debug, Clone)]
pub struct RoomRunState {
    pub specials: Vec<&'static str>,
    pub secrets: Vec<&'static str>,
    pub region_secrets: [i32; 5],
    pub pit_needed_depth: i32,
}

/// `SpecialRoom.initForRun` + `SecretRoom.initForRun`.
pub fn init_rooms_for_run() -> RoomRunState {
    let specials = special_init();
    let (secrets, region_secrets) = secret_init();
    RoomRunState {
        specials,
        secrets,
        region_secrets,
        pit_needed_depth: -1,
    }
}
