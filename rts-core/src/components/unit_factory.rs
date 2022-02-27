use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

use super::play_ground::Identifier;

type InnerIdentifier = Rc<RefCell<Identifier>>;

pub struct UnitFactory {
    counter: Counter,
}

pub struct Counter {
    identifier: InnerIdentifier,
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
            identifier: Rc::new(RefCell::new(0)),
        }
    }

    fn get_next(&self) -> Result<Identifier, RtsException> {
        self.increment()?;
        let id = Rc::clone(&self.identifier);
        let mutex = id.borrow();
        Ok(*mutex)
    }

    fn increment(&self) -> Result<(), RtsException> {
        let id = Rc::clone(&self.identifier);
        let mut id = id.borrow_mut();

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
