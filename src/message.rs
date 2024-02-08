
pub use crate::server_error::*;
pub use http::{self, Method, Request, Response, header, method, status, uri};
pub use std::{
    fmt::{Display, Debug},
    fs
};

pub type ResultResponse<T> = std::result::Result<Response<T>, ServerError>;

pub use serde_json::{Result as JSONResult, Value};
pub trait Message {
    //required methods to overload
    fn process_request(req: Request<String>) -> ResultResponse<Vec<u8>>;
    fn response(file_path: &str) -> ResultResponse<Vec<u8>>;
    fn error_response<S: Print, E: Print>(msg: S, value: E) -> ResultResponse<Vec<u8>>;

    //provided methods
    fn parse_body(body: &String) -> JSONResult<Value>{
        serde_json::from_str(body)
    }
}

//Used in error_response
pub trait Print : Display + Debug {}
impl<T: Display + Debug> Print for T {}


