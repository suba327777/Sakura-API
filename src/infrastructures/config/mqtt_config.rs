use paho_mqtt::Message;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

type MessageHandler = Arc<dyn Fn(&Message) + Send + Sync>;

#[derive(Serialize, Deserialize)]
pub struct MqttConfig {
    pub address: String,
    pub device_id: String,
}

impl ::std::default::Default for MqttConfig {
    fn default() -> Self {
        Self {
            address: "localhost:8888".into(),
            device_id: "backend".into(),
            handlers: HashMap::new(),
        }
    }
}
