use agrisensei::*;
use regex::Regex;



fn main(){
    // run_tests();
    start_server("127.0.0.1:5500");
    // let uri_path = r"/user/1/";
    // print!("{}", uri_path);
    //     let user_id = Regex::new(r"^\/user\/(?<user_id>\d+)\/?$")
    //         .unwrap()
    //         .captures(uri_path)
    //         .and_then(|cap|cap
    //         .name("user_id"))
    //         .and_then(|mat| mat.as_str().parse::<u64>().ok()).unwrap();
    // print!("\nID: {}", user_id);
}

fn run() {
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