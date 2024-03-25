use crate::{error::Error, tiled};

pub const CHUNK_ID: u16 = 0;

pub struct Street {
    pub position: tiled::Map,
    // pub image: tiled::Image,
}

impl TryFrom<&Vec<u16>> for Street {
    type Error = Error;
    fn try_from(chunk: &Vec<u16>) -> Result<Self, Self::Error> {
        Ok(Self {
            position: tiled::Map::try_from(&chunk[19..]).unwrap(),
            // image: tiled::Image::Object(0, 0),
        })
    }
}

impl Default for Street {
    fn default() -> Self {
        Self {
            position: tiled::Map::default(),
            // image: tiled::Image::Object(u16::default(), u16::default()),
        }
    }
}
