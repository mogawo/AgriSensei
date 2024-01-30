
use rusqlite::{named_params, params, Connection, Result};
use rusqlite::Error as SQLError;

pub struct Database<'d>{
    name: &'d str,
    path: &'d str,
    connection: Connection
}
const DB_NAME: &str = "AgriSensei DB";
const DB_PATH: &str = "\\data\\agrisensei.db";
impl<'d> Database<'d>{
    pub fn new() -> Result<Database<'d>, SQLError>{

        let db = Database{
            name: DB_NAME,
            path: DB_PATH,
            connection: Connection::open(DB_PATH)?
        };

        let _ = db.connection.execute(
            "CREATE TABLE [users] (
                [userID]  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                [userName] TEXT DEFAULT 'John Smith'
            )
            
            CREATE TABLE [sensors] (
                [sensorID] INTEGER NOT NULL PRIMARY,
                [sensorType] TEXT NOT NULL,
                [userID] INTEGER NOT NULL
            )

            CREATE TABLE [sensorData] (
                [dateTime] TEXT NOT NULL PRIMARY,
                [samplePeriod] INTEGER NOT NULL,
                [sampleFrequency] INTEGER NOT NULL,
                [sensorID] INTEGER NOT NULL PRIMARY
            )
            ",
            ()
        )?;
        Ok(db)
    }

    //userId auto increments in sqlite
    pub fn newUser(&mut self, name: &str) -> Result<(), SQLError>{
        let rows_updated = self.connection.execute(
            "INSERT INTO ([usersName]) VALUES (?1, ?2)", 
            params![name]
        )?;

        if (rows_updated == 0){
            println!("User Already Exists. No user was added")
        }
        Ok(())
    }
}