
pub use std::fs::remove_file;
pub use std::io::ErrorKind as IOError;
pub use std::io::ErrorKind::NotFound;
pub use std::fmt::{Display, Debug};

pub use rusqlite::{named_params, params, Connection};
pub use rusqlite::Error as SQLError;


pub type Result<T> = std::result::Result<T, DBError>;
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
    pub const USERS: &'static str = "users";
    pub const USER_ID: &'static str = "userID";
    pub const USER_NAME: &'static str = "userName";
    
    //Sensor Table
    pub const SENSORS: &'static str = "sensors";
    pub const SENSOR_ID: &'static str = "sensorID";
    pub const SENSOR_TYPE: &'static str = "sensorType";
    
    //Data Packet
    pub const DATA_PACKET: &'static str = "dataPacket";
    pub const DATE_TIME: &'static str = "dateTime";
    pub const SAMPLE_FREQUENCY: &'static str = "sampleFrequency";
    pub const SAMPLE_DURATION: &'static str = "sampleDuration";
    pub const SAMPLE_AMOUNT: &'static str = "sampleAmount";
}

use TableColumnNames as Col;

pub struct Database{}



impl<'d> Database{
    const DB_NAME: &'static str = "AgriSensei Database";
    const DB_PATH: &'static str = r"data\agrisensei.db";

    pub fn connect() -> Connection{
        match Connection::open(Database::DB_NAME){
            Ok(conn) => conn,
            Err(_) => panic!("Can not connect to {0} @path=...\\{1}", Database::DB_NAME, Database::DB_PATH)
        }
    }

    pub fn new(){
        let _ = remove_file(Database::DB_PATH);

        Database::connect().execute_batch(format!(
            "CREATE TABLE IF NOT EXISTS  {users} (
                {userID}  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                {userName} TEXT DEFAULT 'John Smith'
              );
              
              CREATE TABLE IF NOT EXISTS {sensors} (
                {sensorID} INTEGER NOT NULL PRIMARY KEY,
                {sensorType} TEXT NOT NULL,
                {userID} INTEGER NOT NULL,
                FOREIGN KEY (userID) REFERENCES users (userID)
              );
              
              CREATE TABLE IF NOT EXISTS {dataPacket} (
                {dateTime} TEXT NOT NULL PRIMARY KEY,
                {sampleFrequency} INTEGER NOT NULL,
                {sampleDuration} INTEGER NOT NULL,
                {sampleAmount} INTEGER NOT NULL,
                FOREIGN KEY (sensorID) REFERENCES sensors (sensorID)
              );",
              users=Col::USERS,
              userID=Col::USER_ID,
              userName=Col::USER_NAME,
              sensors=Col::SENSORS,
              sensorID=Col::SENSOR_ID,
              sensorType=Col::SENSOR_TYPE,
              dataPacket=Col::DATA_PACKET,
              dateTime=Col::DATE_TIME,
              sampleFrequency=Col::SAMPLE_FREQUENCY,
              sampleDuration=Col::SAMPLE_DURATION,
              sampleAmount=Col::SAMPLE_AMOUNT,
            ).as_str()).unwrap();
    }

    //userId auto increments in sqlite
    pub fn new_user(name: &str) -> Option<u64>{
        let user_insert = format!(r"INSERT INTO {user}({userName}) VALUES (?1)", user=Col::USERS, userName=Col::USER_NAME);
        match Database::connect().execute(user_insert.as_str(), params![name]){
                Ok(0) => None,
                Ok(_rowsChanged) => Some(Database::connect().last_insert_rowid().try_into().unwrap()),
                Err(err) => panic!("Bad SQL Insert in Database.rs")
            }
    }

    pub fn new_sensor(&mut self, user_id: &u8){
        todo!()
    }
}