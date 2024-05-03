use std::io::{Error as IOError, ErrorKind};

#[derive(Debug)]
pub enum Error {
    Config(String),
    IO(ErrorKind),
    Certificate(String),
    ConfigParse(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IO(value.kind())
    }
}
