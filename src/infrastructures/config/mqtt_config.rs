use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MqttConfig {
    pub address: String,
    pub device_id: String,
    pub card_receive_path: String,
    pub key_state_path: String,
    pub door_state_request_path: String,
    pub door_state_response_path: String,
    pub door_switch_state_request_path: String,
    pub door_switch_state_response_path: String,
}

impl ::std::default::Default for MqttConfig {
    fn default() -> Self {
        Self {
            address: "mqtt:1883".into(),
            device_id: "backend".into(),
            card_receive_path: "card".into(),
            key_state_path: "key".into(),
            door_state_request_path: "door/state-request".into(),
            door_state_response_path: "door/state".into(),
            door_switch_state_request_path: "door/switch-state".into(),
            door_switch_state_response_path: "door/switch-state".into(),
        }
    }
}
