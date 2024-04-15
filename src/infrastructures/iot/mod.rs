use crate::infrastructures::iot::mqtt_client::MqttClient;
use std::sync::Arc;
use paho_mqtt::Message;
use crate::infrastructures::config::mqtt_config::MqttConfig;

pub mod mqtt_client;

pub async fn run() -> std::io::Result<()> {
    let cfg = confy::load::<MqttConfig>("Sakura-API", None)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut client = MqttClient::new(cfg.device_id, cfg.address);
    client.init_mqtt().await?;
    subscribe_topics(client);
    Ok(())

}

// TODO: configから読み取る
fn subscribe_topics(mut mqtt_client: MqttClient) {
    mqtt_client
        .subscribe(
            "hoge",
            Arc::new(|msg: &Message| {
                println!("Received message on {}: {}", msg.topic(), msg);
            }),
        )
        .unwrap();
}
