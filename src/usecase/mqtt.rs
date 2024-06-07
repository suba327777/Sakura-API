use std::time::Duration;
use crate::adapter::mqtt_listener::mqtt_register_listener;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;

pub async fn run(mut client: impl MqttClientRepository, cfg: MqttConfig) -> anyhow::Result<()> {
    let result = client.connect();
    mqtt_register_listener(&mut client, cfg.clone());
    client.start_mqtt_check().await;
    result
}

async fn start_mqtt_check(mut client: impl MqttClientRepository) {
    let mut strm = client.get_stream(25);

    println!("Waiting for messages...");
    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            if msg.retained() {
                print!("(R) ");
            }
            println!("{}", msg);
            // ここで対応するハンドラーを呼び出す
            let handlers = client.handlers;
            if let Some(handler) = handlers.get(msg.topic()) {
                handler(&msg);
            }
        } else {
            // A "None" means we were disconnected. Try to reconnect...
            println!("Lost connection. Attempting reconnect.");
            while let Err(err) = client.reconnect().await {
                println!("Error reconnecting: {}", err);
                // For tokio use: tokio::time::delay_for()
                async_std::task::sleep(Duration::from_millis(1000)).await;
            }
        }
    }
}
