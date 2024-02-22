pub use crate::message::*;
pub use crate::database::*;
pub use crate::components::*;
pub use http::response;
pub use http::StatusCode;
pub use regex::Regex;
pub use serde_json::json;
pub use serde_json::value;
pub use serde_json::{Result as JSONResult, Value};
pub use http::Error as HTTPError;
pub use crate::server_error::ServerError::*;
pub use crate::comps::{sensor::Sensor};


  // /user/{userid}/                                 pulls up userid profile
  // /user/new/                               creates new user profile and pulls the new user profile

  // /user/{userid}/sensor/{sensorid}                pulls sensor of userid
  // /user/{userid}/sensor/new                       creats a new sensor attached to userid
  // /user/{userid}/data

pub struct PostMessage{}
struct Patterns{}
//TODO - Seperate Regex into 
// /user/new
// /user/##/sensor/new
// /user/##/sensor/data/new            Body will have the data
// /user/##/sensor/?=sen<#-#-...-#>&&data<> (IMPLEMENT ONLY IF HAVE TIME)
impl Message for PostMessage{
    
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>> {
        
        println!("Processing POST Request...");
        let uri_path = req.uri().path();
        //TODO Deal with html headers for POST
        let json_data = PostMessage::parse_body(req.body())?;
        // match uri_path{
        //     "/user/new" => PostMessage::new_user(json_data),
        //     "/user/sensor/new" => , 
        // }\
        todo!()
    }

    fn response<S: AsRef<str>>(file_path: S) -> ResultResponse<'static, Vec<u8>> {
        let file_data = fs::read(file_path.as_ref()).unwrap(); //Panic if server pages is not set correctly
        let response = Response::builder()
            .status(302)
            .header("Content-Type", "text/html")
            .header("Content-Length", file_data.len())
            .header("Location", format!(r"{host}\pages\main_page\index.html", host=crate::HOST_ADDRESS))
            .body(file_data);

        match response{
            Ok(r) => Ok(r),
            Err(e) => PostMessage::error_response(ServerError::HTTPError(e))
        }
    }
}

impl PostMessage{
    fn new_user(json_data: Value) -> ResultResponse<'static, Vec<u8>>{
        println!("New User is being created");
        let Some(name) = json_data[TableColumnNames::USER_NAME].as_str() else{
            return PostMessage::error_response(MessageError("No Name for User was Provided"));
        };
        let Some(user_id) = Database::new_user(name) else {
            return PostMessage::error_response(MessageError("Could Not Insert New User into Database"));
        };
        PostMessage::response(format!(r"/user/{user_id}"))
    }

    fn new_sensor(json_data: Value) -> ResultResponse<'static, Vec<u8>>{
        println!("New Sensor is being created");
        let sensor: Sensor = serde_json::from_value(json_data)?;
        println!("{:?}", sensor);
        todo!()
    }
}