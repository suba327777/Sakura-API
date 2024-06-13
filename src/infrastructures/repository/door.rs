use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::domain::object::door::Door;
use crate::domain::object::mqtt::door_state::DoorState;
use crate::domain::object::mqtt::door_switch_state::DoorSwitchState;
use crate::domain::repository::door::DoorRepository;
use crate::infrastructures::database::models::{DoorEntity, NewDoorEntity};

pub struct DoorRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}

impl NewDoorEntity {
    pub fn new(device_id: String, door_state: bool, door_switch_state: bool) -> Self {
        Self {
            device_id,
            door_state,
            door_switch_state,
        }
    }
}

impl DoorEntity {
    fn of(&self) -> Door {
        Door {
            device_id: self.device_id.clone(),
            door_state: self.door_state,
            door_switch_state: self.door_switch_state,
        }
    }
}

impl DoorRepository for DoorRepositoryImpl {
    fn insert(
        &self,
        door_state: DoorState,
        door_switch_state: DoorSwitchState,
    ) -> anyhow::Result<()> {
        use super::super::database::schema::door::dsl;

        if door_state.device_id != door_switch_state.device_id {
            anyhow::bail!(
                "Device IDs do not match: {} != {}",
                door_state.device_id,
                door_switch_state.device_id
            );
        }

        let entity = NewDoorEntity::new(
            door_state.device_id,
            door_state.is_open,
            door_switch_state.is_open,
        );
        let mut conn = self.pool.get()?;
        diesel::insert_into(dsl::door)
            .values(entity)
            .execute(&mut conn)?;

        Ok(())
    }

    fn status_update(&self, door: Door) -> anyhow::Result<()> {
        use super::super::database::schema::door::dsl;

        let mut conn = self.pool.get()?;

        diesel::update(dsl::door.find(door.device_id))
            .set((
                dsl::door_state.eq(door.door_state),
                dsl::door_switch_state.eq(door.door_switch_state),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    fn find_by_device_id(&self, _device_id: String) -> anyhow::Result<Door> {
        use super::super::database::schema::door::{dsl, device_id};

        let mut conn = self.pool.get()?;
        let entity: DoorEntity = dsl::door
            .filter(dsl::device_id.eq(_device_id))
            .get_result(&mut conn)?;

        Ok(entity.of())
    }

}
