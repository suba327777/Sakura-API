use crate::domain::object::card::Card;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct CardListResponse {
    cards: Vec<CardDto>,
}

impl CardListResponse {
    pub fn new(cards: Vec<Card>) -> CardListResponse {
        CardListResponse {
            cards: cards.iter().map(CardDto::new).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CardDto {
    id: i64,
    card_name: String,
}

impl CardDto {
    pub fn new(model: &Card) -> CardDto {
        CardDto {
            id: model.id.get(),
            card_name: model.card_name.to_owned(),
        }
    }
}
