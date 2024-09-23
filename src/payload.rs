use serde::{Serialize, Deserialize};
use crate::gettime;

#[derive(Serialize, Deserialize, Debug)]
struct Sensor {
    sensor_id: u8,
    temp: f32,
    rh: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    dev_id: String,
    ts: String,
    data: Vec<Sensor>,
}

pub fn payload(tz:bool,devid:&str, counter:u64)->String{
    let ct = gettime::current_time(tz);
    let timestamp = ct.to_rfc3339();

    let sensors = vec![
        Sensor {
            sensor_id: 1,
            temp: 15.1 + counter as f32,
            rh: 25.5 + counter as f32,
        },
        Sensor {
            sensor_id: 2,
            temp: 25.1 + counter as f32,
            rh: 55.5 + counter as f32,
        },
    ];

    let payload = Data {
        dev_id: devid.to_string(),
        ts: timestamp,
        data: sensors,
    };

    let content = serde_json::to_string_pretty(&payload).unwrap();
    return content;
    
}