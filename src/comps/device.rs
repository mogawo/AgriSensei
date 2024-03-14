use serde::de;

use crate::{comps::components::*, UserProfile};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device{
    pub user_id: u64,
    pub device_id: u64,
    pub sensors: Vec<Measurements> // DIFFERENT FROM THE OTHER SENSOR STRUCT
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements{
    pub sensor_id: u64,
    pub value: f64
}

impl Device{
    pub fn push_device(&self, user_id: u64){
        Database::add_device_measurements(&self);
    }
    pub fn pull_device(user_id: u64, device_id: u64) -> Result<Self, rusqlite::Error>{
        let conn = Database::connect();
        let (table, userID) = 
            (TableColumnNames::DEVICE_TABLE, TableColumnNames::USER_ID);
        let mut statement = conn.prepare(&format!(
            "SELECT * 
            FROM {table} 
            WHERE {table}.{userID} = (?1)
            ")).unwrap();
        
            let mut dev = Device{user_id: user_id, device_id: device_id, sensors: Vec::new()};
        let measures_iter = statement.query_map(params![user_id], |row|{
            
            Ok(Measurements{
                sensor_id: row.get(1)?,
                value: row.get(2)?
            })
        })?;
        measures_iter.for_each(|m| dev.sensors.push(m.unwrap()));
        Ok(dev)
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(self).unwrap()
    }
}

impl Display for Device{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

