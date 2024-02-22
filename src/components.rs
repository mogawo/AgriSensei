
use core::panic;
pub use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Chain;
use std::str::from_utf8;

use rusqlite::{params, OptionalExtension, Row, ToSql};
use rusqlite::types::{FromSql, FromSqlError, ValueRef::*};

pub use chrono::prelude::*;
use chrono::serde::ts_seconds;

use crate::{database::{Database, TableColumnNames}};
use std::marker::Copy;
use std::vec::Vec;
use std::option;

use serde::{Serialize, Deserialize};
pub struct UserProfile{
    pub user_id: u32,
    pub username: String,
    pub sensors: Option<Vec<Sensor>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor{ 
    pub sensor_id  : u32,
    pub user_id    : u32,
    pub sensor_type: SensorType,
    pub packets    : Option<Vec<DataPacket>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType{
    Moisture,
    Temperature,
    UnknownType 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPacket{
    #[serde(with = "ts_seconds")]
    pub date_time : DateTime<Utc>,
    pub frequency: u32,
    pub duration : u32,
    pub amount   : u32,
    pub sensor_id : u32,
}
#[derive(Serialize, Deserialize, Debug)]
enum Query{
    TimeRange(TimeRange),
    All,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeRange{
    #[serde(with = "ts_seconds")]
    pub from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub to: DateTime<Utc>
}


impl UserProfile{
    pub fn pull_user(user_id: u32) -> Option<Self>{
        UserProfile::pull(user_id).ok()
    }

    pub fn without(mut self, sensors: &[u32]) -> Self{
        self.sensors = Sensor::pull_sensors(self.user_id, sensors);
        self
    }

    pub fn within(mut self, query: &Option<Query>) -> Self{
        for sen in self.sensors.iter_mut().flatten(){
            sen.within_range(query);
        }
        self
    }

    fn pull(user_id: u32) -> Result<Self, rusqlite::Error>{ 
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::USERS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(&format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        Ok::<UserProfile, rusqlite::Error>(
            statement.query_row(params![user_id], 
            |row|{
            Ok(
                UserProfile{
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    sensors: None // <--array stopping filters
                }
            )
        })?)
    }


}


impl Sensor{    
    pub fn pull_sensors(user_id: u32, sensor_filter: &[u32]) -> Option<Vec<Sensor>>{
        Sensor::pull(user_id, sensor_filter).ok()
    }
    fn pull(user_id: u32, sensor_filter: &[u32]) -> Result<Vec<Self>, rusqlite::Error>{
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
                        packets: Some(DataPacket::pull(row.get(0)?, &None)?)
                    }
                )
            })?;

        sensor_iter.filter(|sen|{
            sen.as_ref()
                .is_ok_and(|s| sensor_filter.contains(&s.sensor_id))})
                .collect()
        
    }
    fn within_range(&mut self, query: &Option<Query>) -> &mut Self{
        self.packets = DataPacket::pull_packets(self.sensor_id, query);
        self
    }
}

// impl Deserializer<Value> for Sensor{

// }

impl DataPacket{
    pub fn pull_packets(sensor_id: u32, query: &Option<Query>) -> Option<Vec<Self>>{
        DataPacket::pull(sensor_id, query).ok()
    }
    fn pull(sensor_id: u32, query: &Option<Query>) -> Result<Vec<Self>, rusqlite::Error>{
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

//Trait Implementations
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


