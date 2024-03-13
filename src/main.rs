
fn main(){
    agrisensei::connect_db();
    let profile  = serde_json::to_string_pretty(&agrisensei::pull_user_profile(1).unwrap()).unwrap();
    println!("{}", profile);
}

fn run_tests() {
    agrisensei::test_database_init();
    agrisensei::test_database_add_sensor();
    agrisensei::test_database_add_packets();
    let profile = serde_json::to_string_pretty(&agrisensei::pull_user_profile(1).unwrap()).unwrap();
    println!("{}", profile);
}