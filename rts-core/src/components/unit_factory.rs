use std::sync::{Arc, Mutex};

use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

use super::play_ground::Identifier;

pub struct UnitFactory {
    counter: Counter,
}

pub struct Counter {
    identifier: Arc<Mutex<Identifier>>,
}

impl Default for UnitFactory {
    fn default() -> Self {
        Self::new()
    }
}
impl UnitFactory {
    fn new() -> Self {
        UnitFactory {
            counter: Counter::new(),
        }
    }

    pub fn build_unit(&self, unit_type: UnitType) -> Result<Unit, RtsException> {
        let next_identifier = self.counter.get_next()?;
        match unit_type {
            UnitType::Classic => Ok(Unit::from(next_identifier, 20, 10, 5, 15, Vec::new())),
        }
    }
}

impl Counter {
    fn new() -> Self {
        Counter {
            identifier: Arc::new(Mutex::new(0)),
        }
    }

    fn get_next(&self) -> Result<Identifier, RtsException> {
        self.increment()?;
        let id = Arc::clone(&self.identifier);
        let mutex = id.lock();
        match mutex {
            Ok(mutex) => Ok(*mutex),
            Err(_) => Err(RtsException::GeneralException(
                "Failed to unlock Counter mutex".to_string(),
            )),
        }
    }

    fn increment(&self) -> Result<(), RtsException> {
        let id = Arc::clone(&self.identifier);
        let id = id.lock();
        if id.is_err() {
            return Err(RtsException::GeneralException(
                "Failed to unlock Counter mutex".to_string(),
            ));
        }
        let mut id = id.unwrap();

        if let Some(res) = id.checked_add(1) {
            *id = res;
            Ok(())
        } else {
            Err(RtsException::GeneralException(
                "Something went really bad went generating a id for unit".to_string(),
            ))
        }
    }
}
