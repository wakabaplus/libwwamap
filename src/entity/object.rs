use log::{debug, info, warn};

use crate::{binary::Binary, error::Error};
use std::ops::Index;

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

pub struct Object {
    type_array: Vec<ObjectType>,
    // pub normal: Normal,
    // pub message: Message,
}

impl Index<usize> for Object {
    type Output = ObjectType;
    fn index(&self, id: usize) -> &Self::Output {
        &self.type_array[id]
    }
}

impl TryFrom<&Binary> for Object {
    type Error = Error;
    fn try_from(bin: &Binary) -> Result<Self, Self::Error> {
        const CHUNK_SIZE: usize = 60;
        let entities = bin.object_entity.chunks_exact(CHUNK_SIZE);
        let object_entity_size = usize::from(bin.header[18]) * CHUNK_SIZE;
        let mut type_array = Vec::with_capacity(object_entity_size);
        for (id, entity) in entities.enumerate() {
            debug!("{id}: {entity:?}");
            let ty = ObjectType::try_from(entity);
            if let Ok(t) = ty {
                type_array.push(t)
            } else {
                warn!("{id}: {:?}", ty.err());
                type_array.push(ObjectType::BrokenObject)
            }
        }
        Ok(Self { type_array })
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.type_array)
    }
}

pub enum MoveTo {
    None,
    Player,
    Escape,
    Random,
}

impl TryFrom<u16> for MoveTo {
    type Error = Error;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Player),
            2 => Ok(Self::Escape),
            3 => Ok(Self::Random),
            _ => Err(Error::InvalidByte { byte: v }),
        }
    }
}

impl TryFrom<&Vec<u16>> for MoveTo {
    type Error = Error;
    fn try_from(v: &Vec<u16>) -> Result<Self, Self::Error> {
        Self::try_from(v[32])
    }
}

pub enum ObjectType {
    Normal,
    Message,
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
    BrokenObject,
}

impl std::fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Message => write!(f, "Message"),
            Self::BrokenObject => write!(f, "<Broken>"),
        }
    }
}

impl TryFrom<&[u16]> for ObjectType {
    type Error = Error;
    fn try_from(chunk: &[u16]) -> Result<Self, Self::Error> {
        let b = chunk[3];
        match b {
            n if n == normal::CHUNK_ID => Ok(Self::Normal),
            n if n == message::CHUNK_ID => Ok(Self::Message),
            _ => Err(Error::InvalidByte { byte: b }),
        }
    }
}

impl PartialEq for ObjectType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
