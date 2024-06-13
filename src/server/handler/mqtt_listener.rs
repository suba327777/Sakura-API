use std::sync::Arc;

use paho_mqtt::{AsyncClient, Message};

use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::server::connection::RequestContext;
use crate::usecase;

pub fn mqtt_register_listener(mqtt_client: &mut impl MqttClientRepository, cfg: MqttConfig) {
    let cfg_clone = cfg.clone();
    let device_id = cfg.device_id.clone();
    let key_state_path = cfg.key_state_path.clone();
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
                    usecase::mqtt::check_card(client, msg, data, key_state_path.clone());
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
