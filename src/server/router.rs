use crate::server::connection::RequestContext;
use crate::server::handler;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RequestContext::new())) // RequestContextを使用する箇所
            .service(handler::account::post_account)
            .service(handler::account::get_accounts)
            .service(handler::account::get_account)
            .service(handler::account::delete_account)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
