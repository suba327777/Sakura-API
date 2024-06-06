use super::super::database::schema::*;
use chrono::NaiveDateTime;

#[derive(Debug, Insertable)]
#[table_name = "account"]
pub struct NewAccountEntity {
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "account"]
pub struct AccountEntity {
    pub id: i64,
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "card"]
pub struct NewCardEntity {
    pub account_id: i64,
    pub card_name: String,
    pub card_number: Vec<u8>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "card"]
pub struct CardEntity {
    pub id: i64,
    pub account_id: i64,
    pub card_name: String,
    pub card_number: Vec<u8>,
    pub created_at: NaiveDateTime,
}
