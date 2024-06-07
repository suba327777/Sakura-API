use std::sync::Arc;
use paho_mqtt::Message;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::mqtt_card::MqttCard;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::infrastructures::iot::mqtt_client::MqttClient;

pub fn mqtt_register_listener(mqtt_client: &mut MqttClient, cfg: MqttConfig){
    let cfg_clone = cfg.clone();
    let device_id = cfg.device_id.clone();  // cfg.device_id をクローンして String を作成

    mqtt_client
        .subscribe(
            "test/test_message",
            Arc::new(|msg: &Message| {
                println!("Received message on {}: {}", msg.topic(), msg);
            }),
        )
        .unwrap();

    mqtt_client
        .subscribe(
            &cfg_clone.card_receive_path,
            Arc::new(move |msg: &Message| {
                let card: MqttCard = serde_json::from_str(&msg.payload_str()).unwrap();
                // TODO: カードを受け取れるので、これを照合して開ける
                // TODO: またはカード登録処理のために別でフックする。
                let is_open = true;  // 本来はデータベース等でチェック
                let open_state = DoorState {
                    device_id: device_id.clone(),
                    open: is_open,
                    timestamp: chrono::offset::Local::now(),
                };
                let json_str = serde_json::to_string(&open_state).unwrap();
                println!("Received message on {}", msg.topic());
                println!(": id        : {:?}", card.id);
                println!(": device_id : {}", card.device_id);
                println!(": timestamp : {}", card.timestamp);
                // TODO: publish mqtt server.
                // mqtt_client.publish(&cfg.key_state_publish_path, &json_str);
            }),
        )
        .unwrap();
}