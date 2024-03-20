use crate::{
    binary,
    entity::{
        EntityArray,
        player::Player
    },
    error::Error,
    string::array::StringArray
};

pub struct WWAMap {
    pub version: u8,
    pub player: Player,
    pub string: StringArray,
    pub entity: EntityArray,
}

impl WWAMap {
    pub fn parse(bin: &binary::Binary) -> Result<Self, Error> {
        let version = bin.header[1].to_le_bytes()[0];
        if version != 30 && version != 31 {
            return Err(Error::UnsupportedVersion { version });
        }

        let string: StringArray = StringArray::try_from(bin).unwrap();
        let player: Player = Player::parse(bin);
        let entity: EntityArray = EntityArray::from(bin);

        Ok(Self {
            version,
            player,
            string,
            entity,
        })
    }
}
