use crate::error::Error;

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

impl TryFrom<&[u16]> for Map {
    type Error = Error;
    fn try_from(chunk: &[u16]) -> Result<Self, Self::Error> {
        let ty = chunk[3].to_le_bytes()[0];
        match ty {
            n if n == street::Street::CHUNK_ID => Ok(Self::Street(street::Street::parse(chunk))),
            _ => Err(Error::InvalidByte { byte: ty }),
        }
    }
}