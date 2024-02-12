use crate::{binary, error};
use core::char::{decode_utf16, DecodeUtf16Error};
use core::fmt::{self, Display, Debug, Formatter};
use core::str::FromStr;

#[derive(Clone)]
pub struct WWAString {
    vec: Vec<u16>,
}

impl FromStr for WWAString {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a: Vec<u16> = s.encode_utf16().collect();
        Ok(Self::from(a))
    }
}

impl From<Vec<u16>> for WWAString {
    fn from(vec: Vec<u16>) -> Self {
        const MAX_STRING: usize = 755;
        assert!(vec.len() <= MAX_STRING);
        Self { vec }
    }
}

impl Display for WWAString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s: String = decode_utf16(self.vec.iter().cloned())
        .map(|c: Result<char, DecodeUtf16Error>| c.unwrap())
        .collect();
        writeln!(f, "{:?}", s)
    }
}

impl Debug for WWAString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl PartialEq<&str> for WWAString {
    fn eq(&self, other: &&str) -> bool {
        self.vec == other.encode_utf16().collect::<Vec<u16>>()
    }
}

pub(crate) struct StringList {
    pub max: u16,
    pub list: Vec<WWAString>,
    pub password: WWAString,
    pub title: WWAString,
    pub img_file: WWAString,
    pub new_string_list: Vec<WWAString>,
}

impl TryFrom<&binary::Binary> for StringList {
    type Error = error::Error;

    fn try_from(bin: &binary::Binary) -> Result<Self, Self::Error> {
        let max: u16 = bin.header[24];
        let mut str = bin.str.split(|&b| b == 0);
        let password = WWAString::from(str.next().unwrap().to_vec());
        let mut list: Vec<WWAString> = Vec::with_capacity(max as usize);
        for _ in 0..max {
            list.push(WWAString::from(str.next().unwrap().to_vec()));
        }
        let title = WWAString::from(str.next().unwrap().to_vec());
        let _ = WWAString::from(str.next().unwrap().to_vec()); // older password
        let _ = WWAString::from(str.next().unwrap().to_vec()); // older img_file (bmp)
        let img_file  = WWAString::from(str.next().unwrap().to_vec());
        let mut new_string_list: Vec<WWAString> = Vec::with_capacity(20);
        for _ in 0..20 {
            new_string_list.push(WWAString::from(str.next().unwrap().to_vec()));
        }
        Ok(Self {
            max,
            list,
            password,
            title,
            img_file,
            new_string_list
        })
    }
}