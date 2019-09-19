use std::path::Path;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    Deserialize(toml::de::Error),
    FileOpen(std::io::Error),
    FileRead(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Deserialize(ref err) => write!(f, "error when deserializing config: {}", err),
            Self::FileOpen(ref err) => write!(f, "error when opening file: {}", err),
            Self::FileRead(ref err) => write!(f, "error when reading file: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Deserialize(ref err) => Some(err),
            Self::FileOpen(ref err) => Some(err),
            Self::FileRead(ref err) => Some(err),
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    token: String,
    get_link_api_url: String,
    list_link_api_url: String,
}

impl Config {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        toml::from_str(s).map_err(Error::Deserialize)
    }

    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
        let p = p.as_ref();
        let mut f = File::open(p).map_err(Error::FileOpen)?;

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).map_err(Error::FileRead)?;

        Self::from_str(&buffer)
    }
}
