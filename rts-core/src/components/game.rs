use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio::time;

use crate::components::building::{Bank, Barrack};
use crate::components::displayer::{ConsoleDisplayer, Displayer};
use crate::components::play_ground::{Coordinate, Identifier, PlayGround, PlayGroundObserver};
use crate::entity::game_actions::{Action, MoveState};
use crate::entity::player::Player;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

type InnerPlayer = Rc<RefCell<Player>>; // May evolve to Arc<Mutex<>>
type InnerMoveState = Arc<Mutex<Vec<MoveState>>>;
type InnerUnitsPlayGround = Arc<Mutex<PlayGround<Unit>>>;

// This is an event loop
pub struct Game {
    barrack: Barrack,
    players: Vec<InnerPlayer>,
    moves: InnerMoveState,
    map: InnerUnitsPlayGround,
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
            map: Arc::new(Mutex::new(PlayGround::default())),
        }
    }

    pub fn get_players(&self) -> &[InnerPlayer] {
        &self.players
    }

    pub fn console_display(&self) -> Result<(), RtsException> {
        let play_ground_ptdr = Arc::clone(&self.map);
        let play_ground_mutex = play_ground_ptdr
            .lock()
            .map_err(|_| RtsException::GeneralException("".to_string()))?;
        ConsoleDisplayer::display(&play_ground_mutex)
    }

    /// Events loop to handle game state
    pub async fn start(&self) -> Result<(), RtsException> {
        loop {
            self.execute_recurring_actions()?;

            if self.check_game_is_over()? {
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

    // Essentialy to ease tests
    pub fn play_with(&self, index: usize, action: Action) -> Result<(), RtsException> {
        if index >= self.players.len() {
            Err(RtsException::ExecuteActionException(format!(
                "Failed to find player {} when executing action {}",
                index,
                action.get_name()
            )))
        } else if let Some(player) = self.players.get(index) {
            println!("Executing action {}", action.get_name());
            let result = self.execute_action(Rc::clone(player), action)?;
            self.update_moves_state(result)?;
            self.check_game_is_over()?;
            self.update_observer()?;
            Ok(())
        } else {
            Err(RtsException::ExecuteActionException(format!(
                "Failed to execute action {} for player {}",
                action.get_name(),
                index
            )))
        }
    }

    fn update_observer(&self) -> Result<(), RtsException> {
        let moves_ptr = Arc::clone(&self.moves);
        let moves = moves_ptr.lock().map_err(|_| {
            RtsException::UpdatePlayGroundException(
                "Failed to acquire mutex for moves when updating playground observers".to_string(),
            )
        })?;
        for m in moves.iter() {
            if let MoveState::BuyUnit(unit) = m {
                let unit_clone = unit.clone(); // Clone here should be ok, it will be the stored item
                let play_ground_ptr = Arc::clone(&self.map);
                let mut play_ground_mutex = play_ground_ptr.lock().map_err(|_| {
                    RtsException::UpdatePlayGroundException(
                        "Failed to acquire mutex for playground when updating this one"
                            .to_string(),
                    )
                })?;
                play_ground_mutex.update(unit_clone);
            }
        }
        Ok(())
    }

    fn check_game_is_over(&self) -> Result<bool, RtsException> {
        let moves_ptr = Arc::clone(&self.moves);
        let moves_mutex = moves_ptr.lock();
        if let Ok(moves_mutex) = moves_mutex {
            Ok(moves_mutex.contains(&MoveState::EndGame))
        } else {
            Err(RtsException::GeneralException(
                "Failed to aquire mutex for moves when checking game is over".to_string(),
            ))
        }
    }

    fn update_moves_state(&self, move_state: MoveState) -> Result<(), RtsException> {
        let moves_ptr = Arc::clone(&self.moves);
        let mut moves_mutex = moves_ptr.lock().map_err(|_| {
            RtsException::GeneralException(
                "Failed to aquire mutex for moves when updating playground state".to_string(),
            )
        })?;
        moves_mutex.push(move_state);
        Ok(())
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
            Action::MoveUnit(i, c) => self.move_unit(i, c),
        }
    }

    /// Available actions to be executed on the game
    fn move_unit(
        &self,
        identifier: Identifier,
        coordinate: Coordinate,
    ) -> Result<MoveState, RtsException> {
        let play_ground_ptr = Arc::clone(&self.map);
        let play_ground_mutex = play_ground_ptr.lock().map_err(|_| {
            RtsException::UpdatePlayGroundException(
                "Failed to acquire playground mutex on moving unit".to_string(),
            )
        })?;
        play_ground_mutex
            .update_cell(identifier, coordinate)
            .map(|_| MoveState::MoveUnit)
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
        println!("Player: {}, Unit: {}", player, unit);
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
