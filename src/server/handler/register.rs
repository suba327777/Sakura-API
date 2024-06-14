use actix_web::{HttpResponse, post, Responder, web};
use actix_web::web::Json;

use crate::server::connection::RequestContext;
use crate::server::request::account::AccountRequest;
use crate::server::response::mqtt_card::MqttCardIdResponse;
use crate::usecase;

#[post("/register")]
async fn register(
    data: web::Data<RequestContext>,
    request: Json<AccountRequest>,
) -> impl Responder {
    match usecase::register::start_register(&data.register_repository()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}

#[post("/register/card")]
async fn get_card(
    data: web::Data<RequestContext>,
    request: Json<AccountRequest>,
) -> impl Responder {
    match usecase::register::get_card(&data.register_repository()) {
        Ok(card_id) => HttpResponse::Ok().json(MqttCardIdResponse::new(card_id)),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}
