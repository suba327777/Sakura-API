use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    pub device_id: String,
}

impl ::std::default::Default for MqttConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            device_id: "backend".into(),
        }
    }
}
