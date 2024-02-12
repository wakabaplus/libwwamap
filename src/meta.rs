use crate::{
    binary,
    entity::player::Player,
    error::Error,
    string::{
        StringList,
        WWAString
    }
};

pub struct Meta {
    pub version: u8,
    pub player: Player,
    pub string: Vec<WWAString>,
    pub string_max: u16,
    pub password: WWAString,
    pub title: WWAString,
    pub img_file: WWAString,
    pub confirm_link_message: WWAString,
    pub insufficient_funds_message: WWAString,
    pub no_item_message: WWAString,
    pub use_item_message: WWAString,
    pub get_item_message: WWAString,
    pub full_item_message: WWAString,
    pub sound_confirm_message: WWAString,
}

impl Meta {
    pub fn parse(bin: &binary::Binary) -> Result<Self, Error> {
        let version = bin.header[1].to_le_bytes()[0];
        if version != 30 && version != 31 {
            return Err(Error::UnsupportedVersion { version });
        }
        let string_list: StringList = StringList::try_from(bin)?;
        let string: Vec<WWAString> = string_list.list;
        let string_max: u16 = string_list.max;
        let confirm_link_message: WWAString = string[5].clone();
        let insufficient_funds_message: WWAString = string[6].clone();
        let no_item_message: WWAString = string[7].clone();
        let use_item_message: WWAString = string[8].clone();
        let get_item_message: WWAString = string_list.new_string_list[0].clone();
        let full_item_message: WWAString = string_list.new_string_list[1].clone();
        let sound_confirm_message: WWAString = string_list.new_string_list[2].clone();
        let password: WWAString = string_list.password;
        let title: WWAString = string_list.title;
        let img_file: WWAString = string_list.img_file;
        let player: Player = Player::parse(bin);
        Ok(Self {
            player,
            version,
            string,
            string_max,
            password,
            title,
            img_file,
            confirm_link_message,
            insufficient_funds_message,
            no_item_message,
            use_item_message,
            get_item_message,
            full_item_message,
            sound_confirm_message,
        })
    }
}
