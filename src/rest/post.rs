use crate::Message;
use http::{self, Method, Request, Response, header, method, status, uri};

pub struct PostMessage{}
impl Message for PostMessage{
    fn response(file_path: &str) -> Result<http::Response<Vec<u8>>, crate::server_error::ServerError> {
        todo!()
    }

    fn request(uri_path: &str) -> Result<http::Response<Vec<u8>>, crate::server_error::ServerError> {
        todo!()
    }

    fn error_response<S: crate::message::Print, E: crate::message::Print>(msg: S, value: E) -> Result<http::Response<Vec<u8>>, crate::server_error::ServerError> {
        todo!()
    }
}