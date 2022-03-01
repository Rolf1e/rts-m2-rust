use crate::entity::game_actions::Action;
use crate::entity::player::TurnStrategyRequester;
use crate::exceptions::RtsException;

pub enum TurnStrategy {
    AI,
}

impl TurnStrategyRequester for TurnStrategy {
    fn request(&self) -> Result<Action, RtsException> {
        match &self {
            TurnStrategy::AI => todo!(),
        }
    }
}
