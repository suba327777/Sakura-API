use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::infrastructures::iot::mqtt_client::MqttClient;
use reqwest::Client;
use std::collections::HashMap;
use std::env;

pub struct MqttConnection {
    cfg: MqttConfig,
}

impl MqttConnection {
    pub fn new(cfg: MqttConfig) -> MqttConnection {
        MqttConnection { cfg }
    }

    pub fn mqtt_client_repository(&self) -> impl MqttClientRepository {
        let host = env::args()
            .nth(1)
            .unwrap_or_else(|| "mqtt://".to_string() + &self.cfg.address);

        let create_opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(&self.cfg.device_id)
            .finalize();

        let cli = paho_mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            eprintln!("Error creating the client: {:?}", e);
            std::process::exit(1);
        });
        MqttClient {
            device_id: self.cfg.device_id.clone(),
            address: self.cfg.address.clone(),
            client: cli,
            handlers: HashMap::new(),
        }
    }
}
