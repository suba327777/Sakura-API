use crate::domain::object::account::AccountId;
use crate::domain::object::Id;

pub type CardId = Id<crate::domain::object::card::Card>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Door {
    pub device_id: String,
    pub door_state: bool,
    pub door_switch_state: bool,
}

impl crate::domain::object::door::Door {
    pub fn new(device_id: String, door_state: bool, door_switch_state: bool) -> Self {
        Self {
            device_id,
            door_state,
            door_switch_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_card() {
        let account_id = AccountId::new(1);
        let card_name = "suica".to_string();
        let card_number = [1, 16, 3, 16, 197, 20, 106, 38].to_vec();

        let card = crate::domain::object::door::Door::new(account_id, card_name, card_number);

        assert_eq!(card.account_id.get(), 1);
        assert_eq!(card.card_name, "suica");
        assert_eq!(card.card_number, vec![1, 16, 3, 16, 197, 20, 106, 38])
    }
}
