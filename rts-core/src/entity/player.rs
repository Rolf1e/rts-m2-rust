use std::fmt::Display;

use crate::exceptions::RtsException;
use crate::entity::game_actions::Action;

pub trait TurnStrategyRequester {
    fn request(&self) -> Result<Action, RtsException>;
}

pub struct Player<TurnStrategy>
where
    TurnStrategy: TurnStrategyRequester,
{
    name: String,
    wallet: Wallet,
    turn_strategy_requester: TurnStrategy,
}

impl<TurnStrategy> Player<TurnStrategy>
where
    TurnStrategy: TurnStrategyRequester,
{
    pub fn new(name: String, turn_strategy_requester: TurnStrategy) -> Self {
        Player {
            name,
            wallet: Wallet::new(),
            turn_strategy_requester,
        }
    }

    pub fn request(&self) -> Result<Action, RtsException> {
        self.turn_strategy_requester.request()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_money(&self) -> &i32 {
        &self.wallet.money
    }

    pub fn update_money(&mut self, amount: i32) -> Option<i32> {
        if let Some(res) = self.wallet.money.checked_add(amount) {
            self.wallet.money = res;
            Some(self.wallet.money)
        } else {
            None
        }
    }
}

struct Wallet {
    money: i32,
}

impl Wallet {
    fn new() -> Self {
        Wallet { money: 0 }
    }
}

impl<TurnStrategy> Display for Player<TurnStrategy>
where
    TurnStrategy: TurnStrategyRequester,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} wallet: {}", self.name, self.wallet)
    }
}

impl Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.money)
    }
}

#[cfg(test)]
mod test_wallet {

    use crate::entity::game_actions::Action;
    use crate::exceptions::RtsException;

    use super::Player;
    use super::TurnStrategyRequester;

    pub struct TestTurnStrategyRequester;
    impl TurnStrategyRequester for TestTurnStrategyRequester {
        fn request(&self) -> Result<Action, RtsException> {
            Ok(Action::GiveMoneyBatch)
        }
    }

    #[test]
    pub fn should_earn_money() {
        let mut player = Player::new("Tigran".to_string(), TestTurnStrategyRequester);
        player.update_money(10);
        assert_eq!(&10, player.get_money())
    }

    #[test]
    pub fn should_loose_money() {
        let mut player = Player::new("Tigran".to_string(), TestTurnStrategyRequester);
        player.update_money(10);
        player.update_money(-8);
        assert_eq!(&2, player.get_money())
    }
}
