use crate::domain::repository::mqtt::client::{MessageHandler, MqttClientRepository};
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::server::handler::mqtt_listener::mqtt_register_listener;
use async_std::channel::Receiver;
use async_std::stream::StreamExt;
use paho_mqtt::Message;
use std::collections::HashMap;
use std::time::Duration;

pub async fn run(mut client: impl MqttClientRepository, cfg: MqttConfig) -> anyhow::Result<()> {
    client.connect().await?;
    println!("connected");
    mqtt_register_listener(&mut client, cfg.clone());
    println!("Register listeners");
    client.start();
    println!("owari");
    Ok(())
}
