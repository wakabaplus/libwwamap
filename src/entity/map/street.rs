use crate::index;

pub struct Street {
    pub position: index::Map,
    pub image: index::Image,
}

impl Street {
    pub const CHUNK_ID: u8 = 0;
    pub fn parse(bin: &[u16]) -> Self {
        Self {
            position: index::Map::try_from(&bin[19..]).unwrap(),
            image: index::Image::Object(0, 0),
        }
    }
}

impl Default for Street {
    fn default() -> Self {
        Self {
            position: index::Map::default(),
            image: index::Image::Object(u16::default(), u16::default()),
        }
    }
}
