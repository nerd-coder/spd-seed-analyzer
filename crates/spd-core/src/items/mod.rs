//! Item-related generation helpers (identities, registries).

pub mod catalog;
pub mod enchants;
pub(crate) mod exotic;
pub mod identities;
pub mod model;
pub mod randomize;
pub mod status_handler;

pub use identities::{init_identities, IdentityMaps};
pub use model::{GeneratedItem, ItemCategory};
pub use status_handler::assign_labels;
