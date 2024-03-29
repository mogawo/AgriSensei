use serde_json::json;

use crate::{message::*, Device};

use super::post::Database;
use crate::database::TableColumnNames;

// -GET REQUESTS-
// GET /user/<user_id>/<device_id> HTTP/1.1          Returns ALL device data from a user
// <no body or header needed>

// -SERVER RESPONSE- you can run this files main.rs to get an example json response as well
// HTTP/1.1 200
// Content-Type: application/json
// Content-Length: ## of bytes of json body
// {
//     "user_id": 1,
//     "devices": [
//       {
//         "user_id": 1,
//         "device_id": 1,
//         "sensors": [
//           {
//             "sensor_id": 1,
//             "value": 2.0,
//             "date_time": "2024-03-15T04:21:38.553920300Z"
//           },
//           {
//             "sensor_id": 1,
//             "value": 3.0,
//             "date_time": "2024-03-15T04:21:38.558603Z"
//           },
//           {
//             "sensor_id": 1,
//             "value": 4.0,
//             "date_time": "2024-03-15T04:21:38.563353900Z"
//           }
//         ]
//     }
// }
pub struct GetMessage{}
impl Message for GetMessage{
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>>
    {
        let uri_path = req.uri().path();
           match uri_path
                {
                    "/last_user_id" => {
                        let conn = Database::connect();
                        let statement = format!("SELECT MAX({}) FROM {};", TableColumnNames::USER_ID, TableColumnNames::USERS);
                        let last_user_id: i64 = conn.query_row(&statement, [], |row| row.get(0)).unwrap();
                        let json_data = json!({
                            "last_user_id" : last_user_id
                        }).to_string().into_bytes();
                        GetMessage::response_data(json_data)
                    },
                    r"/pages/main_page/index.html" => GetMessage::response(r"pages/main_page/index.html"),
                    r"/pages/main_page/index.html/ws" => GetMessage::response(r"pages/main_page/index.html"),
                    r"/pages/main_page/script.js" => GetMessage::response(r"pages/main_page/script.js"),
                    r"/pages/main_page/script.js/ws" => GetMessage::response(r"pages/main_page/script.js"),
                    r"/pages/main_page/style.css" => GetMessage::response(r"pages/main_page/style.css"),
                    r"/pages/main_page/style.css/ws" => GetMessage::response(r"pages/main_page/style.css"),
                    r"/pages/login/index.html" => GetMessage::response(r"pages/login/index.html"),
                    r"/pages/login/index.html/ws" => GetMessage::response(r"pages/login/index.html"),
                    r"/pages/login/script.js" => GetMessage::response(r"pages/login/script.js"),
                    r"/pages/login/script.js/ws" => GetMessage::response(r"pages/login/script.js"),
                    r"/pages/login/style.css" => GetMessage::response(r"pages/login/style.css"),
                    r"/pages/login/style.css/ws" => GetMessage::response(r"pages/login/style.css"),
                    r"/pages/main_page/images/cog-xxl.png" => GetMessage::response(r"pages/main_page/images/cog-xxl.png"),
                    r"/pages/main_page/images/pencil.png" => GetMessage::response(r"pages/main_page/images/pencil.png"),
                    r"/favicon.ico" => GetMessage::response(r"pages/main_page/images/favicon.ico"),
                    r"/pages/main_page/jscharting/JSC/jscharting.js" => GetMessage::response(r"pages/main_page/jscharting/JSC/jscharting.js"),
                    r"/pages/main_page/jscharting/JSC/modules/debug.js" => GetMessage::response(r"pages/main_page/jscharting/JSC/modules/debug.js"),
                    _ => {
                        let other_path: Vec<&str> = uri_path.split('/').collect();
                        let device_id = other_path.get(2).unwrap().parse::<i64>();
                        if let Ok(ur_id) = device_id {
                            let device = Device::pull_device(ur_id).unwrap();
                            return GetMessage::response_data(device.to_json().into_bytes());
                        } else {
                            return GetMessage::error_response(MessageError("Get Request Error"))
                        }
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