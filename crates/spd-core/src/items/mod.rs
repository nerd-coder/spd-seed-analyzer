//! Item-related generation helpers (identities, registries).

pub mod catalog;
pub mod identities;
pub mod status_handler;

pub use identities::{IdentityMaps, init_identities};
pub use status_handler::assign_labels;
