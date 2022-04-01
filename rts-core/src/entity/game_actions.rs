use crate::components::play_ground::{Coordinate, Identifier};

use super::unit::{Unit, UnitType};

#[derive(Debug,PartialEq)]
pub enum Action {
    // Unit related
    BuyUnit(UnitType),
    MoveUnit(Identifier, Coordinate),
    // Player related
    GiveMoneyBatch,
    // Game related
    EndGame,
}

impl Action {
    pub fn get_name(&self) -> String {
        match &self {
            Action::BuyUnit(t) => format!("Buy new unit {}", t),
            Action::GiveMoneyBatch => String::from("Give new money batch to"),
            Action::EndGame => String::from("Game is over !"),
            Action::MoveUnit(i, (x, y)) => format!("Move unit {} to ({},{})", i, x, y),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum MoveState {
    BuyUnit(Unit),
    MoveUnit,
    GiveMoneyBatch,
    EndGame,
}
