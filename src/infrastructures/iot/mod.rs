use crate::domain::object::mqtt::Mqtt;
use crate::infrastructures::config;
use confy::load;
use mqtt::Message;
use paho_mqtt as mqtt;
use std::time::Duration;
use std::{env, process};

pub trait MessageListener: Fn(Message) + Send + Sync + 'static {}
impl<T> MessageListener for T where T: Fn(Message) + Send + Sync + 'static {}

pub fn init_mqtt() {
    let cfg = confy::load::<config::mqtt_config::MqttConfig>("Sakura-API", None)?;

    let host = env::args().nth(1).unwrap_or_else(|| cfg.host);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(cfg.device_id)
        .finalize();

    let cli = mqtt::Client::new(create_opts)
        .unwrap_or_else(|err| println!("Error creating the client: {:?}", err));

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
    }
}

pub fn subscribe_topics(
    topic: String,
    cli: &mqtt::Client,
    listener: &Box<dyn MessageListener<Output = ()>>,
) {
    if let Err(e) = cli.subscribe_many(&topic, &[0i32]) {
        println!("Error subscribes topics: {:?}", e);
        process::exit(1);
    }

    let handler = move |client: &mqtt::Client, msg: &Message| {
        listener(msg.clone());
    };

    // TODO: add request handler
    match cli.subscribe(&topic.clone(), 0) {
        Ok(mut sub_token) => {
            if sub_token.wait().is_err() {
                if let Some(err) = sub_token.error() {
                    eprintln!("Error subscribing to topic {}: {:?}", topic, err);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error subscribing to topic {}: {:?}", topic, e);
            process::exit(1);
        }
    }
}
