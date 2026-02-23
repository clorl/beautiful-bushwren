use std;

#[derive(Debug)]
pub enum Error {
    Config(String),
    Nickel(String),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Config(s) => write!(f, "Config Error: {s}"),
            Error::Nickel(s) => write!(f, "Nickel Error: {s}"),
            Error::Other(s) => write!(f, "Error: {s}"),
        }
    }
}

impl std::error::Error for Error {}
