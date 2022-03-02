use std::fmt::Display;

use crate::entity::unit::UnitType;

#[derive(Debug)]
pub enum RtsException {
    GeneralException(String),
    BuyUnitException(UnitType, String),
    ExecuteActionException(String),
    StoreUnitCoordinatesException(String),
    UpdatePlayGroundException(String),
    UpdatePlayerException(String), // action
    PythonException(String),
    PythonCompileCodeException(String),
}

impl Display for RtsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RtsException::UpdatePlayerException(action) => {
                write!(
                    f,
                    "Rts Game: Failed to execute action {} with player ",
                    action
                )
            }
            RtsException::GeneralException(m)
            | RtsException::ExecuteActionException(m)
            | RtsException::UpdatePlayGroundException(m)
            | RtsException::StoreUnitCoordinatesException(m)
            | RtsException::PythonException(m) => {
                write!(f, "Rts Game: {}", m)
            }
            RtsException::BuyUnitException(u, m) => {
                write!(f, "Rts Game: Failed to buy unit {} because {}", u, m)
            }
            RtsException::PythonCompileCodeException(hash) => write!(
                f,
                "Rts Game: Failed to compile python code with hash {}",
                hash
            ),
        }
    }
}
