#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    BinCodeError(bincode::Error)
}

use Error::*;
use crate::error::Error::IoError;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        IoError(e)
    }
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Error {
        BinCodeError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
