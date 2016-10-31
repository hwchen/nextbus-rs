//! Module for custom errors

use std::error;
use std::fmt;
use hyper::error::Error as HyperError;
use rquery::{DocumentError as DocError, SelectError as SelError};

pub type Result<T> = ::std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    BuildCommandError,
    BuildUrlError,
    HttpError(HyperError),
    ParseError,
    XmlError(RqueryError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BuildCommandError => write!(f, "Error Building Command"),
            Error::BuildUrlError => write!(f, "Error Building Url"),
            Error::HttpError(ref err) => write!(f, "HTTP Error: {}", err),
            Error::ParseError => write!(f, "Error Parsing Nextbus API"),
            Error::XmlError(ref err) => write!(f, "Error Parsing XML: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BuildCommandError => "Error Building Command",
            Error::BuildUrlError => "Error Building Url",
            Error::HttpError(ref err) => err.description(),
            Error::ParseError => "Error Parsing Nextbus API",
            Error::XmlError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::HttpError(ref err) => Some(err),
            Error::XmlError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::HttpError(err)
    }
}

// Consolidating rquery errors into one error enum
// This requires a whole another section for setting up
// display and error traits. Also, rquery errors do not
// implement Error trait!

#[derive(Debug)]
pub enum RqueryError {
    DocumentError(DocError),
    SelectError(SelError),
}

impl fmt::Display for RqueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RqueryError::DocumentError(_) => write!(f, "Document Error"),
            RqueryError::SelectError(_) => write!(f, "XML Select Error"),
        }
    }
}

impl error::Error for RqueryError {
    fn description(&self) -> &str {
        match *self {
            RqueryError::DocumentError(_) => "Error Loading Document",
            RqueryError::SelectError(_) => "Selector Error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

// Now can implement From for Error

impl From<DocError> for Error {
    fn from(err: DocError) -> Error {
        Error::XmlError(RqueryError::DocumentError(err))
    }
}

impl From<SelError> for Error {
    fn from(err: SelError) -> Error {
        Error::XmlError(RqueryError::SelectError(err))
    }
}
