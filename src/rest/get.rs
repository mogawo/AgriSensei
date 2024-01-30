
use crate::ServerError::{self, PathError, MimeTypeError};
use std::{
    fmt::{Display, Debug},
    fs
};
use http::{self, Method, Request, Response, header, method, status, uri};

pub fn request(uri_path: &str) -> Result<http::Response<Vec<u8>>, ServerError>
{
    return match uri_path
    {
        "/" => response("pages\\main_page\\index.html"),
        "/script.js" => response("pages\\main_page\\script.js"),
        "/style.css" => response("pages\\main_page\\style.css"),
        "/images/cog-xxl.png" => response("pages\\main_page\\images\\cog-xxl.png"),
        "/favicon.ico" => response("pages\\main_page\\images\\favicon.ico"),
        _ => error_response("Requested Path not Found", ServerError::PathError { msg: "Requested Path not Found".to_string(), path: uri_path.to_string() })
    };
}

pub fn response(file_path: &str) -> Result<Response<Vec<u8>>, ServerError>
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

pub trait Print : Display + Debug {}
impl<T: Display + Debug> Print for T {}

pub fn error_response<S: Print, E: Print>(msg: S, value: E) -> Result<Response<Vec<u8>>, ServerError>
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