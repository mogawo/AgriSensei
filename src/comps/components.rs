
pub use std::fmt::Display;


pub use rusqlite::{params, OptionalExtension, Row, ToSql};
pub use rusqlite::types::{FromSql, FromSqlError, ValueRef::*};

pub use chrono::prelude::*;
pub use chrono::serde::ts_seconds;

pub use crate::{database::{Database, TableColumnNames}};

pub use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Query{
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