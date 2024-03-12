use crate::{binary::Binary, error::Error};

use super::WWAString;

pub struct StringList {
    pub extend_message: Vec<WWAString>,
    pub max_message: u16,
    pub message: Vec<WWAString>,
    pub password: WWAString,
    pub title: WWAString,
    pub img_file: WWAString,
}

pub enum Meta {
    ConfirmLink = 5,
    InsufficientFunds = 6,
    NoItem = 7,
    UseItem = 8,
    GetItem = 9,
    FullItem = 10,
    SoundConfirm = 11
}

impl TryFrom<&Binary> for StringList {
    type Error = Error;
    fn try_from(bin: &Binary) -> Result<Self, Self::Error> {
        let mut str = bin.str.split(|&b| b == 0);
        let password: WWAString = WWAString::from(str.next().unwrap().to_vec());

        let max_message: u16 = bin.header[24];
        let mut message: Vec<WWAString> = Vec::with_capacity(max_message as usize);
        let mut extend_message: Vec<WWAString> = Vec::with_capacity(20);

        for _ in 0..max_message {
            message.push(WWAString::from(str.next().unwrap().to_vec()));
        }

        let title: WWAString = WWAString::from(str.next().unwrap().to_vec());
        let _ = str.next(); // older password
        let _ = str.next(); // older img_file (bmp)
        let img_file: WWAString  = WWAString::from(str.next().unwrap().to_vec());

        for _ in 0..20 {
            extend_message.push(WWAString::from(str.next().unwrap().to_vec()));
        }

        Ok(Self { max_message, message, extend_message, password, title, img_file })
    }
}
