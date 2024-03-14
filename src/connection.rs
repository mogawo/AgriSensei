use crate::*;

use self::rest::get::GetMessage;

pub fn connect_db(){
    let c = Database::connect();
    // GetMessage::process_request(req);

}
