use chrono::prelude::*;
use serde_json::{json, Value};

use crate::{database::Database, rest::post::PostMessage};

const TEST_NAMES: [&str; 20] = ["Yareli", "Sophie", "Winston", "Norman", 
                                "Kimberly", "Kara", "Juan", "Billy", 
                                "Braulio", "Damien", "Ezra", "Margarita", 
                                "Gisselle", "Leeann", "Davis", "Alex", 
                                "Justin", "Kenna", "Jorden", "Remy"];

pub fn init_database(){
    Database::new();
    for name in TEST_NAMES{
        let json_data = json!({"name" : name}).as_object().unwrap().to_owned();
        PostMessage::new_user(json_data);
    }
}

pub fn add_sensor(){
    for user_id in 1..10{
        let json_data = json!(
            {
                "sensor_type" : "Moisture",
            }
        ).as_object().unwrap().to_owned();

        PostMessage::new_sensor(user_id, json_data);
    }
    for user_id in 11..20{
        let json_data = json!(
            {
                "sensor_type" : "Temperature",
            }
        ).as_object().unwrap().to_owned();

        PostMessage::new_sensor(user_id, json_data);
    }
}

pub fn add_packet(){
    for user_id in 1..20 {
        let time_now = Utc::now();
        let json_data = json!(
            {
                "date_time": time_now,
                "frequency": 5, //seconds
                "duration" : 5, //seconds
                "amount"   : 5, 
            }
        ).as_object().unwrap().to_owned();
        PostMessage::add_packet(user_id, json_data);
    }
}

