use paho_mqtt as mqtt;
use std::{env, process};
use std::time::Duration;

const DFLT_BROKER:&str = "tcp://broker.emqx.io:1883";
const DFLT_CLIENT:&str = "rust_publish";
const DFLT_TOPICS:&[&str] = &["rust/mqtt", "rust/test"];

pub fn initMqtt() {
    let host = env::args().nth(1).unwrap_or_else(||
        DFLT_BROKER.to_string()
    );

// Define the set of options for the create.
// Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

// Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

// Define the set of options for the connection.
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

// Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }
}
