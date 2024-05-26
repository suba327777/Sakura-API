use super::Id;
use crate::utils::time::create_time;
use chrono::NaiveDateTime;

pub type AccountId = Id<Account>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub username: String,
    pub grade: i32,
    pub expiration_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Account {
    pub fn create(username: String, grade: i32, expiration_date: NaiveDateTime) -> Self {
        Self {
            id: Default::default(),
            username,
            grade,
            expiration_date,
            created_at: create_time(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_create_account() {
        let username = "test_user".to_string();
        let grade = 4;
        let current_time = create_time();
        let expiration_date = current_time + Duration::hours(1);

        let account = Account::create(username.clone(), grade, expiration_date);

        assert_eq!(account.id.get(), 0);
        assert_eq!(account.username, username);
        assert_eq!(account.expiration_date, expiration_date);
    }
}
