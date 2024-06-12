use diesel::{dsl, PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};

use crate::domain::object::door::Door;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
use crate::domain::repository::door::DoorRepository;
use crate::infrastructures::database::models::{DoorEntity, NewDoorEntity};

pub struct DoorRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl NewDoorEntity {
    pub fn new(
        device_id: String,
        door_state: bool,
        door_switch_state: bool,
    ) -> Self {
        Self {
            device_id,
            door_state,
            door_switch_state,
        }
    }
}

impl DoorEntity {}

impl DoorRepository for DoorRepositoryImpl {
    fn insert(&self, door_state: DoorState, door_switch_state: DoorSwitchState) -> anyhow::Result<()> {
        use super::super::database::schema::door::dsl;

        if (door_state.device_id != door_switch_state.device_id) {
            anyhow::bail!("Device IDs do not match: {} != {}", door_state.device_id, door_switch_state.device_id);
        }

        let entity = NewDoorEntity::new(door_state.device_id, door_state.is_open, door_switch_state.is_open);
        let mut conn = self.pool.get()?;
        diesel::insert_into(dsl::door)
            .values(entity)
            .execute(&mut conn)?;

        Ok(())
    }

    fn list(&self) -> anyhow::Result<Vec<Door>> {
        todo!()
    }

    fn delete(&self, card: &Door) -> anyhow::Result<()> {
        todo!()
    }
}