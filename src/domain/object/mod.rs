use std::marker::PhantomData;

mod card;
mod door_state;
mod door_switch_state;
mod key_state;
pub mod account;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Id<T> {
    id: i64,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    pub fn get(&self) -> i64 {
        self.id
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Id::new(0)
    }
}
