
use std::fs::remove_file;
use std::io::ErrorKind as IOError;
use std::io::ErrorKind::NotFound;
use std::fmt::{Display, Debug};

use rusqlite::{named_params, params, Connection};
use rusqlite::Error as SQLError;

pub struct Database<'d>{
    name: &'d str,
    path: &'d str,
    connection: Connection
}
type Result<T> = std::result::Result<T, DBError>;
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


const DB_NAME: &str = "AgriSensei DB";
const DB_PATH: &str = "data\\agrisensei.db";
impl<'d> Database<'d>{
    pub fn new() -> Result<Database<'d>>{

        let db = Database{
            name: DB_NAME,
            path: DB_PATH,
            connection: Connection::open(DB_PATH)?
        };

        let _ = db.connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS  [users] (
                [userID]  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                [userName] TEXT DEFAULT 'John Smith'
              );
              
              CREATE TABLE IF NOT EXISTS [sensors] (
                [sensorID] INTEGER NOT NULL PRIMARY KEY,
                [sensorType] TEXT NOT NULL,
                [userID] INTEGER NOT NULL,
                FOREIGN KEY (userID) REFERENCES users (userID)
              );
              
              CREATE TABLE IF NOT EXISTS [dataPacket] (
                [dateTime] TEXT NOT NULL PRIMARY KEY,
                [samplePeriod] INTEGER NOT NULL,
                [sensorID] INTEGER NOT NULL,
                FOREIGN KEY (sensorID) REFERENCES sensors (sensorID)
              );
              ")?;
        Ok(db)
    }

    //userId auto increments in sqlite
    pub fn new_user(&mut self, name: &str) -> Result<()>{
        let rows_updated = self.connection.execute(
            "INSERT INTO users(userName) VALUES (?1)", 
            params![name]
        )?;

        if rows_updated == 0{
            println!("User Already Exists. No user was added")
        }
        Ok(())
    }

    pub fn new_sensor(&mut self, userID: &u8){
        
    }



    fn new_database(&mut self) -> Result<Database<'d>>{
        Database::new()
    }

    pub fn delete_database(){
        let _ = remove_file(DB_PATH);
    }

    pub fn reset_database(&mut self, del_file: bool) -> Result<()>{
        Database::delete_database();
        *self = self.new_database()?;
        Ok(())
    }
}