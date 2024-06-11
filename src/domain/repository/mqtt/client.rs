use paho_mqtt::{self as mqtt, AsyncClient, AsyncReceiver, Message};
use std::collections::HashMap;
use std::sync::Arc;
use crate::server::connection::RequestContext;

pub trait MqttClientRepository {
    async fn connect(&mut self) -> anyhow::Result<()>;
    fn disconnect(&self) -> anyhow::Result<()>;
    fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error>;
    fn publish(&self, topic: &str, message: &str) -> anyhow::Result<()>;

    fn get_connection(&self) -> &paho_mqtt::AsyncClient;

    fn get_handlers(&self) -> &HashMap<String, MessageHandler>;

    fn start(&mut self);

    // fn get_stream(&mut self) -> &AsyncReceiver<Option<Message>>;
}

pub type MessageHandler = Arc<dyn Fn(&AsyncClient, &Message, &RequestContext) + Send + Sync>;

pub trait MqttListener: Fn(AsyncClient, Message) + Send + Sync + 'static {}
