use super::MoveTo;
use crate::{
    error::Error,
    tiled,
};

pub struct Message {
    pub position: tiled::Map,
    // pub image: tiled::Image,
    pub move_to: MoveTo,
    // pub sound_id,
    // pub wait_time,
    // pub message_id,
    // pub dynamic_parts
}

pub const CHUNK_ID: u8 = 1;

impl TryFrom<&[u8]> for Message {
    type Error = Error;
    fn try_from(chunk: &[u8]) -> Result<Self, Self::Error> {
        let position = tiled::Map::try_from(&chunk[38..])?;
        let move_to = MoveTo::try_from(chunk)?;

        Ok(Self {
            position,
            // image: tiled::Image::Object(0, 0),
            move_to,
        })
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            position: tiled::Map::default(),
            // image: tiled::Image::Object(u16::default(), u16::default()),
            move_to: MoveTo::None,
        }
    }
}
