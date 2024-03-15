

use crate::data_packet::DataPacket;
pub use crate::message::*;
pub use crate::database::*;
pub use crate::components::*;
use crate::sensor::SensorType;
use crate::Device;
pub use http::response;
pub use http::StatusCode;
pub use regex::Regex;
pub use serde_json::json;
pub use serde_json::value;
pub use serde_json::{Result as JSONResult, Value};
pub use http::Error as HTTPError;
pub use crate::server_error::ServerError::*;
pub use crate::comps::{sensor::Sensor, components::Patterns};

// -POST REQUESTS-
// 
// /new/user/                    creates new user profile
// /new/user/<user_id>/sensor/   creates new sensor for user_id
// /new/user/<user_id>/data/     creates new data packet for user_id

// POST /new/user/<user_id>/sensor/ HTTP/1.1

// POST /new/user/<user_id>/sensor/ HTTP/1.1
// {
//     "sensor_type" : "Temperature" or "Moisture"
// }

// POST /new/user/<user_id>/data/ HTTP/1.1
// {
//     "date_time": time_now,
//     "frequency": 5, //int seconds
//     "duration" : 5, //int seconds
//     "amount"   : 5,
//     "sensor_id": sensor_id 
// }

// -SERVER RESPONSE- 
//
// HTTP/1.1 200
// Location: /user/<user_id>/            <- uri path where user should be redirected too; 
//                                          need to added htmx path to get.rs match cases

// POST /new/user/<user_id>/measurements/ HTTP/1.1
// {
//   "device_id": 1,
//   "sensors": [
//     {
//       "sensor_id": 1,
//       "value": 2.0
//     },
//     {
//       "sensor_id": 1,
//       "value": 3.0
//     }
//   ]
// }


pub struct PostMessage{}
impl Message for PostMessage{
    
     fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>> {
        
        println!("Processing POST Request...");
        
        let uri_path = req.uri().path();
        let json_data = PostMessage::parse_body(req.body())?;
        let uri_capture = Regex::new(Patterns::GET_USERID)
            .unwrap()
            .captures(uri_path)
            .ok_or(MessageError("URI does not match Regex pattern"))?;

        let user_id = uri_capture.name("user_id");
        let user_options = uri_capture.name("user_options");

        match (user_id, user_options){
            (None, None) =>  PostMessage::new_user(json_data),   
            (Some(id), Some(opt)) => match opt.as_str(){
                "sensor" => PostMessage::new_sensor(id.as_str().parse()?, json_data),
                "data" => PostMessage::add_packet(id.as_str().parse()?, json_data),
                "measurements" => PostMessage::add_measurements(id.as_str().parse()?, json_data),
                    _ => PostMessage::error_response(MessageError("Invalid User Options {sensor, data}"))
            },
            (_, _) => PostMessage::error_response(MessageError("Modifying Server Components have not been implemented yet"))
        }
    }
}

impl PostMessage{
    pub  fn response<S: AsRef<str>>(file_path: S, location: S) -> ResultResponse<'static, Vec<u8>> {
        let file_data = fs::read(file_path.as_ref()).unwrap(); //Panic if server pages is not set correctly
        let response = Response::builder()
            .status(302)
            .header("Content-Type", "text/html")
            .header("Content-Length", file_data.len())
            .header("Location", format!(r"{locat}", locat=location.as_ref()))
            .body(file_data);

        match response{
            Ok(r) => Ok(r),
            Err(e) => PostMessage::error_response(ServerError::HTTPError(e))
        }
    }
    pub  fn new_user(json_data: serde_json::Map<String, Value>) -> ResultResponse<'static, Vec<u8>>{
        println!("New User is being created");
        let Some(name) = json_data.get(TableColumnNames::USER_NAME).and_then(|n| n.as_str()) else{
            return PostMessage::error_response(MessageError("No Name for User was Provided"));
        };
        let Some(user_id) = Database::new_user(name) else {
            return PostMessage::error_response(MessageError("Could Not Insert New User into Database"));
        };
        PostMessage::response(r"pages\test_pages\sensor_confirm.html", &format!(r"/user/{user_id}")) 
    }

    pub  fn new_sensor(user_id: u64, json_data: serde_json::Map<String, Value>) -> ResultResponse<'static, Vec<u8>>{
        println!("New Sensor is being created");
        let sensor_type: SensorType = json_data.get(TableColumnNames::SENSOR_TYPE)
            .and_then(|sen| sen.as_str())
            .unwrap_or("UnknownType")
            .into();

        if matches!(sensor_type, SensorType::UnknownType){ return Err(MessageError("No Sensor Type Provided"))};
        let sensor_id = Database::new_sensor(sensor_type, user_id).ok_or(MessageError("Could not make a new Sensor"))?;
        PostMessage::response(r"pages\test_pages\sensor_confirm.html", &format!(r"/user/{user_id}/sensor/{sensor_id}"))
    }

    pub  fn add_packet(user_id: u64, json_data: serde_json::Map<String, Value>) -> ResultResponse<'static, Vec<u8>> {
        println!("Adding Data Packets...");
        let packet: DataPacket = serde_json::from_value(Value::Object(json_data))?;
        packet.push_packet()?;
        PostMessage::response(r"pages\test_pages\added_packet_confirm.html", &format!(r"/user/{user_id}/"))
    }

    pub fn add_measurements(user_id: u64, json_data: serde_json::Map<String, Value>) -> ResultResponse<'static, Vec<u8>>{
        println!("Adding Measurements...");
        let mut with_usersid: serde_json::Map<String, Value> = json!({
            TableColumnNames::USER_ID: user_id
        }).as_object().unwrap().clone();

        with_usersid.extend(json_data);
        let measure: Device = serde_json::from_value(Value::Object(with_usersid)).unwrap();
        measure.push_device();
        PostMessage::response(r"pages\test_pages\measurement_confirm.html", "/measurement_wip/")
    }
}