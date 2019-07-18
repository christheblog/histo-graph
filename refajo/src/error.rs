#[derive(Debug)]
pub enum Error {
    FileError(histo_graph_file::error::Error),
    SerdeJsonError(serde_json::error::Error),
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

use Error::*;

impl From<histo_graph_file::error::Error> for Error {
    fn from(e: histo_graph_file::error::Error) -> Error {
        FileError(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        SerdeJsonError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        IoError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        ParseIntError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
