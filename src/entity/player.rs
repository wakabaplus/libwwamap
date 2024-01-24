use super::Status;
use crate::{binary::Binary, index};

pub struct Player {
    pub position: index::Map,
    pub energy_max: u16,
    pub status: Status,
    pub gameover_position: index::Map,
}

impl Player {
    pub fn parse(bin: &Binary) -> Self {
        let bin = &bin.header;
        println!("{:?}", bin);
        Self {
            position: index::Map::try_from(&bin[10..]).unwrap(),
            status: Status::try_from(&bin[5..]).unwrap(),
            energy_max: bin[16],
            gameover_position: index::Map::try_from(&bin[21..]).unwrap(),
        }
    }
}
