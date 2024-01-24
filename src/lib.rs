mod binary;
mod entity;
mod error;
pub mod index;
mod meta;
mod utils;

// #[cfg(target_family = "wasm")]
// mod wasm;

pub mod wwa {
    pub use crate::{binary::Binary, entity::map::Map, entity::object::Object, index, meta::Meta};
}
