#[macro_use]
extern crate diesel;


use crate::infrastructures::config::mqtt_config::MqttConfig;
use anyhow::Error;

use std::{thread};

mod domain;
mod infrastructures;
mod server;
mod tests;
mod usecase;
mod utils;

fn main() -> std::io::Result<()> {
    let cfg = confy::load_path::<MqttConfig>("./config.yaml")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    thread::spawn(move || {
        let future = async move {
            println!("mqtt start");
            let con = infrastructures::mqtt_connection::MqttConnection::new(cfg.clone());
            if let Err(e) = usecase::mqtt::run(con.mqtt_client_repository(), cfg.clone()).await {
                eprintln!("ERROR: {}", e);
                return Err(e);
            }
            println!("mqtt end");
            Ok::<(), Error>(())
        };

        let runtime = tokio::runtime::Runtime::new().unwrap();
        if let Err(e) = runtime.block_on(future) {
            eprintln!("Thread error: {}", e);
        }
    });

    println!("server start");
    server::router::run()
}
