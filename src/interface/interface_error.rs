use std::fmt;
use std::error::Error;
use std::io;

use lights::LightsError;
use interface::serde_json;

#[derive(Debug)]
pub enum InterfaceError {
    LightsError(LightsError),
    DeserializationError(serde_json::Error),
    IoError(io::Error),
    OtherError(Box<Error + 'static + Send>),
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterfaceError::LightsError(ref err) => fmt::Display::fmt(err, f),
            InterfaceError::DeserializationError(ref err) => fmt::Display::fmt(err, f),
            InterfaceError::IoError(ref err) => fmt::Display::fmt(err, f),
            InterfaceError::OtherError(ref err) => fmt::Display::fmt(&(*err), f),
        }
    }
}

impl Error for InterfaceError {
    fn description(&self) -> &str {
        match *self {
            InterfaceError::LightsError(ref err) => err.description(),
            InterfaceError::DeserializationError(ref err) => err.description(),
            InterfaceError::IoError(ref err) => err.description(),
            InterfaceError::OtherError(ref err) => (*err).description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            InterfaceError::LightsError(ref err) => err as &Error,
            InterfaceError::DeserializationError(ref err) => err as &Error,
            InterfaceError::IoError(ref err) => err as &Error,
            InterfaceError::OtherError(ref err) => &(**err) as &Error,
        })
    }
}

impl From<io::Error> for InterfaceError {
    fn from(err: io::Error) -> InterfaceError {
        InterfaceError::IoError(err)
    }
}

impl From<LightsError> for InterfaceError {
    fn from(err: LightsError) -> InterfaceError {
        InterfaceError::LightsError(err)
    }
}

impl From<serde_json::Error> for InterfaceError {
    fn from(err: serde_json::Error) -> InterfaceError {
        InterfaceError::DeserializationError(err)
    }
}
