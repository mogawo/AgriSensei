use crate::message::*;
pub struct GetMessage{}

impl Message for GetMessage{
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>>
    {
        return match req.uri().path()
        {
            "/" => GetMessage::response(r"pages\main_page\index.html"),
            "/script.js" => GetMessage::response(r"pages\main_page\script.js"),
            "/style.css" => GetMessage::response(r"pages\main_page\style.css"),
            "/images/cog-xxl.png" => GetMessage::response(r"pages\main_page\images\cog-xxl.png"),
            "/favicon.ico" => GetMessage::response(r"pages\main_page\images\favicon.ico"),
            path => GetMessage::error_response(ServerError::PathError(("Requested Path not Found", path.to_string())))
        };
    }

    fn response(file_path: &str) -> ResultResponse<Vec<u8>>
    {
        let (_, extension) = 
            file_path.split_once('.')
            .ok_or(ServerError::PathError(("Requested File has no extension", file_path.to_string())))?;
        let mime_type = 
            mime_guess::from_ext(extension)
            .first_raw()
            .ok_or(MessageError("Unable to Parse HTTP extensions"))?;
        let file_data = fs::read(file_path)?;

        println!("{}, {}, {}", file_path, mime_type, file_data.len());
        let response = Response::builder()
            .header("Content-Type", mime_type)
            .header("Content-Length", file_data.len())
            .body(file_data);

        Ok(response?)
    }
}