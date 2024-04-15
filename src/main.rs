#[macro_use]
extern crate diesel;

mod domain;
mod infrastructures;
mod server;
mod usecase;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = server::run();
    infrastructures::iot::run().await
}
