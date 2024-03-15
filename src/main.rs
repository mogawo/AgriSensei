
fn main(){
    agrisensei::test_database_init();
    
    // agrisensei::test_database_add_sensor();
    // agrisensei::test_database_add_packets();
    // println!("{}", agrisensei::pull_user_profile(1));
    agrisensei::test_add_measurements();
    // println!("{}", agrisensei::pull_device(1, 1));
    print!("Pulling devices\n{}", agrisensei::pull_devices(1).to_json());
    agrisensei::start_server("127.0.0.1:5500");
    // agrisensei::start_server_default();
}
