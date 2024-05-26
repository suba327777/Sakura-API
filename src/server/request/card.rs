use crate::domain::object::account::AccountId;
use crate::domain::object::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CardRequest {
    account_id: AccountId,
    card_name: String,
    card_number: Vec<u8>,
}

impl CardRequest {
    pub fn of(&self) -> Card {
        Card::create(
            self.account_id.to_owned(),
            self.card_name.to_owned(),
            self.card_number.to_owned(),
        )
    }
}
