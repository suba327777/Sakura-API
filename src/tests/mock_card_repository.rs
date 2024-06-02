use crate::domain::object::{
    account::AccountId,
    card::{Card, CardId},
};
use crate::domain::repository::card::CardRepository;
use anyhow;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct MockCardRepository {
    pub pool: RefCell<HashMap<i64, Card>>,
}

impl CardRepository for MockCardRepository {
    fn insert(&self, card: &Card) -> anyhow::Result<()> {
        let _ = &self
            .pool
            .borrow_mut()
            .entry(card.id.get())
            .or_insert_with(|| card.clone());

        Ok(())
    }

    fn list(&self, account_id: &AccountId) -> anyhow::Result<Vec<Card>> {
        let cards: Vec<Card> = self
            .pool
            .borrow()
            .values()
            .filter(|card| card.account_id == *account_id)
            .cloned()
            .collect();

        Ok(cards)
    }

    fn find_by_id(&self, card_id: &CardId, account_id: &AccountId) -> anyhow::Result<Card> {
        match self
            .pool
            .borrow()
            .get(&card_id.get())
            .filter(|card| card.account_id == *account_id)
        {
            Some(card) => Ok(card.clone()),
            None => Err(anyhow::anyhow!("Card not found")),
        }
    }
}
