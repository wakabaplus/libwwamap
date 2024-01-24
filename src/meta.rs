use crate::{entity::player::Player, error::Error, utils, wwa::Binary};

pub struct Meta {
    pub version: u8,
    pub player: Player,
    pub msg_max: u16,
    // pub imgfile: String,
    // pub password: String,
    // pub title: String,
}

impl Meta {
    pub fn parse(bin: &Binary) -> Result<Self, Error> {
        let version: u8 = utils::u16_to_first_u8_le(bin.header[1]);
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
