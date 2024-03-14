
use core::panic;
use std::fmt::format;
pub use std::fs::remove_file;
use std::fs::File;
use std::fs::OpenOptions;
pub use std::io::ErrorKind as IOError;
pub use std::io::ErrorKind::NotFound;
pub use std::fmt::{Display, Debug};

use chrono::prelude::*;
pub use rusqlite::{named_params, params, Connection};
pub use rusqlite::Error as SQLError;

pub use crate::comps::{user_profile, sensor, data_packet, device};



#[derive(Debug)]
pub enum DBError{
    SQLError(SQLError),
    IOError(IOError),
}
impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = 100;
        match self{
            
            DBError::SQLError(err) => write!(f, "{}" , format!(">{:=^width$}<\n{:?}", "[SQL Error]", err)),
            DBError::IOError(err) => write!(f, "{}", format!(">{:=^width$}<\n{:?}", "[IO Error]", err)) 
        }
    }
}

impl std::error::Error for DBError{}

impl From<SQLError> for DBError{
    fn from(value: SQLError) -> Self {
        DBError::SQLError(value)
    }
}

impl From<IOError> for DBError{
    fn from(value: IOError) -> Self {
        DBError::IOError(value)
    }
}



pub struct TableColumnNames{}
impl TableColumnNames{
    //User Table
    pub const USERS: &'static str = r"users";
    pub const USER_ID: &'static str = r"user_id";
    pub const USER_NAME: &'static str = r"user_name";

    //Device Table
    pub const DEVICE_TABLE: &'static str = r"devices";
    pub const DEVICE_ID: &'static str = r"device_id";
    pub const VALUE: &'static str = r"value";
    // pub const SENSOR_ID: &'static str = r"sensor_id"; as foreign key
    
    //Sensor Table
    pub const SENSORS: &'static str = r"sensors";
    pub const SENSOR_ID: &'static str = r"sensor_id";
    pub const SENSOR_TYPE: &'static str = r"sensor_type";
    
    //Data Packet
    pub const DATA_PACKET: &'static str = r"data_packet";
    pub const DATE_TIME: &'static str = r"date_time";
    pub const SAMPLE_FREQUENCY: &'static str = r"frequency";
    pub const SAMPLE_DURATION: &'static str = r"duration";
    pub const SAMPLE_AMOUNT: &'static str = r"amount";

    // Data table
    pub const DATA_TABLE: &'static str = r"data_table";
    pub const DATA_POINT: &'static str = r"data_point";
    

    //(Self::USERS, Self::USER_ID, Self::USER_NAME)
    pub fn users_columns() -> (&'static str, &'static str, &'static str){
        (Self::USERS, Self::USER_ID, Self::USER_NAME)
    }

    //(Self::SENSORS, Self::SENSOR_ID, Self::SENSOR_TYPE, Self::USER_ID)
    pub fn sensor_columns() -> (&'static str, &'static str, &'static str, &'static str){
        (Self::SENSORS, Self::SENSOR_ID, Self::SENSOR_TYPE, Self::USER_ID)
    }

    //(Self::DATA_PACKET, Self::DATE_TIME, Self::SAMPLE_FREQUENCY, Self::SAMPLE_DURATION, Self::SAMPLE_AMOUNT, Self::SENSOR_ID)
    pub fn packet_columns() -> (&'static str, &'static str, &'static str, &'static str, &'static str, &'static str){
        (Self::DATA_PACKET, Self::DATE_TIME, Self::SAMPLE_FREQUENCY, Self::SAMPLE_DURATION, Self::SAMPLE_AMOUNT, Self::SENSOR_ID)
    }
}


use TableColumnNames as Col;

use crate::components;


pub struct Database{}



impl<'d> Database{
    const DB_NAME: &'static str = "AgriSensei Database";
    const DB_PATH: &'static str = r"data\agrisensei.db";

    pub fn connect() -> Connection{
        match Connection::open(Database::DB_PATH){
            Ok(conn) => conn,
            Err(_) => panic!("Can not connect to {0} @path=...\\{1}", Database::DB_NAME, Database::DB_PATH)
        }
    }

    pub fn new(){
        let _ = remove_file(Database::DB_PATH);
        //std::fs::write(Database::DB_PATH, "");
        
        let users_table = format!(
            "CREATE TABLE IF NOT EXISTS  [{users}](
                [{userID}]  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                [{userName}] TEXT DEFAULT 'John Smith'
              );",
              users    = Col::USERS,
              userID   = Col::USER_ID,
              userName = Col::USER_NAME, 
            );

              
        let sensor_table = format!("CREATE TABLE IF NOT EXISTS [{sensors}](
            [{sensorID}] INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            [{sensorType}] TEXT NOT NULL,
            [{userID}] INTEGER NOT NULL,
            FOREIGN KEY ({userID}) REFERENCES {users} ({userID})
          );",
          sensors    = Col::SENSORS,
          sensorID   = Col::SENSOR_ID,
          sensorType = Col::SENSOR_TYPE,
          users      = Col::USERS,
          userID     = Col::USER_ID,);
              
        let packet_table = format!("CREATE TABLE IF NOT EXISTS [{dataPacket}](
            [{dateTime}] TEXT NOT NULL PRIMARY KEY,
            [{sampleFrequency}] INTEGER NOT NULL,
            [{sampleDuration}] INTEGER NOT NULL,
            [{sampleAmount}] INTEGER NOT NULL,
            [{sensorID}] INTEGER NOT NULL,
            FOREIGN KEY ({sensorID}) REFERENCES {sensors} ({sensorID})
            );",
            dataPacket      = Col::DATA_PACKET,
            dateTime        = Col::DATE_TIME,
            sampleFrequency = Col::SAMPLE_FREQUENCY,
            sampleDuration  = Col::SAMPLE_DURATION,
            sampleAmount    = Col::SAMPLE_AMOUNT,
            sensors         = Col::SENSORS,
            sensorID        = Col::SENSOR_ID);

        let data_table = format!("CREATE TABLE IF NOT EXISTS [{dataTable}](
            [{dateTime}] TEXT NOT NULL,
            [{dataPoint}] INTEGER NOT NULL
        );",
        dataTable = Col::DATA_TABLE,
        dateTime  = Col::DATE_TIME,
        dataPoint = Col::DATA_POINT);

        let device_table = format!("CREATE TABLE IF NOT EXISTS [{devices}](
            [{userID}] INTEGER NOT NULL,
            [{deviceID}] INTEGER NOT NULL,
            [{sensorID}] INTEGER NOT NULL,
            [{value}] FLOAT NOT NULL,
            FOREIGN KEY ({userID}) REFERENCES {usersForeign} ({userID})
          );",
          devices      = Col::DEVICE_TABLE,
          usersForeign = Col::USERS,
          userID       = Col::USER_ID,
          deviceID     = Col::DEVICE_ID,
          sensorID     = Col::SENSOR_ID,
          value        = Col::VALUE
        );

        let conn = Database::connect();
        conn.execute(&users_table, ()).unwrap();
        conn.execute(&sensor_table, ()).unwrap();
        conn.execute(&packet_table, ()).unwrap();
        conn.execute(&data_table, ()).unwrap();
        conn.execute(&device_table, ()).unwrap();
    }
    //userId auto increments in sqlite
    pub fn new_user(name: &str) -> Option<u64>{
        let conn = Database::connect();
        let user_insert = format!(r"INSERT INTO {user}({userName}) VALUES (?1)", user=Col::USERS, userName=Col::USER_NAME);
        match conn.execute(user_insert.as_str(), params![name]){
                Ok(0) => None,
                Ok(_) => {
                    let user_id = conn.last_insert_rowid().try_into().unwrap();
                    println!("Inserted New User with ID={user_id}"); 
                    Some(user_id)
                },
                Err(e) => panic!("\n[New User] Bad SQL Insert in Database.rs\n{e}\n")
            }
    }

    pub fn new_sensor(sensor_type: sensor::SensorType, user_id: u64) -> Option<u64>{
        let conn = Database::connect();
        let sensor_insert = format!(r"INSERT INTO {sensors}({sensorType}, {userID}) VALUES (?1, ?2)", sensors=Col::SENSORS, sensorType=Col::SENSOR_TYPE, userID=Col::USER_ID);
        match conn.execute(&sensor_insert, params![sensor_type, user_id]){
            Ok(0) => None,
            Ok(_) => {
                let sensor_id = conn.last_insert_rowid().try_into().unwrap();
                println!("Inserted New Sensor#{sensor_id} for User#{user_id}"); 
                Some(sensor_id)
            }
            Err(e)  => panic!("\n[New Sensor] Bad SQL Insert \n{e}\n")
        }
    }

    pub fn add_packet(packet: &data_packet::DataPacket) -> Option<DateTime<Utc>>{
        let conn = Database::connect();
        let packet_insert = format!("INSERT INTO {dataPacket}({dateTime}, {samFreq}, {samDur}, {samAmnt}, {sensorID}) VALUES (?1, ?2, ?3, ?4, ?5)", 
            dataPacket = Col::DATA_PACKET,
            dateTime   = Col::DATE_TIME,
            samFreq    = Col::SAMPLE_FREQUENCY,
            samDur     = Col::SAMPLE_DURATION,
            samAmnt    = Col::SAMPLE_AMOUNT,
            sensorID   = Col::SENSOR_ID
        );

        let points_added = Database::add_data_point(packet.date_time, &packet.data);
        match conn.execute(&packet_insert, //Args need to match order in SQL Table Column index
            params![packet.date_time,
                    packet.frequency,
                    packet.duration,
                    packet.amount,
                    packet.sensor_id]){
            Ok(0) => {println!("Same Packet was Inputed, packet was not stored"); return None;},
            Err(e) => panic!("\n[Add Packet] Bad SQL Insert\n{e}\n"),
            Ok(_) => println!("Packet with {} points for Sensor {} was added", points_added, packet.sensor_id)
        }
        Some(packet.date_time)
    }

    pub fn add_data_point(date_time: DateTime<Utc>, data: &Vec<u64>) -> u64{
        let conn = Database::connect();
        let point_insert = format!("INSERT INTO {dataTable}({dateTime}, {dataPoint}) VALUES (?1, ?2)",
            dataTable = Col::DATA_TABLE,
            dateTime  = Col::DATE_TIME,
            dataPoint = Col::DATA_POINT);
        
        let mut rows_added = 0;
        for point in data{
            match conn.execute(&point_insert, params![date_time, point]){
                Ok(0) => print!("Same Data Point was Inputed, data point was not stored"),
                Err(e) => panic!("\n[Add Data Point] Bad SQL Insert\n{e}\n"),
                Ok(_) => rows_added+=1
            }
        }
        rows_added
    }

    pub fn add_device_measurements(device: &device::Device){
        // print!("{:#?}", device);
        let conn = Database::connect();
        let device_insert = format!("INSERT INTO {deviceTable}({userID}, {deviceID}, {sensorID}, {value}) VALUES (?1, ?2, ?3, ?4)", 
            deviceTable = Col::DEVICE_TABLE,
            userID      = Col::USER_ID,
            deviceID    = Col::DEVICE_ID,
            sensorID    = Col::SENSOR_ID,
            value       = Col::VALUE
        );

        for sen in &device.sensors{
            match conn.execute(&device_insert, params![device.user_id, device.device_id, sen.sensor_id, sen.value]){
                Ok(0) => {println!("Same Device{} was Inputed, measurement was not stored", device.device_id); },
                Ok(_) => {println!("Device{} was added for Sensor{}", device.device_id, sen.sensor_id)},
                Err(e) => panic!("\n[Add Device Measurement Device Insert] Bad SQL Insert\n{e}\n"),
            };
        }
    }
}