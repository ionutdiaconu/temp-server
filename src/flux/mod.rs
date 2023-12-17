
use chrono::{DateTime, FixedOffset, Utc};
use influxdb2::{Client, FromDataPoint};
use influxdb2::models::Query;
use num_traits::float;

#[derive(Debug, FromDataPoint)]
pub struct SensorData {
    room: String,
    value: f64,
    time: DateTime<FixedOffset>,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            room: "".to_string(),
            value: 0_f64,
            time: chrono::MIN_DATETIME.with_timezone(&chrono::FixedOffset::east(7 * 3600)),
        }
    }
}

pub async fn read_flux() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("INFLUXDB_HOST").unwrap();
    let org = std::env::var("INFLUXDB_ORG").unwrap();
    let token = std::env::var("INFLUXDB_TOKEN").unwrap();
    let client = Client::new(host, org, token);

    let qs = format!("from(bucket: \"temps\")
        |> range(start: -1w)
        |> filter(fn: (r) => r.room == \"{}\")
        |> last()
    ", "mada");
    let query = Query::new(qs.to_string());
    let res: Vec<SensorData> = client.query::<SensorData>(Some(query))
        .await?;
    println!("{:?}", res);

    Ok(())
}

pub async fn write_flux(room: String, temp: f64) -> Result<(), Box<dyn std::error::Error>> {
    use futures::prelude::*;
    use influxdb2::models::DataPoint;
    use influxdb2::Client;

    let host = std::env::var("INFLUXDB_HOST").unwrap();
    let org = std::env::var("INFLUXDB_ORG").unwrap();
    let token = std::env::var("INFLUXDB_TOKEN").unwrap();
    let bucket = "temps";
    let client = Client::new(host, org, token);
     
    let points = vec![
        DataPoint::builder("home")
            .tag("room", room)
            .field("temp", temp)
            .field("hum", 0.0)
            .field("co", 0.0)
            .build()?
    ];
                                                             
    client.write(bucket, stream::iter(points)).await?;
     
    Ok(())
}