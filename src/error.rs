extern crate reqwest;

use std::fmt;

pub enum Error {
    HttpError(reqwest::Error),
    InvalidStatusCode { code: reqwest::StatusCode },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HttpError(ref e) => writeln!(f, "Http Request Failed. {}", e),
            Error::InvalidStatusCode { code: c } => writeln!(f, "Invalid Status Code. {}", c),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::HttpError(e)
    }
}
