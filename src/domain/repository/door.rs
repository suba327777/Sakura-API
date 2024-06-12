use anyhow;

use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;

pub trait CardRepository {
    fn insert(&self, door_state: DoorState, door_switch_state: DoorSwitchState) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Card>>;
    fn delete(&self, card: &Card) -> anyhow::Result<()>;
}
