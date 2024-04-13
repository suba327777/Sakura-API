use paho_mqtt::Client;
use crate::infrastructures;

pub struct Mqtt {
    client: Client,
    config: infrastructures::config::mqtt_config::MqttConfig
}