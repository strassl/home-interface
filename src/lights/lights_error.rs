use std::fmt;
use std::error::Error;
use std::io;

use lights::protocol::ProtocolError;

#[derive(Debug)]
pub enum LightsError {
    ProtocolError(ProtocolError),
    IoError(io::Error)
}

impl fmt::Display for LightsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LightsError::ProtocolError(ref err) => fmt::Display::fmt(err, f),
            LightsError::IoError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

impl Error for LightsError {
    fn description(&self) -> &str {
        match *self {
            LightsError::ProtocolError(ref err) => err.description(),
            LightsError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            LightsError::ProtocolError(ref err) => err as &Error,
            LightsError::IoError(ref err) => err as &Error,
        })
    }
}

impl From<ProtocolError> for LightsError {
    fn from(err: ProtocolError) -> LightsError {
        LightsError::ProtocolError(err)
    }
}

impl From<io::Error> for LightsError {
    fn from(err: io::Error) -> LightsError {
        LightsError::IoError(err)
    }
}
