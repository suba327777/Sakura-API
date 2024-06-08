use crate::domain::repository::mqtt::client::{MessageHandler, MqttClientRepository};
use async_std::channel::Receiver;
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, AsyncClient, AsyncReceiver, Message, MQTT_VERSION_5};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

#[allow(dead_code)]
pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

pub struct MqttClient {
    pub device_id: String,
    pub address: String,
    pub client: paho_mqtt::AsyncClient,
    pub handlers: HashMap<String, MessageHandler>,
}

impl MqttClient {
    pub fn new(device_id: String, address: String) -> MqttClient {
        let host = env::args()
            .nth(1)
            .unwrap_or_else(|| "mqtt://".to_string() + &address);

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(&device_id)
            .finalize();

        let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            eprintln!("Error creating the client: {:?}", e);
            std::process::exit(1);
        });
        MqttClient {
            device_id,
            address,
            client: cli,
            handlers: HashMap::new(),
        }
    }
}

impl MqttClientRepository for MqttClient {
    fn connect(&self) -> anyhow::Result<()> {
        println!(
            "Connecting to the MQTT server at '{}'...",
            "mqtt://".to_string() + &self.address.to_string()
        );

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

        self.client.connect(conn_opts);

        Ok(())
    }

    fn disconnect(&self) -> anyhow::Result<()> {
        self.client.disconnect(None);
        Ok(())
    }

    fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error> {
        block_on(async { self.client.subscribe(topic, 0).await }).map_err(|err| {
            eprintln!("Subscription error: {}", err);
            err
        })?;

        self.handlers.insert(topic.to_string(), handler);
        Ok(())
    }

    fn publish(&mut self, topic: &str, message: &str) -> Result<(), mqtt::Error> {
        let mqtt_data = Message::new(topic, message, 0);
        block_on(async { self.client.publish(mqtt_data).await }).map_err(|err| {
            eprintln!("publish error: {}", err);
            err
        })?;
        Ok(())
    }

    fn get_connection(&self) -> &AsyncClient {
        &self.client
    }

    fn get_handlers(&self) -> &HashMap<String, MessageHandler> {
        &self.handlers
    }

    // fn get_stream(&mut self) -> &AsyncReceiver<Option<Message>> {
    //     &self.client.get_stream(25)
    // }
}
