
use core::panic;
pub use std::fmt::Display;
use std::str::from_utf8;

use rusqlite::{ToSql, params, Row};
use rusqlite::types::{FromSql, FromSqlError, ValueRef::*};

pub use chrono::prelude::*;

use crate::{database::{Database, TableColumnNames}};

pub struct UserProfile{
    pub user_id: u64,
    pub username: String,
    pub sensors: Option<Vec<Sensor>>
}

impl UserProfile{
    pub fn pull_user(user_id: u64, sensor_filter: Option<&[u64]>) -> Option<Self>{
        match UserProfile::pull(user_id, None) {
            Ok(sen) => Some(sen),
            Err(e) => {println!("[Unable to Pull User#{user_id}]:\n{e}"); None}
        }
    }

    //no sensors are filtered
    fn pull(user_id: u64, sensor_filter: Option<&[u64]>) -> Result<UserProfile, rusqlite::Error>{ 
       let sensor_filter = sensor_filter.unwrap_or(&[]);

        let conn = Database::connect();
        let (table, key) = (TableColumnNames::USERS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(
            &format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        Ok::<UserProfile, rusqlite::Error>(
            statement.query_row(params![user_id], 
            |row|{
            Ok(
                UserProfile{
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    sensors: Sensor::pull_sensors(user_id, sensor_filter) // <--array stopping filters
                }
            )
        })?)
    }
}
pub struct Sensor{ 
    pub sensor_id  : u64,
    pub user_id    : u64,
    pub sensor_type: SensorType,
    pub packets    : Vec<DataPacket>
}

pub enum SensorType{
    Moisture,
    Temperature,
    UnknownType 
}
impl Sensor{
    
    pub fn pull_sensors(user_id: u64, sensor_filter: &[u64]) -> Option<Vec<Sensor>>{
        match Sensor::pull(user_id, sensor_filter) {
            Ok(sensors) => Some(sensors),
            Err(_) => None
        }
    }
    fn pull(user_id: u64, sensor_filter: &[u64]) -> Result<Vec<Self>, rusqlite::Error>{
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::SENSORS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(
            &format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        let sensor_iter = statement.query_map(params![user_id], 
            |row|{
                Ok(
                    Sensor{
                        sensor_id: row.get(0)?,
                        sensor_type: row.get(1)?,
                        user_id: row.get(2)?,
                        packets: DataPacket::pull(row.get(0)?)?
                    }
                )
            })?;

        sensor_iter.filter(|sen|{
            sen.as_ref()
                .is_ok_and(|s| sensor_filter.contains(&s.sensor_id))})
                .collect()
    }
}
pub struct DataPacket{
    pub date_time : DateTime<Local>,
    pub frequency: u64,
    pub duration : u64,
    pub amount   : u64,
    pub sensor_id : u64,
}

impl DataPacket{
    fn pull(sensor_id: u64) -> Result<Vec<Self>, rusqlite::Error>{
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::DATA_PACKET, TableColumnNames::SENSOR_ID);
        let mut statement = conn.prepare(
            &format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        let packet_iter = statement.query_map(params![sensor_id], 
            |row|{
                Ok(
                    DataPacket{
                        date_time: row.get(0)?,
                        frequency: row.get(1)?,
                        duration: row.get(2)?,
                        amount: row.get(3)?,
                        sensor_id: sensor_id
                    }
                )
            })?;
        packet_iter.collect()
    }
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
            SensorType::UnknownType => "UnkownType".to_sql()
        }
    }
}

impl FromSql for SensorType{
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let sensor_type = match value{
            Text(st) => from_utf8(st),
            _ => return Err(FromSqlError::InvalidType)
        };

        Ok(match sensor_type{
            Ok("Moisture") => SensorType::Moisture,
            Ok("Temperature") => SensorType::Temperature,
            Ok("UnknownType") => SensorType::UnknownType,
            _ => return Err(FromSqlError::InvalidType)
        })
    }
}


