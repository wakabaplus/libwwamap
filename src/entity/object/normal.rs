use super::MoveTo;
use crate::{
    error::Error,
    tiled,
};

pub struct Normal {
    pub position: tiled::Map,
    // pub image: tiled::Image,
    pub is_layer: bool,
    pub move_to: MoveTo,
}

pub const CHUNK_ID: u8 = 0;

impl TryFrom<&[u8]> for Normal {
    type Error = Error;
    fn try_from(chunk: &[u8]) -> Result<Self, Self::Error> {
        // TODO: Treat chunk[2] as bool
        let is_layer = chunk[4].to_le_bytes()[0] == 1;
        let position = tiled::Map::try_from(&chunk[38..])?;
        let move_to = MoveTo::try_from(chunk)?;

        Ok(Self {
            position,
            // image: tiled::Image::Object(0, 0),
            is_layer,
            move_to,
        })
    }
}

impl Default for Normal {
    fn default() -> Self {
        Self {
            position: tiled::Map::default(),
            // image: tiled::Image::Object(u16::default(), u16::default()),
            is_layer: false,
            move_to: MoveTo::None,
        }
    }
}
