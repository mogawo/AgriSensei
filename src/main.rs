use agrisensei::message::Message;
use http::Request;


fn main(){
    agrisensei::test_database_init();
    
    // agrisensei::test_database_add_sensor();
    // agrisensei::test_database_add_packets();
    // println!("{}", agrisensei::pull_user_profile(1));
    // agrisensei::test_add_measurements();
    // println!("{}", agrisensei::pull_device(1, 1));
    print!("{:#?}", agrisensei::GetMessage::process_request(Request::new(String::new())));
    agrisensei::start_server_default();
}
