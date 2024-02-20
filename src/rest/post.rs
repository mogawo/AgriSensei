pub use crate::message::*;
pub use crate::database::*;
use crate::components::*;
use http::response;
use http::StatusCode;
use regex::Regex;



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
    
    fn process_request(req: Request<String>) -> ResultResponse<Vec<u8>> {
        println!("Processing POST Request...");
        let uri_path = req.uri().path();
        let Some(user_options) = Regex::new(Patterns::USER_OPTIONS).unwrap().captures(uri_path)
            .and_then(|c| c.name("userOptions"))
            .and_then(|m| Some(m.as_str()))
            else {
                return PostMessage::error_response("[Invalid POST URI]", 
                    ServerError::ResponseError { msg: "Incompatiable URI with server's Regex Pattern".to_string() });                    
            };
        
        match user_options {
            "new" => {
                if let Ok(body) = PostMessage::parse_body(req.body()){
                    let name = body[TableColumnNames::USER_NAME]
                        .as_str().and_then(|name| Database::new_user(name));
                }
                println!("(TODO) New User is being created");
                PostMessage::response(r"pages\test_pages\post_forward.html")
            }
            maybe_num => {
                println!("(TODO) Pulling Profile with user_id={maybe_num}");
                if let Ok(id) = maybe_num.parse::<u64>(){
                    todo!()
                }
                PostMessage::response(r"pages\test_pages\user_page.html")
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
            Err(e) => PostMessage::error_response("Couldn't build Post Response", e)
        }
    }
}