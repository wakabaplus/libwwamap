use log::{debug, warn};

use crate::{binary::Binary, error::Error};
use self::{map::Map, object::Object};
use std::cmp;

pub mod map;
pub mod object;
pub mod player;

pub struct Status {
    pub energy: u16,
    pub strength: u16,
    pub defence: u16,
    pub gold: u16,
}

impl TryFrom<&[u16]> for Status {
    type Error = Error;
    fn try_from(arr: &[u16]) -> Result<Self, Self::Error> {
        let [a, b, c, d, ..] = arr else {
            return Err(Error::InvalidArgument);
        };
        Ok(Self {
            energy: *a,
            strength: *b,
            defence: *c,
            gold: *d,
        })
    }
}

pub struct EntityArray {
    object: Vec<Option<Object>>,
    map: Vec<Option<Map>>
}

impl From<&Binary> for EntityArray {
    fn from(bin: &Binary) -> Self {
        const CHUNK_SIZE: usize = 60;
        // The object's properties follow the background's.
        let map_count = cmp::max(bin.header[17].div_ceil(50) * 50, 200);
        let object_count = cmp::max(bin.header[18].div_ceil(50) * 50, 200);
        let map_length = usize::from(map_count) * CHUNK_SIZE;
        let object_length = usize::from(object_count) * CHUNK_SIZE;

        let mut object: Vec<Option<Object>> = std::iter::repeat_with(|| None).take(object_length).collect::<Vec<_>>();
        let mut map: Vec<Option<Map>> = std::iter::repeat_with(|| None).take(map_length).collect::<Vec<_>>();

        for (id, chunk) in bin.object
            .chunks_exact(CHUNK_SIZE)
            .enumerate()
        {
            debug!("Object {id}: {:?}", chunk);
            let obj = Object::try_from(chunk);
            if let Ok(c) = obj {
                object[id] = Some(c);
            } else {
                warn!("Object ID {id}: {:?}
                {:?}", obj.err(), chunk);
            }
        }

        for (id, chunk) in bin.map
            .chunks_exact(CHUNK_SIZE)
            .enumerate()
        {
            debug!("Object {id}: {:?}", chunk);
            let m = Map::try_from(chunk);
            if let Ok(c) = m {
                map[id] = Some(c);
            } else {
                warn!("Map ID {id}: {:?}
                {:?}", m.err(), chunk);
            }
        }
        Self { object, map }
    }
}
