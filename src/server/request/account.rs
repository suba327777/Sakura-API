use crate::domain::object::account::{Account, AccountId};
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AccountRequest {
    username: String,
    grade: i32,
    expiration_date: NaiveDateTime,
}

#[derive(Debug, Default, Deserialize)]
pub struct AccountIdRequest {
    pub account_id: i64,
}

impl AccountRequest {
    pub fn of(&self) -> Account {
        Account::new(
            self.username.to_owned(),
            self.grade.to_owned(),
            self.expiration_date.to_owned(),
        )
    }
    pub fn model(&self, account_id: AccountId, created_at: NaiveDateTime) -> Account {
        Account {
            id: account_id,
            username: self.username.to_owned(),
            grade: self.grade.to_owned(),
            expiration_date: self.expiration_date.to_owned(),
            created_at,
        }
    }
}
