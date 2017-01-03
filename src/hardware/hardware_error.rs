use std::fmt;
use std::error::Error;
use std::io;

use hardware::protocol::ProtocolError;

#[derive(Debug)]
pub enum HardwareError {
    ProtocolError(ProtocolError),
    IoError(io::Error)
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HardwareError::ProtocolError(ref err) => fmt::Display::fmt(err, f),
            HardwareError::IoError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

impl Error for HardwareError {
    fn description(&self) -> &str {
        match *self {
            HardwareError::ProtocolError(ref err) => err.description(),
            HardwareError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            HardwareError::ProtocolError(ref err) => err as &Error,
            HardwareError::IoError(ref err) => err as &Error,
        })
    }
}

impl From<ProtocolError> for HardwareError {
    fn from(err: ProtocolError) -> HardwareError {
        HardwareError::ProtocolError(err)
    }
}

impl From<io::Error> for HardwareError {
    fn from(err: io::Error) -> HardwareError {
        HardwareError::IoError(err)
    }
}
