use crate::domain::repository::account::AccountRepository;
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
        MqttClient::new(self.cfg.device_id.clone(), self.cfg.address.clone());
    }
}
