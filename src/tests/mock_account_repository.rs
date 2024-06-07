use crate::domain::object::account::{Account, AccountId};
use crate::domain::repository::account::AccountRepository;
use anyhow;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct MockAccountRepository {
    pub pool: RefCell<HashMap<i64, Account>>,
}

impl AccountRepository for MockAccountRepository {
    fn insert(&self, account: &Account) -> anyhow::Result<()> {
        let _ = &self
            .pool
            .borrow_mut()
            .entry(account.id.get())
            .or_insert_with(|| account.clone());

        Ok(())
    }
    fn list(&self) -> anyhow::Result<Vec<Account>> {
        let accounts: Vec<Account> = self.pool.borrow().values().cloned().collect();
        Ok(accounts)
    }

    fn find_by_id(&self, account_id: &AccountId) -> anyhow::Result<Account> {
        match self.pool.borrow().get(&account_id.get()) {
            Some(account) => Ok(account.clone()),
            None => Err(anyhow::anyhow!("Account not found")),
        }
    }

    fn update(&self, account: &Account) -> anyhow::Result<()> {
        let _ = &self
            .pool
            .borrow_mut()
            .entry(account.id.get())
            .or_insert_with(|| account.clone());

        Ok(())
    }

    fn delete(&self, account: &Account) -> anyhow::Result<()> {
        let _ = &self.pool.borrow_mut().remove(&account.id.get());
        Ok(())
    }
}
