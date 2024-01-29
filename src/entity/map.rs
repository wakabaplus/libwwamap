use crate::{binary, error::Error};
use log::warn;

pub mod item_check;
pub mod local_gate;
pub mod street;
pub mod url_gate;
pub mod wall;
pub mod world_gate;

pub enum Map {
    Street(street::Street),
    // Wall = 1,
    // LocalGate = 2,
    // WorldGate = 3,
    // UrlGate = 4,
    // ItemCheck = 5,
}

impl Map {
    pub fn parse(bin: &binary::Binary) -> Vec<Self> {
        const PROPERTY_LENGTH: usize = 60;
        let background_length: usize = usize::from(bin.header[17]) * PROPERTY_LENGTH;
        let mut map: Vec<Map> = Vec::with_capacity(background_length);
        for (id, prop) in bin.map[..background_length]
            .chunks_exact(PROPERTY_LENGTH)
            .enumerate()
        {
            if let Ok(c) = Map::parse_chunk(prop) {
                map[id] = c;
            } else {
                let ty: u16 = prop[3];
                warn!("Map ID {id}: Unknown chunk type {ty}.")
            }
        }
        map
    }

    pub fn parse_chunk(chunk: &[u16]) -> Result<Self, Error> {
        let ty = chunk[3].to_le_bytes()[0];
        match ty {
            n if n == street::Street::CHUNK_ID => Ok(Self::Street(street::Street::parse(chunk))),
            _ => Err(Error::InvalidByte { byte: ty }),
        }
    }
}
