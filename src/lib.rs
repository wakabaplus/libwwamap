mod entity;
mod error;
mod utils;

pub mod binary;
pub mod ext;
pub mod wwamap;
pub mod string;
pub mod tiled;

// #[cfg(target_family = "wasm")]
// mod wasm;

pub fn apply_log() {
    use simple_logger::SimpleLogger;
    use log::LevelFilter;
    SimpleLogger::new().with_level(LevelFilter::Warn).init().unwrap();
}
