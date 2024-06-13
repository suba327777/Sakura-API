use anyhow;

use crate::domain::object::door::Door;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;

pub trait DoorRepository {
    fn insert(
        &self,
        door_state: DoorState,
        door_switch_state: DoorSwitchState,
    ) -> anyhow::Result<()>;
    #[allow(dead_code)]
    fn status_update(&self, door: Door) -> anyhow::Result<()>;
    fn find_by_device_id(&self, device_id: String) -> anyhow::Result<Door>;
}
