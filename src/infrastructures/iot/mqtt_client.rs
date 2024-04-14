use crate::domain;
use crate::infrastructures::config;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use crate::infrastructures::repository::account::AccountRepositoryImpl;
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, Message, Topic, MQTT_VERSION_5};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::{env, process};

type MessageHandler = Arc<dyn Fn(&Message) + Send + Sync>;

pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

pub struct MqttClient {
    cfg: MqttConfig,
    client: Option<paho_mqtt::AsyncClient>,
    handlers: Option<HashMap<String, MessageHandler>>,
}

impl MqttClient {
    pub fn new() -> Result<Self, confy::ConfyError> {
        let cfg = confy::load::<MqttConfig>("Sakura-API", None)?;
        Ok(MqttClient {
            cfg,
            client: None,
            handlers: None,
        })
    }

    pub async fn init_mqtt(&mut self) -> Result<(), mqtt::Error> {
        let host = env::args()
            .nth(1)
            .unwrap_or_else(|| "mqtt://".to_string() + &self.cfg.address);

        println!("Connecting to the MQTT server at '{}'...", host);

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(&self.cfg.device_id)
            .finalize();

        let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
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
        block_on(async {
            self.client
                .as_mut()
                .unwrap()
                .subscribe_with_options(topic, 0, &None, None)
                .await
        })
        .map_err(|err| {
            eprintln!("Subscription error: {}", err);
            err
        })?;

        self.handlers
            .get_or_insert_with(HashMap::new)
            .insert(topic.to_string(), handler);
        Ok(())
    }

    async fn start_mqtt_check(&mut self) {
        let mut strm = self.client.get_stream(25);

        println!("Waiting for messages...");
        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                if msg.retained() {
                    print!("(R) ");
                }
                println!("{}", msg);
                // ここで対応するハンドラーを呼び出す
                let handlers = self.handlers.lock().unwrap();
                if let Some(handler) = handlers.get(msg.topic()) {
                    handler(msg);
                }
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = self.client.reconnect().await {
                    println!("Error reconnecting: {}", err);
                    // For tokio use: tokio::time::delay_for()
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }
}
