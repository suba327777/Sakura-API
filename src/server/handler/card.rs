use super::super::request::{account::AccountIdRequest, card::CardRequest};
use super::super::response::card::{CardDto, CardListResponse};
use crate::domain::object::account::AccountId;
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

#[get("/cards")]
async fn get_cards(
    data: web::Data<RequestContext>,
    request: Json<AccountIdRequest>,
) -> impl Responder {
    let account_id = AccountId::new(request.account_id);
    match usecase::card::get_card_list(
        &mut data.card_repository(),
        &mut data.account_repository(),
        &account_id,
    ) {
        Ok(cards) => HttpResponse::Ok().json(CardListResponse::new(cards)),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}
