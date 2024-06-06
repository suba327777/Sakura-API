use crate::domain::object::account::AccountId;
use crate::domain::object::card::Card;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct CardRequest {
    account_id: AccountId,
    card_name: String,
    card_number: Vec<u8>,
}

impl CardRequest {
    pub fn of(&self) -> Card {
        Card::new(
            self.account_id.to_owned(),
            self.card_name.to_owned(),
            self.card_number.to_owned(),
        )
    }
}
