use actix_web::{delete, get, HttpResponse, post, Responder, web, web::Json};

use crate::domain::object::{account::AccountId, card::CardId};
use crate::server::connection::RequestContext;
use crate::usecase;

use super::super::request::{account::AccountIdRequest, card::CardRequest};
use super::super::response::card::{CardDto, CardListResponse};

#[post("/card")]
async fn post_card(data: web::Data<RequestContext>, request: Json<CardRequest>) -> impl Responder {
    match usecase::card::post_card(
        &data.card_repository(),
        &data.account_repository(),
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
        &data.card_repository(),
        &data.account_repository(),
        &account_id,
    ) {
        Ok(cards) => HttpResponse::Ok().json(CardListResponse::new(cards)),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}

#[get("/cards/{id}")]
async fn get_card(
    data: web::Data<RequestContext>,
    request: Json<AccountIdRequest>,
    path_params: web::Path<(i64, )>,
) -> impl Responder {
    let account_id = AccountId::new(request.account_id);
    let card_id = CardId::new(path_params.into_inner().0);
    match usecase::card::get_card(
        &data.card_repository(),
        &data.account_repository(),
        &card_id,
        &account_id,
    ) {
        Ok(card) => HttpResponse::Ok().json(CardDto::new(&card)),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}

#[delete("/cards/{id}")]
async fn delete_card(
    data: web::Data<RequestContext>,
    request: Json<AccountIdRequest>,
    path_params: web::Path<(i64, )>,
) -> impl Responder {
    let account_id = AccountId::new(request.account_id);
    let card_id = CardId::new(path_params.into_inner().0);
    match usecase::card::delete_card(
        &data.card_repository(),
        &data.account_repository(),
        &card_id,
        &account_id,
    ) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("Internal Server Error {}", err))
        }
    }
}
