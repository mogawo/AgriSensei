use std::borrow::BorrowMut;
use crate::comps::{components::*, sensor::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile{
    pub user_id: u64,
    pub username: String,
    pub sensors: Vec<Sensor>
}

impl UserProfile{
    pub fn pull_user(user_id: u64) -> Option<Self>{
        match UserProfile::pull(user_id){
            Ok(mut profile) => Some(profile),
            Err(e) => {
                println!("[UserProfile::pull_user] No User Found for ID {user_id}\n{e}");
                None
            },
        }
    }

    pub fn include(mut self, sensors: &Query) -> Self{
        self.sensors = Sensor::pull_sensors(self.user_id, sensors).unwrap();
        self
    }

    pub fn within(mut self, query: &Query) -> Self{
        for sen in self.sensors.iter_mut(){
            sen.within_range(query);
        }
        self
    }

    fn pull(user_id: u64) -> Result<UserProfile, rusqlite::Error>{ 
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::USERS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(&format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;

        statement.query_row(params![user_id], |row|{
            Ok(UserProfile{
                user_id: row.get(0)?,
                username: row.get(1)?,
                sensors: Vec::new() 
            })
        })
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(self).unwrap()
    }
}