use std::{
    fs::File, 
    io::Read, 
    time::Duration,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: General,  // Field ini harus publik
    pub rabbit: Rabbit,    // Field ini harus publik
}

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub devid: String,     // Field ini harus publik
    pub tz: bool,
    #[serde(with = "humantime_serde")] 
    pub periodic: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rabbit {
    pub host: String,      // Field ini harus publik
    pub tag : String,
    pub que : String,
    #[serde(with = "humantime_serde")] 
    pub reconnect: Duration,
    pub retries: i8,
}


impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }
}