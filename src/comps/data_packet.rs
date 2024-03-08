use crate::comps::{components::*};
use crate::database::TableColumnNames as Col;
#[derive(Serialize, Deserialize, Debug)]
pub struct DataPacket{
    // #[serde(with = "ts_seconds")]
    pub date_time : DateTime<Utc>,
    pub frequency: u64,
    pub duration : u64,
    pub amount   : u64,
    pub sensor_id : u64,
    pub data: Vec<u64>,
}

impl DataPacket{

    pub fn push_packet(&self) -> Result<(), rusqlite::Error>{
        let date_time = Database::add_packet(self).unwrap();
        println!("@ {date_time} > New packet added!\n");
        Ok(())
    }
    pub fn pull_packets(sensor_id: u64, query: &Query) -> Option<Vec<Self>>{
        DataPacket::pull(sensor_id, query).ok().filter(|v| !v.is_empty())
    }
    
    fn pull(sensor_id: u64, query: &Query) -> Result<Vec<Self>, rusqlite::Error>{
        if !matches!(query, Query::None){
            return Ok(Vec::new());
        }
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::DATA_PACKET, TableColumnNames::SENSOR_ID);
        let mut statement = conn.prepare(&format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        let packet_iter = statement.query_map(params![sensor_id], 
            |row|{
                Ok(
                    DataPacket{
                        date_time: row.get(0)?,
                        frequency: row.get(1)?,
                        duration: row.get(2)?,
                        amount: row.get(3)?,
                        sensor_id: sensor_id,
                        data: DataPacket::pull_data(row.get(0)?)?,
                    }
                )
            })?;
        packet_iter.collect()
    }

    fn pull_data(date_time: DateTime<Utc>) -> Result<Vec<u64>, rusqlite::Error>{
        let conn = Database::connect();
        let (table, key) = (TableColumnNames::DATA_TABLE, TableColumnNames::DATE_TIME);
        let mut statement = conn.prepare(&format!("SELECT * FROM {table} WHERE {key} = (?1)"))?;
        let mut data_rows = statement.query(params![date_time])?;

        let mut data_points = Vec::<u64>::new();
        while let Some(row) = data_rows.next()?{
            data_points.push(row.get(1)?);
        }
        Ok(data_points)
    } 
}