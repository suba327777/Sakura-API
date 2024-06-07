use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::infrastructures::iot::mqtt_client::MqttClient;

pub async fn run(mut client: MqttClient, cfg: MqttConfig) -> std::io::Result<()> {
    let result = client.init_mqtt().await;
    crate::infrastructures::iot::initializer::subscribe_topics(&mut client, cfg);
    client.start_mqtt_check().await;
    result
}
