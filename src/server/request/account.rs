use crate::domain::object::account::Account;
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
}
