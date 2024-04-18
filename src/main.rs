#[macro_use]
extern crate diesel;

mod domain;
mod infrastructures;
mod server;
mod usecase;

fn main() -> std::io::Result<()> {
    infrastructures::iot::mqtt_client::run().expect("MQTT client initialization failed");
    server::router::run()
}
