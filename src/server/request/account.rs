use crate::domain::object::account::Account;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AccountRequest {
    username: String,
    grade: i32,
    expiration_date: NaiveDateTime,
}

impl AccountRequest {
    pub fn of(&self) -> Account {
        Account::create(
            self.username.to_owned(),
            self.grade.to_owned(),
            self.expiration_date.to_owned(),
        )
    }
}
