use super::super::database::models::{CardEntity, NewCardEntity};
use super::super::database::schema::card::dsl;
use crate::domain::object::account::AccountId;
use crate::domain::object::card::{Card, CardId};
use crate::domain::repository::card::CardRepository;
use anyhow;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
pub struct CardRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl NewCardEntity {
    pub fn new(
        account_id: AccountId,
        card_name: String,
        card_number: Vec<u8>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            account_id: account_id.get().to_owned(),
            card_name,
            card_number,
            created_at,
        }
    }

    fn from(model: &Card) -> NewCardEntity {
        NewCardEntity {
            account_id: model.account_id.get().to_owned(),
            card_name: model.card_name.to_owned(),
            card_number: model.card_number.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
}

impl CardEntity {
    fn from(model: &Card) -> CardEntity {
        CardEntity {
            id: model.id.get(),
            account_id: model.account_id.get().to_owned(),
            card_name: model.card_name.to_owned(),
            card_number: model.card_number.to_owned(),
            created_at: model.created_at.to_owned(),
        }
    }
    fn of(&self) -> Card {
        Card {
            id: CardId::new(self.id),
            account_id: AccountId::new(self.account_id),
            card_name: self.card_name.to_owned(),
            card_number: self.card_number.to_owned(),
            created_at: self.created_at.to_owned(),
        }
    }
}

impl CardRepository for CardRepositoryImpl {
    fn insert(&self, card: &Card) -> anyhow::Result<()> {
        let entity = NewCardEntity::from(card);
        let mut conn = self.pool.get()?;
        diesel::insert_into(dsl::card)
            .values(entity)
            .execute(&mut conn)?;

        Ok(())
    }

    fn list(&self, account_id: &AccountId) -> anyhow::Result<Vec<Card>> {
        let query = dsl::card
            .filter(dsl::account_id.eq(account_id.get()))
            .into_boxed();
        let mut conn = self.pool.get()?;
        let results: Vec<CardEntity> = query.limit(20).load(&mut conn)?;

        Ok(results.into_iter().map(|e| e.of()).collect())
    }
}