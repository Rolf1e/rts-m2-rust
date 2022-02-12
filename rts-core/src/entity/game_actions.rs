use super::unit::{Unit, UnitType};

pub enum Action {
    BuyUnit(UnitType),
    GiveMoneyBatch,
    EndGame
}

impl Action {
    pub fn get_name(&self) -> String {
        match &self {
            Action::BuyUnit(t) => format!("Buy new unit {}", t),
            Action::GiveMoneyBatch => String::from("Give new money batch to"),
            Action::EndGame => String::from("Game is over !"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum MoveState {
    BuyUnit(Unit),
    GiveMoneyBatch,
    EndGame
}
