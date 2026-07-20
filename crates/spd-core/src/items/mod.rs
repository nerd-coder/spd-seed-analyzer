//! Item-related generation helpers (identities, registries).

pub mod catalog;
pub mod enchants;
pub mod identities;
pub mod model;
pub mod randomize;
pub mod status_handler;

pub use identities::{IdentityMaps, init_identities};
pub use model::{GeneratedItem, ItemCategory};
pub use status_handler::assign_labels;
