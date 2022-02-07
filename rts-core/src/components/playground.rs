use std::cell::RefCell;
use std::rc::Rc;

use crate::components::building::Building;
use crate::entity::player::Player;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

type InnerPlayer = Rc<RefCell<Player>>;

pub struct PlayGround {
    barrack: Building,
    players: Vec<InnerPlayer>,
}

#[derive(Debug)]
pub enum Action {
    BuyUnit(UnitType),
}

pub enum MoveState {
    BuyUnit(Unit),
}

impl PlayGround {
    pub fn new(players: Vec<Player>) -> Self {
        let players: Vec<InnerPlayer> = players
            .into_iter()
            .map(|player| Rc::new(RefCell::new(player)))
            .collect();
        PlayGround {
            barrack: Building::default(),
            players,
        }
    }

    /// Plays a turn with player execute given action
    pub fn play_with(&self, index: usize, action: Action) -> Result<Vec<MoveState>, RtsException> {
        if index > self.players.len() {
            return Err(RtsException::ExecuteActionException(format!(
                "Failed to find player {} when executing action {:?}",
                index, action
            )));
        }

        if let Some(player) = self.players.get(index) {
            self.execute_action(Rc::clone(player), action)
        } else {
            Err(RtsException::ExecuteActionException(format!(
                "Failed to execute action {:?} for player {}",
                action, index
            )))
        }
    }

    fn execute_action(
        &self,
        player: InnerPlayer,
        action: Action,
    ) -> Result<Vec<MoveState>, RtsException> {
        match action {
            Action::BuyUnit(unit_type) => self.buy_unit(unit_type, player),
        }
    }

    pub fn get_players(&self) -> &[InnerPlayer] {
        &self.players
    }

    fn buy_unit(
        &self,
        unit_type: UnitType,
        player: InnerPlayer,
    ) -> Result<Vec<MoveState>, RtsException> {
        let mut player = player.borrow_mut();
        let unit = self.barrack.buy_unit(unit_type, &mut player)?;
        Ok(vec![MoveState::BuyUnit(unit)])
    }
}

#[cfg(test)]
mod tests_play_ground {

    use super::{Action, MoveState, PlayGround};
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;
    use std::cell::RefCell;

    #[test]
    pub fn should_play_with_ai() {
        let mut tigran = Player::new("Tigran".to_string());
        tigran.update_money(100);

        let emma = Player::new("Emma".to_string());

        let play_ground = PlayGround::new(vec![tigran, emma]);

        if let Ok(moves) = play_ground.play_with(0, Action::BuyUnit(UnitType::Classic)) {
            let MoveState::BuyUnit(unit) = &moves[0];
            assert_eq!(&20, unit.get_health());
            let players = play_ground.get_players();
            let p1 = RefCell::borrow(&players[0]);
            assert_eq!(&80, p1.get_money());
        } else {
            assert!(false);
        }
    }
}
