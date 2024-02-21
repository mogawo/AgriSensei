
use std::fmt;
pub use http::{Error as HTTPError};
pub use httparse::Error as HTTParseError;
pub use serde_json::Error as SerdeJSONError;
pub use std::num::ParseIntError;

use std::io::Error as IOError;
pub type BoxedError = Box<dyn std::error::Error>;
#[derive(Debug)]
pub enum ServerError<'se>
{
    //Error Constructors
    HTTPError(HTTPError),
    HTTParseError(HTTParseError),
    ParseIntError(ParseIntError),
    PathError((&'se str, String)),
    IOError(IOError),
    AssembleError(&'se str),
    ThreadError(&'se str),
    MessageError(&'se str),
    JSONError(SerdeJSONError),
}

impl std::error::Error for ServerError<'_>{}
impl fmt::Display for ServerError<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            ServerError::HTTPError(e) => {
                write!(f, "[HttpError]\r\n{}\r\n", e)
            },
            ServerError::PathError((msg, path)) => {
                write!(f, "[PathError]: {msg}\r\nPath:\r\n{path}\r\n")
            },
            Self::IOError(e) => {
                write!(f, "[IOError]:\r\n{e}\r\n")
            }
            ServerError::HTTParseError(e) => {
                write!(f, "[MimeTypeError]\r\n{e}\r\n")
            }
            ServerError::ParseIntError(e) => {
                write!(f, "[ParseIntError]\r\n{e}\r\n")
            }
            ServerError::AssembleError(msg) => {
                write!(f, "[AssembleError] {msg}\r\n")
            }
            ServerError::ThreadError(msg) => {
                write!(f, "[ThreadError] {msg}\r\n")
            }
            ServerError::MessageError(msg) => {
                write!(f, "[MessageError] {msg}\r\n")
            }
            ServerError::JSONError(e) => {
                write!(f, "[JSONError] {e}\r\n")
            }
        }
    }
}

impl From<http::Error> for ServerError<'_>{
    fn from(value: http::Error) -> Self {
        Self::HTTPError(value)
    }
}

impl From<httparse::Error> for ServerError<'_>{
    fn from(value: httparse::Error) -> Self {
        Self::HTTParseError(value)
    }
}

impl From<serde_json::Error> for ServerError<'_>{
    fn from(value: serde_json::Error) -> Self {
        Self::JSONError(value)
    }
}

impl From<std::io::Error> for ServerError<'_>{
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ParseIntError> for ServerError<'_>{
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
