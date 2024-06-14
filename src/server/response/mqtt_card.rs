use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct MqttCardIdResponse {
    card_id: String,
}

impl MqttCardIdResponse {
    pub fn new(card_id: String) -> MqttCardIdResponse {
        MqttCardIdResponse { card_id }
    }
}
