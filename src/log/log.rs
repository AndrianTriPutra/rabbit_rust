
use std::process;
use chrono::prelude::*;

static mut ACCESS: u8 = 5;

pub fn load(access: &str) {
    unsafe {
        match access {
            "fatal" => ACCESS = 0,
            "panic" => ACCESS = 1,
            "error" => ACCESS = 2,
            "warning" => ACCESS = 3,
            "info" => ACCESS = 4,
            "debug" => ACCESS = 5,
            "trace" => ACCESS = 6,
            _ => {
                ACCESS = 6; // Default to trace level
                println!("Load default trace level");
            }
        }
    }
}

pub fn logger(level: &str, location: &str, message: &str) {
    let access: u8;

    unsafe {
        access = ACCESS;
    }

    let dt: DateTime<Utc> = Utc::now();
    let ts: String = dt.to_rfc3339();
    match level {
        "fatal" => {
            if access <= 6 {
                eprintln!("{}", format!("[{}] [FATAL] [{}]: {}",ts, location, message));
                process::exit(1);
            }
        }
        "panic" => {
            if access >= 1 && access <= 6  {
                panic!("{}", format!("[{}] [PANIC] [{}]: {}",ts, location, message));
            }
        }
        "error" => {
            if access >= 2 && access <= 6 {
                eprintln!("{}", format!("[{}] [ERROR] [{}]: {}",ts, location, message));
            }
        }
        "warning" => {
            if  access >= 3 && access <= 6{
                println!("{}", format!("[{}] [WARNING] [{}]: {}",ts, location, message));
            }
        }
        "info" => {
            if   access >= 4 && access <= 6 {
                println!("{}", format!("[{}] [INFO] [{}]: {}",ts, location, message));
            }
        }
        "debug" => {
            if  access >= 5 && access <= 6{
                println!("{}", format!("[{}] [DEBUG] [{}]: {}",ts, location, message));
            }
        }
        "trace" => {
            if access == 6 {
                println!("{}", format!("[{}] [TRACE] [{}]: {}",ts, location, message));
            }
        }
        _ => println!("{}", format!("[{}] [UNKNOWN] [{}]: {}",ts, location, message)),
    }
}
