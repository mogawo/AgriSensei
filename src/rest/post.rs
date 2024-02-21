pub use crate::message::*;
pub use crate::database::*;
use crate::components::*;
use http::response;
use http::StatusCode;
use regex::Regex;
pub use serde_json::{Result as JSONResult, Value};
pub use http::Error as HTTPError;
pub use crate::server_error::ServerError::*;



// \users\{userid}\                                 pulls up userid profile
// \users\new\                               creates new user profile and pulls the new user profile

// \users\{userid}\sensor\{sensorid}                pulls sensor of userid
// \users\{userid}\sensor\new                       creats a new sensor attached to userid
// \users\{userid}\data

pub struct PostMessage{}
struct Patterns{}
impl Patterns{
    const USER_OPTIONS: &'static str = r"\/user\/(?<userOptions>new|\d+)";
}

impl Message for PostMessage{
    
    fn process_request(req: Request<String>) -> ResultResponse<'static, Vec<u8>> {
        println!("Processing POST Request...");
        let uri_path = req.uri().path();
        let Some(user_options) = Regex::new(Patterns::USER_OPTIONS).unwrap().captures(uri_path)
            .and_then(|c| c.name("userOptions"))
            .and_then(|m| Some(m.as_str()))
            else {
                return PostMessage::error_response(ServerError::MessageError(r"No User ID was provided e.g. ..\users\<user_id>\.."));                    
            };
        match user_options {
            "new"     => PostMessage::new_user(PostMessage::parse_body(req.body())?),
            maybe_num =>{
                            let user_id = maybe_num.parse::<u32>()?;
                            let profile = UserProfile::pull_user(user_id);
                            todo!()
                        }   
            }
    }
    fn response(file_path: &str) -> ResultResponse<Vec<u8>> {
        let file_data = fs::read(file_path).unwrap(); //Panic if server pages is not set correctly
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
        let name = 
            json_data[TableColumnNames::USER_NAME]
            .as_str()
            .and_then(|name| Database::new_user(name));
        println!("New User is being created");
        PostMessage::response(r"pages\test_pages\post_forward.html")
    }
}