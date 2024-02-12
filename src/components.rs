
pub use std::fmt::Display;

pub use rusqlite::ToSql;

pub use chrono::prelude::*;



pub struct UserProfile<'u>{
    user_id: u64,
    username: &'u str,
    sensors: Vec<Sensors>
}

impl<'u> UserProfile<'u>{
    pub fn new(name: &'u str) -> Option<Self>{
        todo!()
    }
}

pub struct Sensors{
    sensor_id: u64,
    sensor_type: SensorType,
}

pub enum SensorType{
    Moisture,
    Temperature,
    UnknownType 
}

pub struct DataPacket{
    dateTime: DateTime<Local>,
    frequency: u64,
    duration: u64,
    amount: u64,
    userID: u64,
    sensorID: u64,
}

impl Display for SensorType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            SensorType::Moisture => write!{f, "Moisture"},
            SensorType::Temperature => write!{f, "Temperature"},
            SensorType::UnknownType =>  write!{f, "UnkownType"}
        }
    }
}

impl ToSql for SensorType{
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self{
            SensorType::Moisture => "Moisture".to_sql(),
            SensorType::Temperature => "Temperature".to_sql(),
            SensorType::UnknownType =>  "UnkownType".to_sql()
        }
    }
}

