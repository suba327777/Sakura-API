use std::collections::HashMap;
use std::sync::Arc;
use paho_mqtt::{self as mqtt, Message};

pub trait MqttClientRepository {
    fn connect(&self) -> anyhow::Result<()>;
    fn disconnect(&self)  -> anyhow::Result<()>;
    fn subscribe(&self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error>;
    fn publish(&mut self, topic: &str, message: &str) -> Result<(), mqtt::Error>;
}

pub type MessageHandler = Arc<dyn Fn(&Message) + Send + Sync>;

pub trait MqttListener: Fn(Message) + Send + Sync + 'static {}