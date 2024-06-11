use crate::domain::repository::mqtt::client::{MqttClientRepository};
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::server::handler::mqtt_listener::mqtt_register_listener;






pub async fn run(mut client: impl MqttClientRepository, cfg: MqttConfig) -> anyhow::Result<()> {
    client.connect().await?;
    println!("connected");
    mqtt_register_listener(&mut client, cfg.clone());
    println!("Register listeners");
    client.start();
    println!("owari");
    Ok(())
}
