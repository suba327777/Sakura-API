use crate::adapter::mqtt_listener::mqtt_register_listener;
use crate::domain::repository::mqtt::client::{MessageHandler, MqttClientRepository};
use crate::infrastructures::config::mqtt_config::MqttConfig;
use async_std::channel::Receiver;
use async_std::stream::StreamExt;
use paho_mqtt::Message;
use std::collections::HashMap;
use std::time::Duration;

pub fn run(mut client: impl MqttClientRepository, cfg: MqttConfig) -> anyhow::Result<()> {
    let result = client.connect();
    mqtt_register_listener(&mut client, cfg.clone());
    result
}
