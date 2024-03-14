use crate::{
    binary,
    entity::player::Player,
    error::Error,
    string::array::StringArray
};

pub struct WWAMap {
    pub version: u8,
    pub player: Player,
    pub string: StringArray,
}

impl WWAMap {
    pub fn parse(bin: &binary::Binary) -> Result<Self, Error> {
        let version = bin.header[1].to_le_bytes()[0];
        if version != 30 && version != 31 {
            return Err(Error::UnsupportedVersion { version });
        }

        let message: StringArray = StringArray::try_from(bin).unwrap();
        let player: Player = Player::parse(bin);

        Ok(Self {
            version,
            player,
            string: message,
        })
    }
}
