use crate::error::Error;

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

impl TryFrom<&[u8]> for MoveTo {
    type Error = Error;
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(v[32])
    }
}

pub enum Object {
    Normal(normal::Normal),
    Message(message::Message),
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

impl TryFrom<&[u8]> for Object {
    type Error = Error;
    fn try_from(chunk: &[u8]) -> Result<Self, Self::Error> {
        let b = chunk[6];
        match b {
            n if n == normal::CHUNK_ID => Ok(Self::Normal(normal::Normal::try_from(chunk)?)),
            n if n == message::CHUNK_ID => Ok(Self::Message(message::Message::try_from(chunk)?)),
            _ => Err(Error::InvalidByte { byte: b }),
        }
    }
}
