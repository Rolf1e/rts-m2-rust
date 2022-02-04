use crate::entity::unit::{Unit, UnitType};

pub struct UnitFactory {}

impl UnitFactory {
    pub fn new() -> Self {
        UnitFactory {}
    }

    pub fn build_unit(&self, unit_type: UnitType) -> Unit {
        match unit_type {
            UnitType::Classic => Unit::from(20, 10, 5, 15, Vec::new()),
        }
    }
}
