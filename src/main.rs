#[macro_use]
extern crate diesel;

mod domain;
mod infrastructures;
mod server;
mod usecase;
mod utils;

fn main() -> std::io::Result<()> {
    server::router::run()
}
