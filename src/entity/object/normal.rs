use super::MoveTo;
use crate::{index, utils};
use log::warn;

pub struct Normal {
    pub position: index::Map,
    pub image: index::Image,
    pub is_layer: bool,
    pub move_to: MoveTo,
}

impl Normal {
    pub const CHUNK_ID: u8 = 0;
    pub fn parse(bin: &[u16]) -> Self {
        let mut is_layer = false;
        let layer_value = utils::u16_to_first_u8_le(bin[2]);
        if layer_value == 1 {
            is_layer = true;
        } else if layer_value != 0 {
            warn!("Unexpected value: {layer_value} (should be 0 or 1)")
        }
        Self {
            position: index::Map::try_from(&bin[19..]).unwrap(),
            image: index::Image::Object(0, 0),
            is_layer,
            move_to: MoveTo::try_from(utils::u16_to_first_u8_le(bin[16])).unwrap(),
        }
    }
}

impl Default for Normal {
    fn default() -> Self {
        Self {
            position: index::Map::default(),
            image: index::Image::Object(u16::default(), u16::default()),
            is_layer: false,
            move_to: MoveTo::None,
        }
    }
}
