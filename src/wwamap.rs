use crate::{
    binary::Binary,
    entity::{object::Object, player::Player},
    error::Error,
    string::array::StringArray,
};

pub struct WWAMap {
    pub version: u8,
    pub player: Player,
    pub string: StringArray,
    // pub map: Map,
    pub object: Object,
}

impl TryFrom<Binary> for WWAMap {
    type Error = Error;

    fn try_from(bin: Binary) -> Result<Self, Self::Error> {
        let version = bin.header[1].to_le_bytes()[0];
        if version != 30 && version != 31 {
            return Err(Error::UnsupportedVersion { version });
        }

        Ok(Self {
            version,
            player: Player::parse(&bin),
            string: StringArray::try_from(&bin)?,
            // map: Map::try_from(bin)?,
            object: Object::try_from(&bin)?,
        })
    }
}
