use std::sync::Arc;

use base64::engine::general_purpose;
use base64::Engine;
use paho_mqtt::{AsyncClient, Message};

use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
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
            &cfg.key_state_path,
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
                // curl mqtt://localhost:1883/card -d "{\"id\": [1, 2, 3, 4, 5], \"timestamp\": \"2024-06-11T10:00:00+09:00\", \"device_id\": \"device123\"}"
                move |client: &AsyncClient, msg: &Message, data: &RequestContext| {
                    let result = serde_json::from_str::<MqttCard>(&msg.payload_str());

                    if let Err(e) = result {
                        println!("Failed to parse Error: {}", e);
                        return;
                    }

                    let card = result.unwrap();
                    let decoded_id = general_purpose::STANDARD.decode(card.id).unwrap();

                    if !data
                        .card_repository()
                        .find_by_card_number(&decoded_id)
                        .unwrap()
                    {
                        println!("Card not found {:?}", decoded_id);
                        return;
                    }

                    let is_open = true; // TODO: データベースでチェック
                    let open_state = DoorState {
                        device_id: device_id.clone(),
                        is_open,
                        timestamp: chrono::offset::Local::now(),
                    };

                    let json_str = serde_json::to_string(&open_state).unwrap();

                    println!("Received message on {}", msg.topic());
                    println!(": id        : {:?}", decoded_id);
                    println!(": device_id : {}", card.device_id);
                    println!(": timestamp : {}", card.timestamp);

                    client.publish(Message::new(&cfg.key_state_path, json_str, 0));
                },
            ),
        )
        .unwrap();

    mqtt_client
        .subscribe(
            &cfg.door_state_response_path,
            Arc::new(
                |_client: &AsyncClient, msg: &Message, _data: &RequestContext| {
                    let result = serde_json::from_str::<DoorState>(&msg.payload_str());
                    if let Err(e) = result {
                        println!("Failed to parse Error: {}", e);
                    }
                    // TODO: insert or update database table
                },
            ),
        )
        .unwrap();

    mqtt_client
        .subscribe(
            &cfg.door_switch_state_response_path,
            Arc::new(
                |_client: &AsyncClient, msg: &Message, _data: &RequestContext| {
                    let result = serde_json::from_str::<DoorSwitchState>(&msg.payload_str());
                    if let Err(e) = result {
                        println!("Failed to parse Error: {}", e);
                    }
                    // TODO: insert or update database table
                },
            ),
        )
        .unwrap();
}
