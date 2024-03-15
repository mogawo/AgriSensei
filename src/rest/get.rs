use std::fmt::format;

use serde_json::json;

use crate::{components::TableColumnNames, message::*, Device};

use super::post::Database;

// -GET REQUESTS-

// GET /user/<user_id>/<device_id> HTTP/1.1          Returns ALL data about a userprofile
// <no body or header needed>

// -SERVER RESPONSE- you can run this files main.rs to get an example json response as well
// HTTP/1.1 200
// Content-Type: application/json
// Content-Length: ## of bytes of json body
// {
//   {
//   "user_id": 1,
//   "device_id": 1,
//   "sensors": [
//     {
//       "sensor_id": 1,
//       "value": 1.0
//     },
//     {
//       "sensor_id": 1,
//       "value": 1.0
//     },
//     {
//       "sensor_id": 1,
//       "value": 1.0
//     },
//     {
//       "sensor_id": 1,
//       "value": 1.0
//     }]
//   }

pub struct GetMessage{}
impl Message for GetMessage{
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>>
    {
        let uri_path = req.uri().path();
         
           match uri_path
                {
                  "/last_user_id" => {
                        let conn = Database::connect();
                        let statement = format!("SELECT MAX({}) FROM {}", TableColumnNames::USER_ID, TableColumnNames::USERS);
                        let last_user_id: i64 = conn.query_row(&statement, [], |row| row.get(0)).unwrap();
                        let json_data = json!({
                            "last_user_id" : last_user_id
                        }).to_string().into_bytes();
                        GetMessage::response_data(json_data)
                    },
                    r"/pages/main_page/index.html" => GetMessage::response(r"pages\main_page\index.html"),
                    r"/pages/main_page/index.html/ws" => GetMessage::response(r"pages\main_page\index.html"),
                    r"/pages/main_page/script.js" => GetMessage::response(r"pages\main_page\script.js"),
                    r"/pages/main_page/script.js/ws" => GetMessage::response(r"pages\main_page\script.js"),
                    r"/pages/main_page/style.css" => GetMessage::response(r"pages\main_page\style.css"),
                    r"/pages/main_page/style.css/ws" => GetMessage::response(r"pages\main_page\style.css"),
                    r"/pages/login/index.html" => GetMessage::response(r"pages\login\index.html"),
                    r"/pages/login/index.html/ws" => GetMessage::response(r"pages\login\index.html"),
                    r"/pages/login/script.js" => GetMessage::response(r"pages\login\script.js"),
                    r"/pages/login/script.js/ws" => GetMessage::response(r"pages\login\script.js"),
                    r"/pages/login/style.css" => GetMessage::response(r"pages\login\style.css"),
                    r"/pages/login/style.css/ws" => GetMessage::response(r"pages\login\style.css"),
                    r"/pages/main_page/images/cog-xxl.png" => GetMessage::response(r"pages\main_page\images\cog-xxl.png"),
                    r"/pages/main_page/images/pencil.png" => GetMessage::response(r"pages\main_page\images\pencil.png"),
                    r"/pages/main_page/images/favicon.ico" => GetMessage::response(r"pages\main_page\images\favicon.ico"),
                    r"/pages/main_page/jscharting/JSC/jscharting.js" => GetMessage::response(r"pages\main_page\jscharting\JSC\jscharting.js"),
                    r"/pages/main_page/jscharting/JSC/modules/debug.js" => GetMessage::response(r"pages\main_page\jscharting\JSC\modules\debug.js"),
                    _                       => 
                    
                    {
                        let other_path: Vec<&str> = uri_path.split('/').collect();
                        let user_id = other_path.get(2);
                        let device_id = other_path.get(3);
                        if let (Some(ur_id), Some(dev_id)) = (user_id, device_id) {
                            let device = Device::pull_device(ur_id.parse::<u64>().unwrap(), dev_id.parse::<u64>().unwrap()).unwrap();
                            return GetMessage::response_data(device.to_json().into_bytes());
                    } else {
                        GetMessage::error_response(ServerError::PathError(("Requested Path not Found", uri_path.to_string())))}
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

        // println!("{}, {}, {}", file_path.as_ref(), mime_type, file_data.len());
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