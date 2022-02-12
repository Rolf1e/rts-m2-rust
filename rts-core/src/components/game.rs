use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio::time;

use crate::components::building::Barrack;
use crate::entity::game_actions::{Action, MoveState};
use crate::entity::player::Player;
use crate::entity::unit::UnitType;
use crate::exceptions::RtsException;

use super::building::Bank;

type InnerPlayer = Rc<RefCell<Player>>; // May evolve to Arc<Mutex<>>
type InnerMoveState = Arc<Mutex<Vec<MoveState>>>;

// This is an event loop
pub struct Game {
    barrack: Barrack,
    players: Vec<InnerPlayer>,
    moves: InnerMoveState,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let players: Vec<InnerPlayer> = players
            .into_iter()
            .map(|player| Rc::new(RefCell::new(player)))
            .collect();
        Game {
            barrack: Barrack::default(),
            players,
            moves: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_players(&self) -> &[InnerPlayer] {
        &self.players
    }

    pub async fn start(&self) -> Result<(), RtsException> {
        loop {
            self.execute_recurring_actions()?;
            if self.check_moves_state() {
                break;
            }
            time::sleep(std::time::Duration::from_secs(10)).await;
        }
        Ok(())
    }

    /// Plays a turn with player execute given action
    pub async fn play_with_async(&self, index: usize, action: Action) -> Result<(), RtsException> {
        self.play_with(index, action)
    }

    pub fn play_with(&self, index: usize, action: Action) -> Result<(), RtsException> {
        if index >= self.players.len() {
            Err(RtsException::ExecuteActionException(format!(
                "Failed to find player {} when executing action {}",
                index,
                action.get_name()
            )))
        } else if let Some(player) = self.players.get(index) {
            println!("Executing action {}", action.get_name());
            let result = self.execute_action(Rc::clone(player), action);
            self.update_moves_state(result?);
            self.check_moves_state();
            Ok(())
        } else {
            Err(RtsException::ExecuteActionException(format!(
                "Failed to execute action {} for player {}",
                action.get_name(),
                index
            )))
        }
    }

    fn check_moves_state(&self) -> bool {
        let mutex = Arc::clone(&self.moves);
        let moves = mutex.lock().unwrap();
        moves.contains(&MoveState::EndGame)
    }

    fn update_moves_state(&self, move_state: MoveState) {
        let moves = Arc::clone(&self.moves);
        let mut moves = moves.lock().unwrap();
        moves.push(move_state);
    }

    fn execute_recurring_actions(&self) -> Result<(), RtsException> {
        for (i, _player) in self.players.iter().enumerate() {
            self.play_with(i, Action::GiveMoneyBatch)?;
        }

        Ok(())
    }

    fn execute_action(
        &self,
        player: InnerPlayer,
        action: Action,
    ) -> Result<MoveState, RtsException> {
        match action {
            Action::BuyUnit(unit_type) => self.buy_unit(unit_type, player),
            Action::GiveMoneyBatch => self.give_money(player),
            Action::EndGame => Ok(MoveState::EndGame),
        }
    }

    fn give_money(&self, player: InnerPlayer) -> Result<MoveState, RtsException> {
        let mut player = player.borrow_mut();
        Bank::give_money(&mut player).map(|_| {
            println!("Successfuly give money to {}", player.get_name());
            MoveState::GiveMoneyBatch
        })
    }

    fn buy_unit(
        &self,
        unit_type: UnitType,
        player: InnerPlayer,
    ) -> Result<MoveState, RtsException> {
        let mut player = player.borrow_mut();
        let unit = self.barrack.buy_unit(unit_type, &mut player)?;
        Ok(MoveState::BuyUnit(unit))
    }
}

#[cfg(test)]
mod tests_play_ground {

    use crate::components::game::Game;
    use crate::entity::game_actions::Action;
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;

    #[test]
    pub fn should_play_with_ai() {
        let mut tigran = Player::new("Tigran".to_string());
        tigran.update_money(100);

        let emma = Player::new("Emma".to_string());

        let game = Game::new(vec![tigran, emma]);

        let m = game.play_with(0, Action::BuyUnit(UnitType::Classic));

        assert!(m.is_ok());
    }

    #[test]
    pub fn should_not_find_user() {
        let tigran = Player::new("Tigran".to_string());
        let game = Game::new(vec![tigran]);

        let res = game.play_with(1, Action::BuyUnit(UnitType::Classic));
        assert!(res.is_err());
    }
}
