use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyState {
    pub open: bool,
    pub timestamp: DateTime<Local>,
    pub device_id: String,
}
