use crate::error::Error;

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
