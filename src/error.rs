//! Module for custom errors

use std::error;
use std::fmt;
use hyper::error::Error as HyperError;


pub type Result<T> = ::std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    BuildUrlError,
    HttpError(HyperError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BuildUrlError => write!(f, "Error Building Url"),
            Error::HttpError(ref err) => write!(f, "HTTP Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BuildUrlError => "Error Building Url",
            Error::HttpError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::HttpError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::HttpError(err)
    }
}

// Create an enum just for builder errors, to get more granular errors.
// Then pass these into the normal error! Maybe do this later

