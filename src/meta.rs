use crate::{binary, entity::player::Player, error::Error, string::StringList};

pub struct Meta {
    pub version: u8,
    pub player: Player,
    pub string: StringList,
    // pub imgfile: String,
    // pub password: String,
    // pub title: String,
}

impl Meta {
    pub fn parse(bin: &binary::Binary) -> Result<Self, Error> {
        let version = bin.header[1].to_le_bytes()[0];
        if version != 30 && version != 31 {
            return Err(Error::UnsupportedVersion { version });
        }
        let string: StringList = StringList::try_from(bin)?;
        let player: Player = Player::parse(bin);
        Ok(Self {
            player,
            version,
            string
        })
    }
}
