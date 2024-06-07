use crate::domain::object::account::{Account, AccountId};
use crate::domain::repository::account::AccountRepository;
use anyhow::Result;

pub fn post_account(repository: &mut impl AccountRepository, account: &Account) -> Result<()> {
    repository.insert(account)
}

pub fn get_account_list(repository: &mut impl AccountRepository) -> Result<Vec<Account>> {
    repository.list()
}

pub fn get_account(
    repository: &mut impl AccountRepository,
    account_id: &AccountId,
) -> Result<Account> {
    repository.find_by_id(account_id)
}

pub fn delete_account(
    repository: &mut impl AccountRepository,
    account_id: &AccountId,
) -> Result<()> {
    let account = repository.find_by_id(account_id)?;
    repository.delete(&account)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::object::account::AccountId;
    use crate::tests::mock_account_repository::MockAccountRepository;
    use chrono::{Duration, Local};
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn success_post_account() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let result = post_account(&mut repository, &test_account);
        assert!(result.is_ok());
    }

    #[test]
    fn success_get_accounts() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let test_account2 = Account {
            id: AccountId::new(2),
            username: "test_user2".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);
        let _ = repository.insert(&test_account2);

        let result = get_account_list(&mut repository);

        let accounts = result.unwrap();
        assert_eq!(accounts.len(), 2);
    }

    #[test]
    fn success_find_account_by_id() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);

        let result = get_account(&mut repository, &test_account.id);

        assert!(result.is_ok());

        let retrieved_account = result.unwrap();

        assert_eq!(retrieved_account.id.get(), test_account.id.get());
        assert_eq!(retrieved_account.username, test_account.username);
        assert_eq!(retrieved_account.grade, test_account.grade);
        assert_eq!(
            retrieved_account.expiration_date,
            test_account.expiration_date
        );
        assert_eq!(retrieved_account.created_at, test_account.created_at);
    }

    #[test]
    fn failed_find_account_by_id() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let _ = repository.insert(&test_account);

        let result = get_account(&mut repository, &AccountId::new(2));

        assert!(result.is_err());
    }

    #[test]
    fn success_delete_account() {
        let mut repository = MockAccountRepository {
            pool: RefCell::new(HashMap::new()),
        };

        let test_account = Account {
            id: AccountId::new(1),
            username: "test_user".to_string(),
            grade: 4,
            expiration_date: Local::now().naive_local() + Duration::hours(1),
            created_at: Local::now().naive_local(),
        };

        let _ = post_account(&mut repository, &test_account);

        let _ = delete_account(&mut repository, &test_account.id);

        assert!(get_account(&mut repository, &test_account.id).is_err())
    }
}
