pub use crate::message::*;
pub use crate::database::*;
use regex::Regex;


// \users\{userid}\                                 pulls up userid profile
// \users\new\profile                               creates new user profile and pulls the new user profile

// \users\{userid}\sensor\{sensorid}                pulls sensor of userid
// \users\{userid}\sensor\new                       creats a new sensor attached to userid
pub struct PostMessage{}
pub struct UserProfile<'u>{
    user_id: u64,
    username: &'u str,
    sensors: Vec<Sensors>
}

pub struct Sensors{
    sensor_id: u64,
    sensor_type: SensorType,
}

enum SensorType{
    Moisture,
    Temperature,
    CustomUOM, 
}

impl<'u> UserProfile<'u>{
    fn new(name: &'u str) -> Option<Self>{
        todo!()
    }
}

impl PostMessage{
    const PATTERN_USER_OPTIONS: &str = r"users\(new|[\d]+|.+)\";
}

struct Patterns{}
impl Patterns{
    const USER_OPTIONS: &str = r"users\(?<user-options>new|[\d]+|.+)\";
}

impl Message for PostMessage{
    
    fn process_request(req: Request<String>) -> ResultResponse<Vec<u8>> {
        let uri_path = req.uri().path();
        if let Ok(data) = PostMessage::parse_body(req.body()){

        };
        let Some(user_options) = Regex::new(Patterns::USER_OPTIONS).unwrap().captures(uri_path)
            .and_then(|c| c.name("user-options"))
            .and_then(|m| Some(m.as_str()))
            else {
                return Err(ServerError::RequestError { msg: "POST URI is invalid".to_string() });
            };
        
        match user_options {
            "new" => {
                if let Ok(body) = PostMessage::parse_body(req.body()){
                    let name = body[TableColumnNames::USER_NAME]
                        .as_str().and_then(|n| UserProfile::new(n));
                }
            }

            maybe_num => {
                if let Ok(num) = maybe_num.parse::<u64>(){
                    todo!("Pull user Profile and send as a Response")
                }
            }   
        }
        
        todo!()
    }

    fn response(file_path: &str) -> ResultResponse<Vec<u8>> {
        todo!()
    }
    
    fn error_response<S: crate::message::Print, E: crate::message::Print>(msg: S, value: E) ->ResultResponse<Vec<u8>>{
        todo!()
    }
}