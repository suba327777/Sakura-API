use paho_mqtt::{self as mqtt, Message};
use std::sync::Arc;

pub trait MqttClientRepository {
    fn connect(&mut self) -> anyhow::Result<()>;
    fn disconnect(&self) -> anyhow::Result<()>;
    fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error>;
    fn publish(&mut self, topic: &str, message: &str) -> Result<(), mqtt::Error>;
    fn start_mqtt_check(&mut self);
}

pub type MessageHandler = Arc<dyn Fn(&Message) + Send + Sync>;

pub trait MqttListener: Fn(Message) + Send + Sync + 'static {}
