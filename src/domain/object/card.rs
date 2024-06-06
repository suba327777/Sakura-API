use super::{account::AccountId, Id};
use crate::utils::time::create_time;
use chrono::NaiveDateTime;

pub type CardId = Id<Card>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Card {
    pub id: CardId,
    pub account_id: AccountId,
    pub card_name: String,
    pub card_number: Vec<u8>,
    pub created_at: NaiveDateTime,
}

impl Card {
    pub fn new(account_id: AccountId, card_name: String, card_number: Vec<u8>) -> Self {
        Self {
            id: Default::default(),
            account_id,
            card_name,
            card_number,
            created_at: create_time(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_card() {
        let account_id = AccountId::new(1);
        let card_name = "suica".to_string();
        let card_number = [1, 16, 3, 16, 197, 20, 106, 38].to_vec();

        let card = Card::new(account_id, card_name, card_number);

        assert_eq!(card.account_id.get(), 1);
        assert_eq!(card.card_name, "suica");
        assert_eq!(card.card_number, vec![1, 16, 3, 16, 197, 20, 106, 38])
    }
}
