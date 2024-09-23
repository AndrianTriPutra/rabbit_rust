use lapin::{options::*, BasicProperties, Channel};
use crate::log;

pub async fn publish(channel: &Channel, que: &str, body: &str) -> Result<(), lapin::Error> {
    let payload = body.as_bytes();

    match channel
        .basic_publish(
            "",  // exchange (empty for default exchange)
            que, // routing key (queue name)
            BasicPublishOptions {
                mandatory: false,
                immediate: false,
                ..Default::default()
            },
            payload, // message body
            BasicProperties::default()
                .with_content_type("application/json".into()), // Content type: application/json
        )
        .await
    {
        Ok(_) => {
            log::log::logger("info", "publish", &format!("Message published to queue:{} payload:{}", que, body));
            Ok(())
        }
        Err(err) => {
            log::log::logger("error", "publish", &format!("Failed to publish message: {}", err));
            Err(err)
        }
    }
}


