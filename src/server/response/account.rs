use crate::domain::object::account::Account;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountListResopnse {
    accounts: Vec<AccountDto>,
}

impl AccountListResopnse {
    pub fn new(accounts: Vec<Account>) -> AccountListResopnse {
        AccountListResopnse {
            accounts: accounts.iter().map(AccountDto::new).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccountDto {
    id: i64,
    username: String,
    grade: i32,
    expiration_date: NaiveDateTime,
    created_at: NaiveDateTime,
}

impl AccountDto {
    pub fn new(model: &Account) -> AccountDto {
        AccountDto {
            id: model.id.get(),
            username: model.username.to_owned(),
            grade: model.grade.to_owned(),
            expiration_date: model.expiration_date.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
}
