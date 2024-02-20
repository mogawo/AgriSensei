
use core::panic;
pub use std::fmt::Display;
use std::str::from_utf8;

use rusqlite::{ToSql, params, Row};
use rusqlite::types::{FromSql, FromSqlError, ValueRef::*};

pub use chrono::prelude::*;

use crate::{database::{Database, TableColumnNames}, message::ServerError};

pub struct UserProfile{
    pub user_id: u64,
    pub username: String,
    pub sensors: Vec<Sensor>
}

impl UserProfile{
    pub fn pull(user_id: u64) -> Option<Self>{
        match UserProfile::user_pull(user_id){
            Ok(profile) => Some(profile),
            Err(e) => {println!("{:?}", e); return None;}
        }
    }

    fn user_pull(user_id: u64) -> Result<UserProfile, rusqlite::Error>{
        let conn = Database::connect();

        let (table, key) = (TableColumnNames::USERS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(
            &format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        let mut profile = Ok::<UserProfile, rusqlite::Error>(statement.query_row(params![user_id], 
            |row|{
            Ok(
                UserProfile{
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    sensors: Vec::new()
                }
            )
        })?)?;

        Ok(profile)

    }

    
}

impl Sensor{
    fn pull(&mut self, user_id: u64) -> Result<Vec<Self>, rusqlite::Error>{
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::SENSORS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(
            &format!("SELECT * FROM {table} WHERE {key} = (?1)", ))?;
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

        sensor_iter.collect()
    }
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

pub struct DataPacket{
    pub date_time : DateTime<Local>,
    pub frequency: u64,
    pub duration : u64,
    pub amount   : u64,
    pub sensor_id : u64,
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



