use crate::message::*;
use crate::server_error::*;
pub struct GetMessage{}

impl Message for GetMessage{
    fn process_request(req: Request<String>) -> ResultResponse<Vec<u8>>
    {
        return match req.uri().path()
        {
            "/" => GetMessage::response(r"pages\main_page\index.html"),
            "/script.js" => GetMessage::response(r"pages\main_page\script.js"),
            "/style.css" => GetMessage::response(r"pages\main_page\style.css"),
            "/images/cog-xxl.png" => GetMessage::response(r"pages\main_page\images\cog-xxl.png"),
            "/favicon.ico" => GetMessage::response(r"pages\main_page\images\favicon.ico"),
            _ => GetMessage::error_response("Requested Path not Found", ServerError::PathError { msg: "Requested Path not Found".to_string(), path: req.uri().path().to_string() })
        };
    }

    fn response(file_path: &str) -> ResultResponse<Vec<u8>>
    {
        let (_, extension) = file_path.split_once('.').ok_or(ServerError::PathError{msg: "Requested File has no extension".to_string(), path: file_path.to_string()})?;
        let mime_type = mime_guess::from_ext(extension).first_raw().ok_or(ServerError::MimeTypeError{msg: "Requested File Extension has no MIME Type".to_string(), path: file_path.to_string()})?;
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
}