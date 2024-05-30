use super::super::object::card::{Card, CardId};
use anyhow;
pub trait CardRepository {
    fn insert(&self, card: &Card) -> anyhow::Result<()>;
}
