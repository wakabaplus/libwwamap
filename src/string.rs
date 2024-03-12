pub mod list;

use crate::error;
use core::{
    char::{
        decode_utf16,
        DecodeUtf16Error
    },
    fmt::{
        self,
        Display,
        Debug,
        Formatter
    },
    str::FromStr
};

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
