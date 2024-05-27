use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MqttConfig {
    pub address: String,
    pub device_id: String,
}

impl ::std::default::Default for MqttConfig {
    fn default() -> Self {
        Self {
            address: "mqtt:1883".into(),
            device_id: "backend".into(),
        }
    }
}