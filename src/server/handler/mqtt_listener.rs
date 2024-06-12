use std::sync::Arc;

use paho_mqtt::{AsyncClient, Message};

use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::mqtt_card::MqttCard;
use crate::domain::repository::card::CardRepository;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::server::connection::RequestContext;

pub fn mqtt_register_listener(mqtt_client: &mut impl MqttClientRepository, cfg: MqttConfig) {
    let cfg_clone = cfg.clone();
    let device_id = cfg.device_id.clone();

    mqtt_client
        .subscribe(
            "test/test_message",
            Arc::new(
                |_client: &AsyncClient, msg: &Message, _data: &RequestContext| {
                    println!("Received message on {}: {}", msg.topic(), msg);
                },
            ),
        )
        .unwrap();

    mqtt_client
        .subscribe(
            &cfg.key_state_publish_path,
            Arc::new(
                |_client: &AsyncClient, msg: &Message, _data: &RequestContext| {
                    println!("Received message on {}: {}", msg.topic(), msg);
                },
            ),
        )
        .unwrap();

    mqtt_client
        .subscribe(
            &cfg_clone.card_receive_path,
            Arc::new(
                move |client: &AsyncClient, msg: &Message, data: &RequestContext| {
                    let card: MqttCard = serde_json::from_str(&msg.payload_str()).unwrap();
                    // curl mqtt://localhost:1883/card -d "{\"id\": [1, 2, 3, 4, 5], \"timestamp\": \"2024-06-11T10:00:00+09:00\", \"device_id\": \"device123\"}"

                    if !data
                        .card_repository()
                        .find_by_card_number(&card.id)
                        .unwrap()
                    {
                        println!("Card not found {:?}", card.id);
                        return;
                    }

                    let is_open = true; // 本来はデータベース等でチェック
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
                    client.publish(Message::new(&cfg.key_state_publish_path, json_str, 0));
                },
            ),
        )
        .unwrap();
}
