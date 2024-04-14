use crate::domain::object::mqtt::Mqtt;
use crate::infrastructures::config;
use confy::load;
use mqtt::Message;
use std::time::Duration;
use std::{env, process};

use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, MQTT_VERSION_5, Topic};
use crate::domain;

pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

pub fn init_mqtt() -> domain::object::mqtt {
    let cfg = confy::load::<config::mqtt_config::MqttConfig>("Sakura-API", None)?;

    // Initialize the logger from the environment
    env_logger::init();

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://" + cfg.address.to_string());

    println!("Connecting to the MQTT server at '{}'...", host);

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(cfg.device_id)
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        // Define the set of options for the connection
        let lwt = mqtt::Message::new(
            "test/lwt",
            "[LWT] Async subscriber v5 lost connection",
            mqtt::QOS_1,
        );

        // Connect with MQTT v5 and a persistent server session (no clean start).
        // For a persistent v5 session, we must set the Session Expiry Interval
        // on the server. Here we set that requests will persist for an hour
        // (3600sec) if the service disconnects or restarts.
        let conn_opts = mqtt::ConnectOptionsBuilder::with_mqtt_version(MQTT_VERSION_5)
            .clean_start(false)
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
            .will_message(lwt)
            .finalize();

        cli.connect(conn_opts).await?;

        let mqtt = domain::object::mqtt {
            client: &cli,
            config: &cfg
        };

        let _ = start_mqtt_check(mqtt);

        return mqtt;

    }) {
        eprintln!("{}", err);
    }
}

pub fn subscribe(topic: String, mqtt_client: &domain::object::mqtt){
    let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); TOPICS.len()];
    if let Err(err) = block_on(async {
        mqtt_client.client.subscribe_with_options(topic, 0, &sub_opts, None)
            .await?;
    }) {
        eprintln!("{}", err);
    }

}

async fn start_mqtt_check(mqtt_client: &domain::object::mqtt){
    let mut strm = mqtt_client.client.get_stream(25);

    println!("Waiting for messages...");
    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            if msg.retained() {
                print!("(R) ");
            }
            println!("{}", msg);
        }
        else {
            // A "None" means we were disconnected. Try to reconnect...
            println!("Lost connection. Attempting reconnect.");
            while let Err(err) = mqtt_client.client.reconnect().await {
                println!("Error reconnecting: {}", err);
                // For tokio use: tokio::time::delay_for()
                async_std::task::sleep(Duration::from_millis(1000)).await;
            }
        }
    }
}