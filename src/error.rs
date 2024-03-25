use core::fmt;

#[derive(Debug)]
pub enum Error {
    NoChunk,
    UnsupportedVersion { version: u8 },
    InvalidChecksum { except: u16, actual: u16 },
    InvalidArgument,
    InvalidByte { byte: u16 },
    TooSmallLength { length: usize, least: usize },
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidChecksum { except, actual } => {
                write!(f, "Checksum is invalid. Except {except} but got {actual}.")
            }
            Error::NoChunk => {
                write!(f, "No chunk found.")
            }
            Error::InvalidByte { byte } => {
                write!(f, "Invalid byte: {byte}")
            }
            Error::TooSmallLength { length, least } => {
                write!(
                    f,
                    "The decoded length is too small: {length} (At least: {least})"
                )
            }
            Error::UnsupportedVersion { version } => {
                write!(
                    f,
                    "This WWAMap version ({version}) is not supported as of now."
                )
            }
            Error::InvalidArgument => {
                write!(f, "The argument is not of the expected type.")
            }
        }
    }
}
