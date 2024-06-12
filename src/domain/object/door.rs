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
