use std::sync::Arc;

use paho_mqtt::{self as mqtt, AsyncClient, Message};

use crate::server::connection::RequestContext;

pub trait MqttClientRepository {
    async fn connect(&mut self) -> anyhow::Result<()>;
    fn disconnect(&self) -> anyhow::Result<()>;
    fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error>;
    fn publish(&self, topic: &str, message: &str) -> anyhow::Result<()>;
    fn start(&mut self);
}

pub type MessageHandler = Arc<dyn Fn(&AsyncClient, &Message, &RequestContext) + Send + Sync>;
