use crate::infrastructures::iot::mqtt_client::MqttClient;
use paho_mqtt::Message;
use std::sync::Arc;

pub mod mqtt_client;

use futures::{executor::block_on, stream::StreamExt, TryStreamExt};

pub fn run() {
    let mut client = MqttClient::new().unwrap();
    block_on(async {
        let result = client.init_mqtt().await;
        if !result.is_err() {
            subscribe_topics(client);
        }
    })
    .map_err(|err| {
        eprintln!("initialized error: {}", err);
        err
    })?;
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
