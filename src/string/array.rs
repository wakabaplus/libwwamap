use crate::{binary::Binary, error::Error};
use core::ops::Index;
use super::WWAString;

pub struct StringArray {
    pub max_message: u16,
    extend_message: Vec<WWAString>,
    message: Vec<WWAString>,
    password: WWAString,
    title: WWAString,
    img_file: WWAString,
}

pub enum Meta {
    ConfirmLink,
    InsufficientFunds,
    NoItem,
    UseItem,
    GetItem,
    FullItem,
    SoundConfirm,
    Title,
    ImgFile,
    Password,
}

impl Index<Meta> for StringArray {
    type Output = WWAString;
    fn index(&self, attr: Meta) -> &Self::Output {
        match attr {
            Meta::ConfirmLink => &self.message[5],
            Meta::InsufficientFunds => &self.message[6],
            Meta::NoItem => &self.message[7],
            Meta::UseItem => &self.message[8],
            Meta::GetItem => &self.extend_message[0],
            Meta::FullItem => &self.extend_message[1],
            Meta::SoundConfirm => &self.extend_message[2],
            Meta::Title => &self.title,
            Meta::ImgFile => &self.img_file,
            Meta::Password => &self.password,
        }
    }
}

impl Index<usize> for StringArray {
    type Output = WWAString;
    fn index(&self, id: usize) -> &Self::Output {
        &self.message[id]
    }
}

impl TryFrom<&Binary> for StringArray {
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
        let img_file: WWAString = WWAString::from(str.next().unwrap().to_vec());

        for _ in 0..20 {
            extend_message.push(WWAString::from(str.next().unwrap().to_vec()));
        }

        Ok(Self { max_message, message, extend_message, password, title, img_file })
    }
}
