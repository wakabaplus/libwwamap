use crate::error::Error;
use core::fmt;

#[derive(Default)]
pub struct Map {
    pub x: u16,
    pub y: u16,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map Tiled Index")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum Image {
    Object(u16, u16),
    Background(u16),
}

impl TryFrom<&[u16]> for Map {
    type Error = Error;

    fn try_from(arr: &[u16]) -> Result<Self, Self::Error> {
        let [x, y, ..] = arr else {
            return Err(Error::InvalidArgument);
        };
        Ok(Self { x: *x, y: *y })
    }
}
