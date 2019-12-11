use cid;
use std::{error, fmt, io, net, num, string};

pub type Result<T> = ::std::result::Result<T, Error>;

/// Error types
#[derive(Debug)]
pub enum Error {
    UnknownProtocol,
    UnknownProtocolString,
    InvalidMultiaddr,
    MissingAddress,
    ParsingError(Box<dyn error::Error + Send + Sync>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;
        match (self, other) {
            (UnknownProtocol, UnknownProtocol) => true,
            (UnknownProtocolString, UnknownProtocolString) => true,
            (InvalidMultiaddr, InvalidMultiaddr) => true,
            (MissingAddress, MissingAddress) => true,
            (ParsingError(_), ParsingError(_)) => true,
            _ => false,
        }
    }
}

impl Eq for Error {}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownProtocol => "unknown protocol",
            Error::UnknownProtocolString => "unknown protocol string",
            Error::InvalidMultiaddr => "invalid multiaddr",
            Error::MissingAddress => "protocol requires address, none given",
            Error::ParsingError(_) => "failed to parse",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::ParsingError(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<cid::Error> for Error {
    fn from(err: cid::Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<data_encoding::DecodeError> for Error {
    fn from(err: data_encoding::DecodeError) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::ParsingError(err.into())
    }
}
