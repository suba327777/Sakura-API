#[macro_use]
extern crate diesel;

use crate::domain::repository::mqtt::client::MqttClientRepository;
use crate::infrastructures::config::mqtt_config::MqttConfig;
use std::collections::HashMap;
use std::thread;

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
            let con = infrastructures::mqtt_connection::MqttConnection::new(cfg.clone());
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
