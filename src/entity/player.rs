use super::Status;
use crate::{
    binary::Binary,
    tiled
};

pub struct Player {
    pub position: tiled::Map,
    pub gameover_position: tiled::Map,
    pub energy_max: u16,
    pub status: Status,
}

impl Player {
    pub fn parse(bin: &Binary) -> Self {
        let bin = &bin.header;
        Self {
            position: tiled::Map::try_from(&bin[19..]).unwrap(),
            gameover_position: tiled::Map::try_from(&bin[21..]).unwrap(),
            energy_max: bin[16],
            status: Status::try_from(&bin[5..]).unwrap(),
        }
    }
}
