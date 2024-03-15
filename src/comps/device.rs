



use crate::{comps::components::*, Sensor};


#[derive(Serialize, Deserialize, Debug)]
pub struct Device{
    pub user_id: i64,
    pub device_id: i64,
    pub sensors: Vec<Measurements> // DIFFERENT FROM THE OTHER SENSOR STRUCT
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Measurements{
    pub sensor_id: i64,
    pub value: f64
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DevicePackage{
    pub user_id: i64,
    pub devices: Vec<Device>
}
impl DevicePackage{
    pub fn to_json(&self) -> String{
        serde_json::to_string_pretty(self).unwrap()
    }
}

pub struct DeviceRow{
    pub user_id: i64,
    pub sensor_id: i64,
    pub device_id: i64,
    pub value: f64
}

impl Device{
    pub fn push_device(&self){
        Database::add_device_measurements(&self);
    }
    pub fn pull_device(user_id: i64) -> Result<DevicePackage, rusqlite::Error>{
        let conn = Database::connect();
        let (table, user_id_col) = 
            (TableColumnNames::DEVICE_TABLE, TableColumnNames::USER_ID);        
        let mut statement = conn.prepare(&format!(
            "SELECT * 
            FROM {table} 
            WHERE {table}.{user_id_col} = (?1)
            ")).unwrap();
        let device_row_iter = statement.query_map(params![user_id], |row|{
            
            Ok(DeviceRow{
                user_id: row.get(0)?,
                device_id: row.get(1)?,
                sensor_id: row.get(2)?,
                value: row.get(3)?
            })
        })?;
        let mut dev_package = DevicePackage{user_id: user_id, devices: Vec::new()};
        let mut seen_device_id = Vec::<i64>::new();
        for row in device_row_iter{
            match row{
                Ok(dev_row) => {
                    if !seen_device_id.contains(&dev_row.device_id){
                        seen_device_id.push(dev_row.device_id);
                        dev_package.devices.push(Device { user_id: user_id, device_id: dev_row.device_id, sensors: Vec::new() });
                        dev_package.devices.last_mut().unwrap().sensors.push(Measurements{
                            sensor_id: dev_row.sensor_id,
                            value: dev_row.value
                        })
                    } else {
                        let found_dev = dev_package.devices.iter_mut().find(|dev| dev.device_id == dev_row.device_id).unwrap();
                        found_dev.sensors.push(Measurements{
                            sensor_id: dev_row.sensor_id,
                            value: dev_row.value
                        })
                    }
                }
                Err(e) => print!("\n[Pull Device Error]\n{e}\n")
            }   
        }
        Ok(dev_package)
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

