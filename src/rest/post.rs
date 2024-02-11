pub use crate::message::*;
pub use crate::database::*;
use http::response;
use regex::Regex;


// \users\{userid}\                                 pulls up userid profile
// \users\new\                               creates new user profile and pulls the new user profile

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

struct Patterns{}
impl Patterns{
    const USER_OPTIONS: &str = r"\/user/(?<userOptions>new|\d+)";
}

impl Message for PostMessage{
    
    fn process_request(req: Request<String>) -> ResultResponse<Vec<u8>> {
        println!("Processing POST Request...");
        let uri_path = req.uri().path();
        if let Ok(data) = PostMessage::parse_body(req.body()){

        };
        let Some(user_options) = Regex::new(Patterns::USER_OPTIONS).unwrap().captures(uri_path)
            .and_then(|c| c.name("userOptions"))
            .and_then(|m| Some(m.as_str()))
            else {
                return panic!("sigh");//Err(ServerError::RequestError { msg: "POST URI is invalid".to_string() });
            };
        
        match user_options {
            "new" => {
                if let Ok(body) = PostMessage::parse_body(req.body()){
                    let name = body[TableColumnNames::USER_NAME]
                        .as_str().and_then(|n| UserProfile::new(n));
                }
                println!("New User is being created");
                PostMessage::response(r"pages\test_pages\post_forward.html")
            }
            maybe_num => {
                println!("Pulling Profile with user_id={maybe_num}");
                if let Ok(num) = maybe_num.parse::<u64>(){
                }
                PostMessage::response(r"pages\test_pages\user_page.html")
            }   
        }
    }

    fn response(file_path: &str) -> ResultResponse<Vec<u8>> {
        let file_data = fs::read(file_path).unwrap(); //Panic if server pages is not set correctly
        let response = Response::builder()
            .header("Content-Type", "text/html")
            .header("Content-Length", file_data.len())
            .header("Location", r"pages\main_page\index.html")
            .body(file_data);

        match response{
            Ok(r) => Ok(r),
            Err(e) => PostMessage::error_response("Couldn't build Post Response", e)
        }
    }
}