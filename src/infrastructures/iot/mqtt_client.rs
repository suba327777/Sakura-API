use crate::domain::repository::mqtt::client::{MessageHandler, MqttClientRepository};
use crate::server::connection::RequestContext;
use async_std::channel::Receiver;
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, AsyncClient, AsyncReceiver, Message, MQTT_VERSION_5};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

#[allow(dead_code)]
pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

const TOPICS: &[&str] = &["test", "hello"];

pub struct MqttClient {
    pub device_id: String,
    pub address: String,
    pub client: AsyncClient,
    pub handlers: HashMap<String, MessageHandler>,
    pub(crate) data: RequestContext,
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

        let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            eprintln!("Error creating the client: {:?}", e);
            std::process::exit(1);
        });

        MqttClient {
            device_id,
            address,
            client: cli,
            handlers: HashMap::new(),
            data: RequestContext::new(),
        }
    }
}

impl MqttClientRepository for MqttClient {
    async fn connect(&mut self) -> anyhow::Result<()> {
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

        // Get message stream before connecting.
        // let mut strm = self.client.get_stream(25);

        // Make the connection to the broker
        self.client.connect(conn_opts).await?;
        self.client.subscribe_many(TOPICS, &[1, 1]).await?;

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
    fn publish(&self, topic: &str, message: &str) -> anyhow::Result<()> {
        &self.client.publish(Message::new(topic, message, 0));
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

    fn start(&mut self) {
        block_on(async {
            let mut strm = self.client.get_stream(25);
            // Just loop on incoming messages.
            println!("Waiting for messages...");

            while let Some(msg_opt) = strm.next().await {
                if let Some(msg) = msg_opt {
                    if msg.retained() {
                        print!("(R) ");
                    }
                    println!("{}", msg);
                    let handlers = &self.handlers;
                    if let Some(handler) = handlers.get(msg.topic()) {
                        handler(&self.client, &msg, &self.data);
                    }
                } else {
                    println!("Lost connection. Attempting reconnect.");
                    while let Err(err) = self.client.reconnect().await {
                        println!("Error reconnecting: {}", err);
                        async_std::task::sleep(Duration::from_millis(1000)).await;
                    }
                }
            }
        })
    }
}
