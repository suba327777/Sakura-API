use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, Message, MQTT_VERSION_5};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;

type MessageHandler = Arc<dyn Fn(&Message) + Send + Sync>;

pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

pub struct MqttClient {
    device_id: String,
    address: String,
    client: Option<paho_mqtt::AsyncClient>,
    handlers: HashMap<String, MessageHandler>,
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

    pub async fn init_mqtt(&mut self) -> Result<(), std::io::Error> {
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

        cli.connect(conn_opts).await?;

        self.client = Some(cli);

        self.start_mqtt_check().await;
        Ok(())
    }

    pub fn subscribe(&mut self, topic: &str, handler: MessageHandler) -> Result<(), mqtt::Error> {
        block_on(async { self.client.as_mut().unwrap().subscribe(topic, 0).await }).map_err(
            |err| {
                eprintln!("Subscription error: {}", err);
                err
            },
        )?;

        self.handlers.insert(topic.to_string(), handler);
        Ok(())
    }

    async fn start_mqtt_check(&mut self) {
        match self.client {
            Some(ref mut client) => {
                let mut strm = client.get_stream(25);

                println!("Waiting for messages...");
                while let Some(msg_opt) = strm.next().await {
                    if let Some(msg) = msg_opt {
                        if msg.retained() {
                            print!("(R) ");
                        }
                        println!("{}", msg);
                        // ここで対応するハンドラーを呼び出す
                        let handlers = &self.handlers;
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
            None => {
                eprintln!("Error: MQTT client is not initialized.");
            }
        }
    }
}
