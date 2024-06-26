use super::super::object::{
    account::AccountId,
    card::{Card, CardId},
};
use anyhow;
pub trait CardRepository {
    fn insert(&self, card: &Card) -> anyhow::Result<()>;
    fn list(&self, account_id: &AccountId) -> anyhow::Result<Vec<Card>>;
    fn find_by_id(&self, card_id: &CardId, account_id: &AccountId) -> anyhow::Result<Card>;
    #[allow(dead_code)]
    fn find_by_card_number(&self, card_number: &[u8]) -> anyhow::Result<bool>;
    fn delete(&self, card: &Card) -> anyhow::Result<()>;
}
