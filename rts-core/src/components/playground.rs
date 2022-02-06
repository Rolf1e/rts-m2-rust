use std::cell::RefCell;
use std::rc::Rc;

use crate::components::building::Building;
use crate::entity::player::Player;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

pub enum PlayerIndex {
    PlayerOne,
    PlayerTwo,
}

pub struct PlayGround {
    barrack: Building,
    p1: Rc<RefCell<Player>>,
    p2: Rc<RefCell<Player>>,
}

pub enum Action {
    BuyUnit(UnitType),
}

pub enum MoveState {
    BuyUnit(Unit),
}

impl PlayGround {
    pub fn new(player_1: Player, player_2: Player) -> Self {
        PlayGround {
            barrack: Building::new(),
            p1: Rc::new(RefCell::new(player_1)),
            p2: Rc::new(RefCell::new(player_2)),
        }
    }

    pub fn play_with(
        &self,
        index: PlayerIndex,
        action: Action,
    ) -> Result<Vec<MoveState>, RtsException> {
        match index {
            PlayerIndex::PlayerOne => self.play_action(Rc::clone(&self.p1), action),
            PlayerIndex::PlayerTwo => self.play_action(Rc::clone(&self.p2), action),
        }
    }

    fn play_action(
        &self,
        player: Rc<RefCell<Player>>,
        action: Action,
    ) -> Result<Vec<MoveState>, RtsException> {
        match action {
            Action::BuyUnit(unit_type) => self.buy_unit(unit_type, player),
        }
    }

    pub fn get_players(&self) -> (Rc<RefCell<Player>>, Rc<RefCell<Player>>) {
        (Rc::clone(&self.p1), Rc::clone(&self.p2))
    }

    fn buy_unit(
        &self,
        unit_type: UnitType,
        player: Rc<RefCell<Player>>,
    ) -> Result<Vec<MoveState>, RtsException> {
        let mut player = player.borrow_mut();
        let unit = self.barrack.buy_unit(unit_type, &mut player)?;
        Ok(vec![MoveState::BuyUnit(unit)])
    }
}

#[cfg(test)]
mod tests_play_ground {

    use super::{Action, MoveState, PlayGround, PlayerIndex};
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;
    use std::cell::RefCell;

    #[test]
    pub fn should_play_with_ai() {
        let mut tigran = Player::new("Tigran".to_string());
        tigran.update_money(100);

        let emma = Player::new("Emma".to_string());

        let play_ground = PlayGround::new(tigran, emma);

        if let Ok(moves) =
            play_ground.play_with(PlayerIndex::PlayerOne, Action::BuyUnit(UnitType::Classic))
        {
            let MoveState::BuyUnit(unit) = &moves[0];
            assert_eq!(&20, unit.get_health());
            let (p1, _) = play_ground.get_players();
            let p1 = RefCell::borrow(&p1);
            assert_eq!(&80, p1.get_money());
        } else {
            assert!(false);
        }
    }
}
