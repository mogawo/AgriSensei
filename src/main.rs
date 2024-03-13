
const LOOP_BACK_ADDRESS: &'static str = "127.0.0.1:7878"; 


fn main(){
    // agrisensei::test_database_init();
    // agrisensei::test_database_add_sensor();
    // agrisensei::test_database_add_packets();
    // let profile = serde_json::to_string_pretty(&agrisensei::pull_user_profile(1).unwrap()).unwrap();
    // println!("{:}", profile);
    agrisensei::start_server_default();
    print!("Server ded")
}

