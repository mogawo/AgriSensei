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
        let json_data = json!({"user_name" : name}).as_object().unwrap().to_owned();
        PostMessage::new_user(json_data);
    }
}

pub fn add_sensor(){
    let json_data = json!(
        {
            "sensor_type" : "Moisture",
        }
    ).as_object().unwrap().to_owned();

    PostMessage::new_sensor(1, json_data);

    let json_data = json!(
        {
            "sensor_type" : "Temperature",
        }
    ).as_object().unwrap().to_owned();

    PostMessage::new_sensor(1, json_data);
}

pub fn add_packet(){
    for sensor_id in 1..=2 {
        for _ in 0..3{
            let time_now = Utc::now();
            let json_data = json!(
                {
                    "date_time": time_now,
                    "frequency": 5,           //seconds
                    "duration" : 5,           //seconds
                    "amount"   : 5,
                    "sensor_id": sensor_id,
                    "data"     : [0,1,2,3,4,5,6,7,8,9]
                }
            ).as_object().unwrap().to_owned();
            PostMessage::add_packet(1, json_data).unwrap();
        }
    }
}

