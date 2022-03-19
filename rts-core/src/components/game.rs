use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::components::building::{Bank, Barrack};
use crate::components::displayer::{ConsoleDisplayer, Displayer};
use crate::components::play_ground::{Coordinate, Identifier, PlayGround, PlayGroundObserver};
use crate::components::turn_strategy::TurnStrategy;
use crate::entity::game_actions::{Action, MoveState};
use crate::entity::player::Player;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

type InnerPlayer = Rc<RefCell<Player<TurnStrategy>>>;
type InnerMoveState = Rc<RefCell<Vec<MoveState>>>;
type InnerUnitsPlayGround = Rc<RefCell<PlayGround<Unit>>>;

const TURN_DURATION_IN_SECONDS: u64 = 10;

/// Public hooks for clients to be update on game state.
pub trait GameStateObserver {
    fn update(&self, m: &MoveState);
}

/// Our RTS game is represented by this structure.
pub struct Game<StateClient>
where
    StateClient: GameStateObserver,
{
    barrack: Barrack,
    players: Vec<InnerPlayer>,
    moves: InnerMoveState,
    map: InnerUnitsPlayGround,
    /// External clients wanting notifications on game state
    game_state_observers: Vec<StateClient>,
}

impl<StateClient> Game<StateClient>
where
    StateClient: GameStateObserver,
{
    /// Create a new game with the given players and clients wanting notifications
    pub fn new(players: Vec<Player<TurnStrategy>>, game_state_observers: Vec<StateClient>) -> Self {
        let players: Vec<InnerPlayer> = players
            .into_iter()
            .map(|player| Rc::new(RefCell::new(player)))
            .collect();
        Game {
            barrack: Barrack::default(),
            players,
            moves: Rc::new(RefCell::new(Vec::new())),
            map: Rc::new(RefCell::new(PlayGround::default())),
            game_state_observers,
        }
    }

    pub fn get_players(&self) -> &[InnerPlayer] {
        &self.players
    }
    pub fn console_display(&self) -> Result<(), RtsException> {
        let play_ground_ptr = Rc::clone(&self.map);
        let play_ground = play_ground_ptr.borrow();
        ConsoleDisplayer::display(&play_ground)
    }

    /// Events loop to handle game state
    pub fn start(&self) -> Result<(), RtsException> {
        loop {
            self.execute_recurring_actions()?;
            self.play_with_all_players()?;
            self.update_observers()?;
            if self.check_game_is_over()? {
                break;
            }

            thread::sleep(Duration::from_secs(TURN_DURATION_IN_SECONDS));
        }
        Ok(())
    }

    fn play_with_all_players(&self) -> Result<(), RtsException> {
        for (i, player) in self.players.iter().enumerate() {
            let player_ptr = Rc::clone(player);
            let action = player_ptr.borrow().request()?;
            self.play(i, action)?;
        }

        Ok(())
    }

    fn play(&self, index: usize, action: Action) -> Result<(), RtsException> {
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
            Ok(())
        } else {
            Err(RtsException::ExecuteActionException(format!(
                "Failed to execute action {} for player {}",
                action.get_name(),
                index
            )))
        }
    }

    fn execute_recurring_actions(&self) -> Result<(), RtsException> {
        for (i, _player) in self.players.iter().enumerate() {
            self.play(i, Action::GiveMoneyBatch)?;
        }

        Ok(())
    }

    fn check_game_is_over(&self) -> Result<bool, RtsException> {
        let moves_ptr = Rc::clone(&self.moves);
        let moves_mutex = moves_ptr.borrow();

        Ok(moves_mutex.contains(&MoveState::EndGame))
    }

    fn update_observers(&self) -> Result<(), RtsException> {
        let moves_ptr = Rc::clone(&self.moves);
        let moves = moves_ptr.borrow();
        for m in moves.iter() {
            if let MoveState::BuyUnit(unit) = m {
                let unit_clone = unit.clone(); // Clone here should be ok, it will be the stored item
                let play_ground_ptr = Rc::clone(&self.map);
                let mut play_ground_mutex = play_ground_ptr.borrow_mut();
                play_ground_mutex.update(unit_clone);
                self.game_state_observers
                    .iter()
                    .for_each(|client| client.update(m));
            }
        }

        Ok(())
    }

    fn update_moves_state(&self, move_state: MoveState) -> Result<(), RtsException> {
        let moves_ptr = Rc::clone(&self.moves);
        let mut moves_mutex = moves_ptr.borrow_mut();
        moves_mutex.push(move_state);
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
        let play_ground_ptr = Rc::clone(&self.map);
        let play_ground_mutex = play_ground_ptr.borrow();
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
    use crate::components::turn_strategy::TurnStrategy;
    use crate::entity::game_actions::{Action, MoveState};
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;

    use super::GameStateObserver;

    struct TestClientGameState();
    impl GameStateObserver for TestClientGameState {
        fn update(&self, _m: &MoveState) {
            println!("client has been updated !");
        }
    }

    #[test]
    pub fn should_play_with_ai() {
        let mut tigran = Player::new("Tigran".to_string(), TurnStrategy::AI);
        tigran.update_money(100);

        let emma = Player::new("Emma".to_string(), TurnStrategy::AI);

        let game = Game::new(vec![tigran, emma], vec![TestClientGameState()]);

        let m = game.play(0, Action::BuyUnit(UnitType::Classic));

        assert!(m.is_ok());
    }

    #[test]
    pub fn should_not_find_user() {
        let tigran = Player::new("Tigran".to_string(), TurnStrategy::AI);
        let game = Game::new(vec![tigran], vec![TestClientGameState()]);

        let res = game.play(1, Action::BuyUnit(UnitType::Classic));
        assert!(res.is_err());
    }
}
