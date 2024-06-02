use crate::domain::object::{
    account::AccountId,
    card::{Card, CardId},
};
use crate::domain::repository::{account::AccountRepository, card::CardRepository};
use anyhow;

pub fn post_card(
    card_repository: &mut impl CardRepository,
    account_repository: &mut impl AccountRepository,
    card: &Card,
) -> anyhow::Result<()> {
    let account_id = &card.account_id;
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            card_repository.insert(card)?;
            Ok(())
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}

pub fn get_card_list(
    card_repository: &mut impl CardRepository,
    account_repository: &mut impl AccountRepository,
    account_id: &AccountId,
) -> anyhow::Result<Vec<Card>> {
    match account_repository.find_by_id(account_id) {
        Ok(_) => {
            let cards = card_repository.list(account_id)?;
            Ok(cards)
        }
        Err(err) => Err(anyhow::anyhow!("Failed to find account: {}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::object::{
        account::{Account, AccountId},
        card::{Card, CardId},
    };
    use crate::tests::{
        mock_account_repository::MockAccountRepository, mock_card_repository::MockCardRepository,
    };
    use crate::usecase::account::*;
    use chrono::{Duration, Local};
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn success_post_card() {
        let mut account_repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let mut card_repository = MockCardRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_card = Card {
            id: CardId::new(1),
            account_id: AccountId::new(1),
            card_name: "suica".to_string(),
            card_number: [1, 16, 3, 16, 197, 20, 106, 38].to_vec(),
            created_at: Local::now().naive_local(),
        };

        let _ = post_account(&mut account_repository, &test_account);

        let result = post_card(&mut card_repository, &mut account_repository, &test_card);

        assert!(result.is_ok());
    }
}
