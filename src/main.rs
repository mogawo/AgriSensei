

fn main(){
    agrisensei::test_database_init();
    agrisensei::start_server_default();
    // agrisensei::test_database_add_sensor();
    // agrisensei::test_database_add_packets();
    // let profile = serde_json::to_string_pretty(&agrisensei::pull_user_profile(1).unwrap()).unwrap();
    // println!("{}", profile);
    agrisensei::test_add_measurements();
    println!("{}", agrisensei::pull_device(1, 1))
}
