use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MqttCard {
    pub id: Vec<u8>,
    pub timestamp: DateTime<Local>,
    pub device_id: String,
}
