
use crate::ServerError::{self, PathError, MimeTypeError};
use http::{self, Method, Request, Response, header, method, status, uri};
use std::{
    fmt::{Display, Debug},
    fs
};

pub trait Print : Display + Debug {}
impl<T: Display + Debug> Print for T {}
pub trait Message {
    fn request(uri_path: &str) -> Result<http::Response<Vec<u8>>, ServerError>;
    fn response(file_path: &str) -> Result<Response<Vec<u8>>, ServerError>;
    fn error_response<S: Print, E: Print>(msg: S, value: E) -> Result<Response<Vec<u8>>, ServerError>;
}


