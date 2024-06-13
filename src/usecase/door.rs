use crate::domain::object::door::Door;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
use crate::domain::repository::door::DoorRepository;

pub fn insert(
    door_repository: &impl DoorRepository,
    door_state: DoorState,
    door_switch_state: DoorSwitchState,
) -> anyhow::Result<()> {
    todo!()
}

pub fn list(door_repository: &impl DoorRepository) -> anyhow::Result<Vec<Door>> {
    todo!()
}

pub fn delete(door_repository: &impl DoorRepository, card: &Door) -> anyhow::Result<()> {
    todo!()
}
