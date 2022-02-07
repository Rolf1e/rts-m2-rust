use std::fmt::Display;

use crate::entity::unit::UnitType;

#[derive(Debug)]
pub enum RtsException {
    GeneralException(String),
    BuyUnitException(UnitType, String),
    ExecuteActionException(String),
}

impl Display for RtsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RtsException::GeneralException(m) | RtsException::ExecuteActionException(m) => {
                write!(f, "Rts Game: {}", m)
            }
            RtsException::BuyUnitException(u, m) => {
                write!(f, "Rts Game: Failed to buy unit {} because {}", u, m)
            }
        }
    }
}
