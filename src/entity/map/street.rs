use crate::tiled;

pub struct Street {
    pub position: tiled::Map,
    pub image: tiled::Image,
}

impl Street {
    pub const CHUNK_ID: u8 = 0;
    pub fn parse(bin: &[u16]) -> Self {
        Self {
            position: tiled::Map::try_from(&bin[19..]).unwrap(),
            image: tiled::Image::Object(0, 0),
        }
    }
}

impl Default for Street {
    fn default() -> Self {
        Self {
            position: tiled::Map::default(),
            image: tiled::Image::Object(u16::default(), u16::default()),
        }
    }
}
