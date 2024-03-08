use std::str::FromStr;

use crate::message::*;
use crate::components::Patterns;
use crate::user_profile::UserProfile;
use regex::Regex;
use serde_json::json;

use super::post::{PostMessage, Query};
// -GET REQUESTS-
// GET /user/<user_id>/ HTTP/1.1          Returns ALL data about a userprofile
// <no body or header needed>

// -SERVER RESPONSE- you can run this files main.rs to get an example json response as well
// HTTP/1.1 200
// Content-Type: application/json
// Content-Length: ## of bytes of json body
// {
//     "user_id": 1,
//     "username": "Yareli",
//     "sensors": [
//       {
//         "sensor_id": 1,
//         "user_id": 1,
//         "sensor_type": "Moisture",
//         "packets": [
//           {
//             "date_time": "2024-03-08T07:22:50.109534300Z",
//             "frequency": 5,
//             "duration": 5,
//             "amount": 5,
//             "sensor_id": 1
//           },
//         ]
//       },
//     ]
//   }

pub struct GetMessage{}
impl Message for GetMessage{
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>>
    {
        let uri_path = req.uri().path();

        //If section returns json of user profile
        // SUPPOSED to return back a html webpage, but this should be
        // fine for now
        if let Some(user_id) = Regex::new(Patterns::USER_OPTIONS)
            .unwrap()
            .captures(uri_path)
            .and_then(|cap|cap
            .name("user_id"))
            .and_then(|mat| mat.as_str().parse::<u64>().ok()){
                let profile = UserProfile::pull_user(user_id).unwrap()
                    .include(&Query::All)
                    .within(&Query::All);

                let profile = serde_json::to_vec(&profile).unwrap();
                GetMessage::response_data(profile)
            } else {
                match uri_path
                {
                    "/" => GetMessage::response(r"pages\main_page\index.html"),
                    "/script.js" => GetMessage::response(r"pages\main_page\script.js"),
                    "/style.css" => GetMessage::response(r"pages\main_page\style.css"),
                    "/images/cog-xxl.png" => GetMessage::response(r"pages\main_page\images\cog-xxl.png"),
                    "/favicon.ico" => GetMessage::response(r"pages\main_page\images\favicon.ico"),
                    _ => GetMessage::error_response(ServerError::PathError(("Requested Path not Found", uri_path.to_string())))
                }
            }
    }
}

impl GetMessage{
     fn response<S: AsRef<str>>(file_path: S) -> ResultResponse<'static, Vec<u8>>
    {
        let (_, extension) = 
            file_path.as_ref().split_once('.')
            .ok_or(ServerError::PathError(("Requested File has no extension", file_path.as_ref().to_string())))?;
        let mime_type = 
            mime_guess::from_ext(extension)
            .first_raw()
            .ok_or(MessageError("Unable to Parse HTTP extensions"))?;
        let file_data = fs::read(file_path.as_ref())?;

        println!("{}, {}, {}", file_path.as_ref(), mime_type, file_data.len());
        let response = Response::builder()
            .header("Content-Type", mime_type)
            .header("Content-Length", file_data.len())
            .body(file_data);

        Ok(response?)
    }

    fn response_data(body: Vec<u8>) -> ResultResponse<'static, Vec<u8>>{
        Ok(Response::builder()
                    .header("Content-Type", "application/json")
                    .header("Content-Length", body.len())
                    .body(body)?)
    }
}