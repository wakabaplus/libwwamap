use crate::{binary, entity::player::Player, error::Error};

pub struct Meta {
    pub version: u8,
    pub player: Player,
    pub msg_max: u16,
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
        let msg_max: u16 = bin.header[24];

        let player: Player = Player::parse(bin);
        Ok(Self {
            player,
            version,
            msg_max,
        })
    }
}
