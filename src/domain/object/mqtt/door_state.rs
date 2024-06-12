use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DoorState {
    pub is_open: bool,
    pub timestamp: DateTime<Local>,
    pub device_id: String,
}
