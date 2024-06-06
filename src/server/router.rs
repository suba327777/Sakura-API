use crate::server::connection::RequestContext;
use crate::server::handler;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RequestContext::new()))
            .service(handler::account::post_account)
            .service(handler::account::get_accounts)
            .service(handler::account::get_account)
            .service(handler::account::delete_account)
            .service(handler::card::post_card)
            .service(handler::card::get_cards)
            .service(handler::card::get_card)
            .service(handler::card::delete_card)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
