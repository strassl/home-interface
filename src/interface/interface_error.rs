use std::fmt;
use std::error::Error;
use std::io;

use hardware::HardwareError;
use interface::serde_json;

#[derive(Debug)]
pub enum InterfaceError {
    HardwareError(HardwareError),
    DeserializationError(serde_json::Error),
    IoError(io::Error),
    OtherError(Box<Error + 'static + Send>),
}

impl fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterfaceError::HardwareError(ref err) => fmt::Display::fmt(err, f),
            InterfaceError::DeserializationError(ref err) => fmt::Display::fmt(err, f),
            InterfaceError::IoError(ref err) => fmt::Display::fmt(err, f),
            InterfaceError::OtherError(ref err) => fmt::Display::fmt(&(*err), f),
        }
    }
}

impl Error for InterfaceError {
    fn description(&self) -> &str {
        match *self {
            InterfaceError::HardwareError(ref err) => err.description(),
            InterfaceError::DeserializationError(ref err) => err.description(),
            InterfaceError::IoError(ref err) => err.description(),
            InterfaceError::OtherError(ref err) => (*err).description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            InterfaceError::HardwareError(ref err) => err as &Error,
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

impl From<HardwareError> for InterfaceError {
    fn from(err: HardwareError) -> InterfaceError {
        InterfaceError::HardwareError(err)
    }
}

impl From<serde_json::Error> for InterfaceError {
    fn from(err: serde_json::Error) -> InterfaceError {
        InterfaceError::DeserializationError(err)
    }
}
