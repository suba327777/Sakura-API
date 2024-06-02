use super::super::object::{
    account::AccountId,
    card::{Card, CardId},
};
use anyhow;
pub trait CardRepository {
    fn insert(&self, card: &Card) -> anyhow::Result<()>;
    fn list(&self, account_id: &AccountId) -> anyhow::Result<Vec<Card>>;
}
