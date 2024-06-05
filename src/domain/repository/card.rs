use super::super::object::{
    account::AccountId,
    card::{Card, CardId},
};
use anyhow;
pub trait CardRepository {
    fn insert(&self, card: &Card) -> anyhow::Result<()>;
    fn list(&self, account_id: &AccountId) -> anyhow::Result<Vec<Card>>;
    fn find_by_id(&self, card_id: &CardId, account_id: &AccountId) -> anyhow::Result<Card>;
    fn find_by_card_number(&self, card_number: &Vec<u8>) -> anyhow::Result<bool>;
}
