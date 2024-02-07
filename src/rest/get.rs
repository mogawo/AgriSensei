use crate::{
    Message::{self},
    ServerError::{self, PathError, MimeTypeError},
};

use crate::message::Print;
use http::{self, Method, Request, Response, header, method, status, uri};
use std::{
    fmt::{Display, Debug},
    fs
};
pub struct GetMessage{}

impl Message for GetMessage{
    fn request(uri_path: &str) -> Result<http::Response<Vec<u8>>, ServerError>
    {
        return match uri_path
        {
            "/" => GetMessage::response("pages\\main_page\\index.html"),
            "/script.js" => GetMessage::response("pages\\main_page\\script.js"),
            "/style.css" => GetMessage::response("pages\\main_page\\style.css"),
            "/images/cog-xxl.png" => GetMessage::response("pages\\main_page\\images\\cog-xxl.png"),
            "/favicon.ico" => GetMessage::response("pages\\main_page\\images\\favicon.ico"),
            _ => GetMessage::error_response("Requested Path not Found", ServerError::PathError { msg: "Requested Path not Found".to_string(), path: uri_path.to_string() })
        };
    }

    fn response(file_path: &str) -> Result<Response<Vec<u8>>, ServerError>
    {
        let (_, extension) = file_path.split_once('.').ok_or(PathError{msg: "Requested File has no extension".to_string(), path: file_path.to_string()})?;
        let mime_type = mime_guess::from_ext(extension).first_raw().ok_or(MimeTypeError{msg: "Requested File Extension has no MIME Type".to_string(), path: file_path.to_string()})?;
        let file_data = match fs::read(file_path){
            Ok(data) => data,
            Err(e) => return Err(ServerError::ResponseError { msg: format!("{}\nCould not open file", e) })
        };

        println!("{}, {}, {}", file_path,mime_type, file_data.len());
        let response = Response::builder()
            .header("Content-Type", mime_type)
            .header("Content-Length", file_data.len())
            .body(file_data);

        Ok(response?)
    }

    fn error_response<S: Print, E: Print>(msg: S, value: E) -> Result<Response<Vec<u8>>, ServerError>
    {
        println!("{}: {}", msg, value);
        let err_file = fs::read("pages\\404.html").unwrap();
        let err_response = Response::builder()
            .status(404)
            .header("Content-Type", "text/html")
            .header("Content-Length", err_file.len())
            .body(err_file);
        Ok(err_response?)
    }
}