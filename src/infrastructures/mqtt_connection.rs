use std::collections::HashMap;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::infrastructures::iot::mqtt_client::MqttClient;

pub struct MqttConnection {
    cfg: MqttConfig,
}

impl MqttConnection {
    pub fn new(cfg: MqttConfig) -> MqttConnection {
        MqttConnection { cfg }
    }

    pub fn mqtt_client_repository(&self) -> impl MqttClientRepository {
        MqttClient {
            device_id: self.cfg.device_id.clone(),
            address: self.cfg.address.clone(),
            client: None,
            handlers: HashMap::new(),
        }
    }
}
