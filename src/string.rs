use crate::{binary, error};
use core::char::{self, DecodeUtf16Error};
use core::{fmt, str::FromStr};
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
        Self{ vec }
    }
}

impl fmt::Display for WWAString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = core::char::decode_utf16(self.vec.iter().cloned())
        .map(|c| c.unwrap())
        .collect();
        writeln!(f, "{:?}", s)
    }
}

impl fmt::Debug for WWAString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self)
    }
}

pub struct StringList {
    pub max: u16,
    pub list: Vec<WWAString>,
}

impl TryFrom<&binary::Binary> for StringList {
    type Error = error::Error;

    fn try_from(bin: &binary::Binary) -> Result<Self, Self::Error> {
        const MAX_STRING: usize = 755;
        let max: u16 = bin.header[24];
        let str = bin.str.split(|&b| b == 0);
        let mut list: Vec<WWAString> = Vec::with_capacity(max as usize);
        for s in str {
            let c: Vec<char> = char::decode_utf16(s.to_vec()).map(|r: Result<char, DecodeUtf16Error>| r.unwrap()).collect();
            assert!(c.len() <= MAX_STRING);
            list.push(WWAString::from(s.to_vec()));
        }
        Ok(Self {
            max,
            list
        })
    }
}