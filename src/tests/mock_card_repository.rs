use crate::domain::object::card::Card;
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
            .entry(card.account_id.get())
            .or_insert_with(|| card.clone());

        Ok(())
    }
}
