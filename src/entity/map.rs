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

impl TryFrom<&[u8]> for Map {
    type Error = Error;
    fn try_from(chunk: &[u8]) -> Result<Self, Self::Error> {
        let b = chunk[6];
        match b {
            n if n == street::CHUNK_ID => Ok(Self::Street(street::Street::try_from(chunk)?)),
            _ => Err(Error::InvalidByte { byte: b }),
        }
    }
}
