use crate::comps::{components::*, data_packet::DataPacket};
use std::str::from_utf8;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor{ 
    pub sensor_id  : u32,
    pub user_id    : u32,
    pub sensor_type: SensorType,
    pub packets    : Option<Vec<DataPacket>>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType{
    Moisture,
    Temperature,
    UnknownType 
}

impl Sensor{    
    pub fn pull_sensors(user_id: u32, sensor_filter: &[u32]) -> Option<Vec<Sensor>>{
        Sensor::pull(user_id, sensor_filter).ok()
    }
    fn pull(user_id: u32, sensor_filter: &[u32]) -> Result<Vec<Self>, rusqlite::Error>{
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
                        packets: DataPacket::pull_packets(row.get(0)?, &None)
                    }
                )
            })?;

        sensor_iter.filter(|sen|{
            sen.as_ref()
                .is_ok_and(|s| sensor_filter.contains(&s.sensor_id))})
                .collect()
        
    }
    pub fn within_range(&mut self, query: &Option<Query>) -> &mut Self{
        self.packets = DataPacket::pull_packets(self.sensor_id, query);
        self
    }
}

//Trait Implementations
impl Display for SensorType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            SensorType::Moisture => write!{f, "Moisture"},
            SensorType::Temperature => write!{f, "Temperature"},
            SensorType::UnknownType =>  write!{f, "UnkownType"}
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
            Ok("UnknownType") => SensorType::UnknownType,
            _ => return Err(FromSqlError::InvalidType)
        })
    }
}
