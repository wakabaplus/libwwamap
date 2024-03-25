pub mod entity;
mod error;
mod utils;

pub mod binary;
pub mod ext;
pub mod string;
pub mod tiled;
pub mod wwamap;

// #[cfg(target_family = "wasm")]
// mod wasm;

pub fn apply_log() {
    use log::LevelFilter;
    use simple_logger::SimpleLogger;
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
}
