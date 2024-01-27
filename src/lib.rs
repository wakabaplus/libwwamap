pub mod binary;
mod entity;
mod error;
pub mod tiled;
pub mod meta;
mod utils;

// #[cfg(target_family = "wasm")]
// mod wasm;

pub mod wwa {
    pub use crate::{entity::map::Map, entity::object::Object, tiled};
}
