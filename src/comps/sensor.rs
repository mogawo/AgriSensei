use crate::comps::{components::*, data_packet::DataPacket};
use std::str::from_utf8;


// measurement - Acts as a heartbeat
//  Device (integer) - The device that has the measurements
//  Sensors (array of objects) - List of sensor measurements
//  Sensor (integer) - The sensor ID that has the measurement
//  Value (float) - The value measured



#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor{ 
    pub sensor_id  : u64,
    pub user_id    : u64,
    pub sensor_type: SensorType,
    pub packets    : Vec<DataPacket>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType{
    Moisture,
    Temperature,
    UnknownType,
}


impl Sensor{
    pub fn push(self){
        if let Some(_) = Database::new_sensor(self.sensor_type, self.user_id){
            let u_id = self.user_id;
            let s_id = self. sensor_id;
            println!("New User[{u_id}]-Sen[{s_id}] added to DB ")
        }
    }
    pub fn pull_sensors(user_id: u64, sensor_filter: &Query) -> Option<Vec<Sensor>>{
        Sensor::pull(user_id, sensor_filter).ok()
    }
    fn pull(user_id: u64, sensor_filter: &Query) -> Result<Vec<Sensor>, ServerError<'static>>{
        if matches!(sensor_filter, Query::None){
            return Ok(Vec::new());
        }
        
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
                        packets: DataPacket::pull_packets(row.get(0)?, &Query::All).unwrap()
                    }
                )
            })?;    
        // let result: &mut Vec<Self> = sensor_iter.filter(|sen|{
        //     sen.is_ok_and(|s| sensor_filter.contains(&s.sensor_id))}).collect();
        match sensor_filter{
            Query::All => {
                let mut ret_list: Vec<Sensor> = Vec::new();
                sensor_iter.for_each(|sen| ret_list.push(sen.unwrap()));
                Ok(ret_list)
            },
            Query::SensorFilter(filt) => {
                let mut ret_list: Vec<Sensor> = Vec::new();
                for sensor in sensor_iter{
                    let sensor = sensor.unwrap();
                    if filt.contains(&sensor.sensor_id){
                        ret_list.push(sensor)
                    }
                }
                Ok(ret_list)
            },
            _ => Err(ServerError::MessageError("Invalid Sensor Query"))
        }
        
    }
    pub fn within_range(&mut self, query: &Query) -> &mut Self{
        self.packets = DataPacket::pull_packets(self.sensor_id, query).unwrap();
        self
    }
}


//Trait Implementations
impl Display for SensorType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            SensorType::Moisture => write!{f, "Moisture"},
            SensorType::Temperature => write!{f, "Temperature"},
            SensorType::UnknownType => write!(f, "UnkownType")
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
            _ => return Err(FromSqlError::InvalidType)
        })
    }
}

impl From<&str> for SensorType{
    fn from(value: &str) -> Self {
        match value {
            "Moisture"    => SensorType::Moisture,
            "Temperature" => SensorType::Temperature,
            _             => SensorType::UnknownType
        }
    }
}