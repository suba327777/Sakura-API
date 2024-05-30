use super::super::request::card::CardRequest;
use crate::server::connection::RequestContext;
use crate::usecase;
use actix_web::{delete, get, post, web, web::Json, HttpResponse, Responder};

#[post("/cards")]
async fn post_card(data: web::Data<RequestContext>, request: Json<CardRequest>) -> impl Responder {
    match usecase::card::post_card(
        &mut data.card_repository(),
        &mut data.account_repository(),
        &request.of(),
    ) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}
