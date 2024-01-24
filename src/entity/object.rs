use crate::{binary::Binary, error::Error, utils};
use log::warn;
pub mod buy;
pub mod door;
pub mod item;
pub mod local_gate;
pub mod message;
pub mod monster;
pub mod normal;
pub mod random;
pub mod score;
pub mod select;
pub mod sell;
pub mod status;
pub mod url_gate;

pub enum MoveTo {
    None,
    Player,
    Escape,
    Random,
}

impl TryFrom<u8> for MoveTo {
    type Error = Error;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Player),
            2 => Ok(Self::Escape),
            3 => Ok(Self::Random),
            _ => Err(Error::InvalidByte { byte: v }),
        }
    }
}

pub enum Object {
    Normal(normal::Normal),
    // Message = 1,
    // UrlGate = 2,
    // Status = 3,
    // Item = 4,
    // Door = 5,
    // Monster = 6,
    // Score = 11,
    // Sell = 14,
    // Buy = 15,
    // Random = 16,
    // Select = 17,
    // LocalGate = 18,
}

impl Object {
    pub fn parse(bin: &Binary) -> Vec<Self> {
        const PROPERTY_LENGTH: usize = 60;
        // The object's properties follow the background's.
        let background_length: usize = usize::from(bin.header[17]) * PROPERTY_LENGTH;
        let object_length: usize = usize::from(bin.header[18]) * PROPERTY_LENGTH;
        let bin_len = bin.object.len();
        if bin_len < background_length + object_length {
            warn!("Unexpected property size. {bin_len} < {background_length}(bkg) + {object_length}(obj)")
        }

        let mut object: Vec<Object> = Vec::with_capacity(object_length);
        for (id, prop) in bin.object[background_length..]
            .chunks_exact(PROPERTY_LENGTH)
            .enumerate()
        {
            if let Ok(c) = Object::parse_chunk(prop) {
                object[id] = c;
            } else {
                let ty: u16 = prop[3];
                warn!("Object ID {id}: Unknown chunk type {ty}.")
            }
        }
        object
    }

    pub fn parse_chunk(chunk: &[u16]) -> Result<Self, Error> {
        let ty = utils::u16_to_first_u8_le(chunk[3]);
        match ty {
            n if n == normal::Normal::CHUNK_ID => Ok(Self::Normal(normal::Normal::parse(chunk))),
            _ => Err(Error::InvalidByte { byte: ty }),
        }
    }
}
