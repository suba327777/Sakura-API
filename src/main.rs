#[macro_use]
extern crate diesel;

use crate::infrastructures::config::mqtt_config::MqttConfig;
use std::thread;
use crate::adapter::mqtt_listener::mqtt_register_listener;

mod adapter;
mod domain;
mod infrastructures;
mod server;
mod tests;
mod usecase;
mod utils;

fn main() -> std::io::Result<()> {
    thread::spawn(move || {
        let cfg = confy::load_path::<MqttConfig>("./config.yaml")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let result = async move {
            println!("mqtt start");
            let con = infrastructures::mqtt_connection::MqttConnection::new(cfg);
            mqtt_register_listener(con.mqtt_client_repository(), cfg.clone());
            let result = usecase::mqtt::run(con.mqtt_client_repository(), cfg.clone());
            println!("mqtt end");
            result
        };

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(result)
    });

    println!("server start");
    server::router::run()
}
