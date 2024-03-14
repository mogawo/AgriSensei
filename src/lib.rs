use std::net::TcpListener;

pub mod server_error;
use components::Query;
pub use serde_json::json;
use server_error::ServerError;

mod thread_pool;
use thread_pool::ThreadPool;

pub mod message;

mod rest{
    pub mod get;
    pub mod post;
}
// use rest::get;

mod database;
use database::Database;

mod handler;
use handler::handle_connection;

pub mod comps{
    pub mod components;
    pub mod data_packet;
    pub mod sensor;
    pub mod user_profile;
}
use comps::*;
use chrono::prelude::*;

mod testing;

pub use comps::user_profile::UserProfile;
pub use comps::sensor::{Sensor, SensorType};
pub use comps::data_packet::DataPacket;

// pub use serde_json;

pub struct ServerAdress{
    pub host_address: &'static mut str
}

//Starts a local server with a given ip string
pub fn start_server(host_address: &'static str){
    let listener = TcpListener::bind(host_address).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {handle_connection(stream);});
    }
}

//Deletes the current database
//Creates a database with 20 default user profile names (Check /data/agrisensei.db)
pub fn test_database_init(){
    testing::init_database();
}

//Init test for adding sensors (Check /data/agrisensei.db)
pub fn test_database_add_sensor(){
    testing::add_sensor();
}

//Init tests adding packets (Check /data/agrisensei.db)
pub fn test_database_add_packets(){
    testing::add_packet();
}

//DELETES current database and init a new database
pub fn new_database(){
    Database::new();
}

//Addes new user to database. Needs the Database to be init first
//Optional value is the user_id
pub fn new_user(name: &str) -> Option<u64>{
    Database::new_user(name)
}

//Addes new sensor to database. Needs the Database to be init first
//Optional value is the sensor_id
pub fn new_sensor(sensor_type: SensorType, user_id: u64) -> Option<u64>{
    Database::new_sensor(sensor_type, user_id)
}

//Addes new packet to database. Needs the Database to be init first
//Optional value is the the chrono::DateTime<Utc> which is the primary
// key for the data packet
pub fn add_packet(packet: &DataPacket) -> Option<chrono::DateTime<Utc>>{
    Database::add_packet(packet)
}

// Pulls all information from a profile with given
// user_id. This function can return Option::None which
// means that the user does not exist within the Database
pub fn pull_user_profile(user_id: u64) -> Option<UserProfile>{
    Some(UserProfile::pull_user(user_id)?.include(&Query::All).within(&Query::All))
}