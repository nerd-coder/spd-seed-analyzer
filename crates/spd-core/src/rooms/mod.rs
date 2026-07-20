//! Room deck init (SpecialRoom / SecretRoom run setup).

pub mod secret;
pub mod special;

/// Run-level room deck state after `initForRun`.
#[derive(Debug, Clone)]
pub struct RoomRunState {
    pub specials: Vec<&'static str>,
    pub secrets: Vec<&'static str>,
    pub region_secrets: [i32; 5],
}

/// `SpecialRoom.initForRun` + `SecretRoom.initForRun`.
pub fn init_rooms_for_run() -> RoomRunState {
    let specials = special::init_for_run();
    let (secrets, region_secrets) = secret::init_for_run();
    RoomRunState {
        specials,
        secrets,
        region_secrets,
    }
}
