use paho_mqtt::{self as mqtt, AsyncReceiver, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub trait MqttClientRepository {
    fn connect(&self) -> anyhow::Result<()>;
    fn disconnect(&self) -> anyhow::Result<()>;
    fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error>;
    fn publish(&mut self, topic: &str, message: &str) -> Result<(), mqtt::Error>;

    fn get_connection(&self) -> &paho_mqtt::AsyncClient;

    fn get_handlers(&self) -> &HashMap<String, MessageHandler>;

    // fn get_stream(&mut self) -> &AsyncReceiver<Option<Message>>;
}

pub type MessageHandler = Arc<dyn Fn(&Message) + Send + Sync>;

pub trait MqttListener: Fn(Message) + Send + Sync + 'static {}
