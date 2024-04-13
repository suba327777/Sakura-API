use crate::infrastructures;
use paho_mqtt::Client;

pub struct Mqtt {
    client: Client,
    config: infrastructures::config::mqtt_config::MqttConfig,
}
