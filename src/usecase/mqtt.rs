use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
use crate::domain::object::mqtt::mqtt_card::MqttCard;
use crate::domain::repository::card::CardRepository;
use crate::domain::repository::door::DoorRepository;
use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::server::connection::RequestContext;
use crate::server::handler::mqtt_listener::mqtt_register_listener;
use base64::engine::general_purpose;
use base64::Engine;
use paho_mqtt::{AsyncClient, Message};

pub async fn run(mut client: impl MqttClientRepository, cfg: MqttConfig) -> anyhow::Result<()> {
    client.connect().await?;
    println!("connected");
    mqtt_register_listener(&mut client, cfg.clone());
    println!("Register listeners");
    client
        .publish("test/test_message", "API UP")
        .expect("failed publish message");
    client.start();
    Ok(())
}

pub fn check_card(
    _client: &AsyncClient,
    msg: &Message,
    _data: &RequestContext,
    key_state_path: String,
) {
    let result = serde_json::from_str::<MqttCard>(&msg.payload_str());

    if let Err(e) = result {
        println!("Failed to parse Error: {}", e);
        return;
    }

    let card = result.unwrap();
    let decoded_id = general_purpose::STANDARD.decode(card.id).unwrap();

    if !_data
        .card_repository()
        .find_by_card_number(&decoded_id)
        .unwrap()
    {
        println!("Card not found {:?}", decoded_id);
        return;
    }

    let result = _data
        .door_repository()
        .find_by_device_id(card.device_id.clone());

    if let Err(e) = result {
        _data
            .door_repository()
            .insert(
                DoorState {
                    is_open: false,
                    timestamp: Default::default(),
                    device_id: card.device_id.clone(),
                },
                DoorSwitchState {
                    is_open: false,
                    timestamp: Default::default(),
                    device_id: card.device_id.clone(),
                },
            )
            .expect("TODO: panic message");
    }

    let is_open = true; // TODO: データベースでチェック
    let open_state = DoorState {
        device_id: card.device_id.clone(),
        is_open,
        timestamp: chrono::offset::Local::now(),
    };

    let json_str = serde_json::to_string(&open_state).unwrap();

    println!("Received message on {}", msg.topic());
    println!(": id        : {:?}", decoded_id);
    println!(": device_id : {}", card.device_id);
    println!(": timestamp : {}", card.timestamp);

    _client.publish(Message::new(key_state_path, json_str, 0));
}
