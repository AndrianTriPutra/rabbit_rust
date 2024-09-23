use std::{
    time::Duration,
    process,
    env,
};
use tokio::{self, time::sleep};

mod log;
mod config;
mod rabbit;
mod gettime;
mod payload;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("argument not found");
        process::exit(1);
    }
    let mode  = args[1..2].join(" ");
    let path = args[2..3].join(" ");
    let access = args[3..4].join(" ");
    log::log::logger("info", "main", &format!("mode  : {}", mode));
    log::log::logger("info", "main", &format!("path  : {}", path));
    log::log::logger("info", "main", &format!("access: {}", access));

    log::log::load(&access);
    let conf = config::config::Config::load(&path).expect("Failed to read config file");
    
    println!();
    let ct = gettime::current_time(conf.general.tz);
    let timestamp = ct.to_rfc3339();
    println!("RUN at : [{}]", timestamp);
    log::log::logger("debug", "main", "============= CONFIG =============");
    log::log::logger("debug", "main", &format!("DEVID    : {}", conf.general.devid));
    log::log::logger("debug", "main", &format!("TZ       : {}", conf.general.tz));
    log::log::logger("debug", "main", &format!("PERIODIC : {:?}", conf.general.periodic));
    log::log::logger("debug", "main", "============= RABBIT =============");
    log::log::logger("debug", "main", &format!("HOST     : {}", conf.rabbit.host));
    log::log::logger("debug", "main", &format!("TAG      : {}", conf.rabbit.tag));
    log::log::logger("debug", "main", &format!("QUE      : {}", conf.rabbit.que));
    log::log::logger("debug", "main", &format!("RECON    : {:?}", conf.rabbit.reconnect));
    log::log::logger("debug", "main", &format!("RETRY    : {}", conf.rabbit.retries));
    log::log::logger("debug", "main", "============= RABBIT =============");
    println!();

    if mode=="publisher"{
        publisher(&conf).await;
    }else if mode=="consumer"{
        consumers(&conf).await;
    }else{
        eprintln!("mode not found");
        process::exit(1);
    }

}

async fn publisher(conf: &config::config::Config){
    let mut failed=0;
    let mut counter=0;
    let mut channel = None; 

    // Loop to handle reconnection attempts
    loop {
        counter+=1;
        log::log::logger("debug", "main", &format!("counter: {:?}", counter));

        if channel.is_none() {
            match rabbit::connect::connection(&conf).await {
                Ok(new_channel) => {
                    failed=0;
                    channel = Some(new_channel);
                    log::log::logger("info", "connection", "Successfully connected to RabbitMQ");
                }
                Err(err) => {
                    failed+=1;
                    log::log::logger("error", "connection", &format!("Failed to connect was [{}] and retry reconnect, error: {}",failed ,err));
                    if failed>conf.rabbit.retries{
                        log::log::logger("fatal", "connection", "failed rabbit more than retries");
                    }
                    sleep(Duration::from_secs(5)).await;  // Wait before retrying
                }
            }
        }

        if let Some(ref ch) = channel {
            //generate payload
            let devid = format!("{}-{}", conf.general.devid,counter);
            // let content = payload::payload(conf.general.tz, &conf.general.devid, counter);
            let content = payload::payload(conf.general.tz, &devid, counter);

            // Publish a message
            match rabbit::publish::publish(ch, &conf.rabbit.que, &content).await {
                Ok(_) => {
                    log::log::logger("info", "publish", "Message published successfully.");
                }
                Err(err) => {
                    log::log::logger("error", "publish", &format!("Failed to publish message: {}", err));
                    channel = None; 
                }
            }
                    
        }

        sleep(conf.rabbit.reconnect).await; 
    }
}

async fn consumers(conf: &config::config::Config){
    let mut failed=0;
    // Loop to handle reconnection attempts
    loop {
        match rabbit::connect::connection(&conf).await {
            Ok(channel) => {
                failed=0;
                log::log::logger("info", "connection", "Successfully connected to RabbitMQ");
                    
                // Start consuming messages
                if let Ok(consumer) = rabbit::consume::consume(&channel, &conf.rabbit.tag, &conf.rabbit.que).await {
                    // Handle messages
                    rabbit::consume::handler(consumer).await;
                }
            }
            Err(err) => {
                failed+=1;
                log::log::logger("error", "connection", &format!("Failed to connect was [{}] and retry reconnect, error: {}",failed ,err));
                if failed>conf.rabbit.retries{
                    log::log::logger("fatal", "connection", "failed rabbit more than retries");
                }
                sleep(Duration::from_secs(5)).await;  // Wait before retrying
            }
        }
    }
}

