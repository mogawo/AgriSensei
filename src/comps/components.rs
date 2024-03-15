
pub use std::fmt::Display;


pub use rusqlite::{params, OptionalExtension, Row, ToSql};
pub use rusqlite::types::{FromSql, FromSqlError, ValueRef::*};

pub use chrono::prelude::*;
pub use chrono::serde::ts_seconds;

pub use crate::database::{Database, TableColumnNames};

pub use serde::{Serialize, Deserialize};
pub use crate::server_error::ServerError;

#[derive(Debug)]
pub enum Query{
    SensorFilter(&'static [u64]),
    TimeRange(TimeRange),
    All,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeRange{
    #[serde(with = "ts_seconds")]
    pub from: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub to: DateTime<Utc>
}

pub struct Patterns{}
impl Patterns{
    pub const USER_OPTIONS: &'static str = r"^\/new\/user\/((?<user_id>\d+)\/(?<user_options>sensor|data)\/?)?$";
    pub const GET_USERID: &'static str = r"^\/user\/(?<user_id>\d+)\/?$";
    pub const GET_DEVICE: &'static str = r"^\/user\/(?<user_id>\d+)\/device\/(?<device_id>\d+)?\/?$";
}   