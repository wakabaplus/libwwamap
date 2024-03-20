use crate::{
    binary::Binary,
    error::Error
};
use log::{
    debug,
    info,
    warn
};
use self::{
    map::Map,
    object::Object
};
use std::{
    any,
    cmp
};

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

fn extract<T>(count: usize, bin: &[u8]) -> Vec<Option<T>>
where T: for<'a> TryFrom<&'a [u8], Error = Error>,
{
    const CHUNK_SIZE: usize = 60;
    let mut entity: Vec<Option<T>> = Vec::with_capacity(count * CHUNK_SIZE);
    let mut iter = bin.chunks_exact(CHUNK_SIZE);
    for id in 0..count {
        let mut e: Option<T> = None;
        if let Some(chunk) = iter.next() {
            let obj: Result<T, Error> = T::try_from(chunk);
            debug!("{} ID {id}: {:?}", any::type_name::<T>(), chunk);
            if let Ok(c) = obj {
                e = Some(c);
            } else {
                warn!("{} ID {id}: {:?} {:?}", any::type_name::<T>(), obj.err(), chunk);
            }
        } else {
            info!("{} ID {id}: None", any::type_name::<T>());
        }
        assert_eq!(entity.len(), id);
        entity.push(e);
    }
    entity
}

impl From<&Binary> for EntityArray {
    fn from(bin: &Binary) -> Self {
        let map_count = usize::from(cmp::max(bin.header[17].div_ceil(50) * 50, 200));
        let object_count = usize::from(cmp::max(bin.header[18].div_ceil(50) * 50, 200));
        let object: Vec<Option<Object>> = extract::<Object>(object_count, &bin.object);
        let map: Vec<Option<Map>> = extract::<Map>(map_count, &bin.map);

        Self { object, map }
    }
}
