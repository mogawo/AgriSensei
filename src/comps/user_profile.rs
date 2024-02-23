use crate::comps::{components::*, sensor::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile{
    pub user_id: u64,
    pub username: String,
    pub sensors: Option<Vec<Sensor>>
}

impl UserProfile{
    pub fn pull_user(user_id: u64) ->  Self{
        match UserProfile::pull(user_id){
            Ok(mut profile) => profile,
            Err(_) => {
                println!("No User Found for ID {user_id}");
                return UserProfile{
                    user_id: 0, // <--------SQL TABLES STARTS FROM 1 THIS SHOULD ERROR DONE THE CHAIN
                    username: format!("No User#{user_id} was Pulled"),
                    sensors: None
                };
            },
        }
    }

    pub fn without(&mut self, sensors: &[u64]) -> &Self{
        self.sensors = Sensor::pull_sensors(self.user_id, sensors);
        self
    }

    pub fn within(&mut self, query: &Option<Query>) -> &Self{
        for sen in self.sensors.iter_mut().flatten(){
            sen.within_range(query);
        }
        self
    }

    fn pull(user_id: u64) -> Result<Self, rusqlite::Error>{ 
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::USERS, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(&format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        Ok::<UserProfile, rusqlite::Error>(
            statement.query_row(params![user_id], |row|{Ok(
                UserProfile{
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    sensors: None // <--array stopping filters
                }
            )})?)
        }
    }