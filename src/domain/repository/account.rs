use super::super::object::account::{Account, AccountId};
use anyhow;

pub trait AccountRepository {
    fn insert(&self, account: &Account) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Account>>;
    fn find_by_id(&self, account_id: &AccountId) -> anyhow::Result<Account>;
    fn update(&self, account: &Account) -> anyhow::anyhow::Result<()>;
    fn delete(&self, account: &Account) -> anyhow::Result<()>;
}
