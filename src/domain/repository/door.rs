use anyhow;

use crate::domain::object::door::Door;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;

pub trait DoorRepository {
    fn insert(&self, door_state: DoorState, door_switch_state: DoorSwitchState) -> anyhow::Result<()>;
    fn update(&self, door: Door);
    fn list(&self) -> anyhow::Result<Vec<Door>>;
    fn delete(&self, card: &Door) -> anyhow::Result<()>;
}
