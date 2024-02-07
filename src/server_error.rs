use std::error;
use std::fmt;
use core::ops::Deref;
use std::io;
use std::error::Error;


#[derive(Debug)]
pub enum ServerError
{
    //HTTPError is the only one with String so it can easily convert a http::Error to a ServerError
    HTTPError{msg: String, err: http::Error},
    RequestError{msg: String},
    ResponseError{msg: String},
    PathError{msg: String, path: String},
    MimeTypeError{msg: String, path: String},
    AssembleError{msg: String},
    ThreadError{msg: String}
}

impl fmt::Display for ServerError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            ServerError::HTTPError{msg, err} => {
                write!(f, "[HttpError] {}\nDetails:\r\n{}\r\n", *msg, *err)
            },
            ServerError::RequestError{msg} => {
                write!(f, "[RequestError] {}\r\n", *msg)
            },
            ServerError::ResponseError{msg} => {
                write!(f, "[ResponseError] {}\r\n", *msg)
            },
            ServerError::PathError{msg, path} => {
                write!(f, "[PathError] {}\r\nDetails:\r\n{}\r\n", *msg, *path)
            },
            ServerError::MimeTypeError{msg, path} => {
                write!(f, "[MimeTypeError] {}\r\nDetails:\r\n{}\r\n", *msg, *path)
            }
            ServerError::AssembleError{msg} => {
                write!(f, "[AssembleError] {}\r\n", *msg)
            }
            ServerError::ThreadError{msg} => {
                write!(f, "[ThreadError] {}\r\n", *msg)
            }
        }
    }
}

impl error::Error for ServerError{}
impl From<http::Error> for ServerError{
    fn from(value: http::Error) -> Self {
        ServerError::HTTPError{msg: value.to_string(), err: value}
    }
}

