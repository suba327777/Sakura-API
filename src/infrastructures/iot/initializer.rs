use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::infrastructures::iot::mqtt_client::MqttClient;
use paho_mqtt::Message;
use std::sync::Arc;

pub async fn run(cfg: MqttConfig) -> std::io::Result<()> {
    println!("read cfg");
    let mut client = MqttClient::new(cfg.device_id, cfg.address);
    println!("start mqtt");
    let result = client.init_mqtt().await;
    println!("subscribe...");
    subscribe_topics(&mut client);
    client.start_mqtt_check().await;
    result
}

// TODO: configから読み取る
pub fn subscribe_topics(mqtt_client: &mut MqttClient) {
    mqtt_client
        .subscribe(
            "test/test_message",
            Arc::new(|msg: &Message| {
                println!("Received message on {}: {}", msg.topic(), msg);
            }),
        )
        .unwrap();
}
