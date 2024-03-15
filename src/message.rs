
pub use crate::server_error::ServerError::{self, *};
pub use http::{self, Method, Request, Response, header, method, status, uri};
use serde_json::Map;
pub use std::{
    fmt::{Display, Debug},
    fs
};

pub type ResultResponse<'a, T> = std::result::Result<Response<T>, ServerError<'a>>;

pub use serde_json::{Result as JSONResult, Value};
pub trait Message {
    //required methods to overload
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>>;

    fn error_response(err: ServerError) -> ResultResponse<Vec<u8>>
    {
        println!("{}", err);
        let err_file = fs::read("pages\\404.html").unwrap();
        let err_response = Response::builder()
            .status(404)
            .header("Content-Type", "text/html")
            .header("Content-Length", err_file.len())
            .body(err_file);
        Ok(err_response?)
    }

    //provided methods
    fn parse_body<S: AsRef<str>>(body: S) -> Result<Map<String, Value>, ServerError<'static>>{
        match serde_json::from_str(body.as_ref()){
            Ok(Value::Object(map)) => Ok(map),
            Ok(_) => Err(MessageError("HTTP Body is not a dictionary")),
            Err(_) => Err(MessageError("Could not parse HTTP Body"))
        }
    }
}


