use crate::domain::repository::mqtt::client::{MessageHandler, MqttClientRepository};
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, Message, MQTT_VERSION_5};
use std::collections::HashMap;
use std::{env};
use std::time::Duration;

#[allow(dead_code)]
pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

pub struct MqttClient {
    pub device_id: String,
    pub address: String,
    pub client: Option<paho_mqtt::AsyncClient>,
    pub handlers: HashMap<String, MessageHandler>,
}

impl MqttClient {
    pub fn new(device_id: String, address: String) -> MqttClient {
        MqttClient {
            device_id,
            address,
            client: None,
            handlers: HashMap::new(),
        }
    }
}

impl MqttClientRepository for MqttClient {
    fn connect(&mut self) -> anyhow::Result<()> {
        let host = env::args()
            .nth(1)
            .unwrap_or_else(|| "mqtt://".to_string() + &self.address);

        println!("Connecting to the MQTT server at '{}'...", host);

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(&self.device_id)
            .finalize();

        let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            eprintln!("Error creating the client: {:?}", e);
            std::process::exit(1);
        });

        // Define the set of options for the connection
        let lwt = mqtt::Message::new(
            "test/lwt",
            "[LWT] Async subscriber v5 lost connection",
            mqtt::QOS_1,
        );

        let conn_opts = mqtt::ConnectOptionsBuilder::with_mqtt_version(MQTT_VERSION_5)
            .clean_start(false)
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
            .will_message(lwt)
            .finalize();

        cli.connect(conn_opts);

        self.client = Some(cli);

        Ok(())
    }

    fn disconnect(&self) -> anyhow::Result<()> {
        match self.client {
            Some(ref mut client) => {
                client.disconnect(None);
            }
            None => {
                eprintln!("Error: MQTT client is not initialized.");
            }
        }
        Ok(())
    }

    fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error> {
        block_on(async { self.client.as_mut().unwrap().subscribe(topic, 0).await }).map_err(
            |err| {
                eprintln!("Subscription error: {}", err);
                err
            },
        )?;

        self.handlers.insert(topic.to_string(), handler);
        Ok(())
    }

    fn publish(&mut self, topic: &str, message: &str) -> Result<(), mqtt::Error> {
        let mqtt_data = Message::new(topic, message, 0);
        block_on(async { self.client.as_mut().unwrap().publish(mqtt_data).await }).map_err(
            |err| {
                eprintln!("publish error: {}", err);
                err
            },
        )?;
        Ok(())
    }


}
