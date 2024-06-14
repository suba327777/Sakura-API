use actix_web::web::Data;
use actix_web::{App, HttpServer};

use crate::server::connection::RequestContext;
use crate::server::handler;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RequestContext::new()))
            .service(handler::account::post_account)
            .service(handler::account::get_accounts)
            .service(handler::account::get_account)
            .service(handler::account::put_account)
            .service(handler::account::delete_account)
            .service(handler::card::post_card)
            .service(handler::card::get_cards)
            .service(handler::card::get_card)
            .service(handler::card::delete_card)
            .service(handler::register::register)
            .service(handler::register::get_card)
            .service(handler::register::is_register)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
