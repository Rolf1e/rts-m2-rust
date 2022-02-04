use crate::components::building::Building;
use crate::entity::player::Player;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

pub type Players = (Player, Player);

pub enum PlayerIndex {
    PlayerOne,
    PlayerTwo,
}

pub struct PlayGround {
    barrack: Building,
    players: Players,
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
            players: (player_1, player_2),
        }
    }

    pub fn play_with(
        &mut self,
        index: PlayerIndex,
        action: Action,
    ) -> Result<Vec<MoveState>, RtsException> {
        match index {
            PlayerIndex::PlayerOne => self.play_action(&mut self.players.0, action),
            PlayerIndex::PlayerTwo => self.play_action(&mut self.players.1, action),
        }
    }

    fn play_action(
        &self,
        player: &mut Player,
        action: Action,
    ) -> Result<Vec<MoveState>, RtsException> {
        match action {
            Action::BuyUnit(unit_type) => self.buy_unit(unit_type, player),
        }
    }

    pub fn get_players(&self) -> &Players {
        &self.players
    }

    fn buy_unit(
        &self,
        unit_type: UnitType,
        player: &mut Player,
    ) -> Result<Vec<MoveState>, RtsException> {
        let unit = self.barrack.buy_unit(unit_type, player)?;
        Ok(vec![MoveState::BuyUnit(unit)])
    }
}

#[cfg(test)]
mod tests_play_ground {

    use super::{Action, MoveState, PlayGround, PlayerIndex};
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;

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
            assert_eq!(&80, p1.get_money());
        } else {
            assert!(false);
        }
    }
}
