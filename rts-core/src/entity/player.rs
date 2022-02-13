use std::fmt::Display;

pub struct Player {
    name: String,
    wallet: Wallet,
}

struct Wallet {
    money: i32,
}

impl Wallet {
    fn new() -> Self {
        Wallet { money: 0 }
    }
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            wallet: Wallet::new(),
        }
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

impl Display for Player {
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

    use super::Player;

    #[test]
    pub fn should_earn_money() {
        let mut player = Player::new("Tigran".to_string());
        player.update_money(10);
        assert_eq!(&10, player.get_money())
    }

    #[test]
    pub fn should_loose_money() {
        let mut player = Player::new("Tigran".to_string());
        player.update_money(10);
        player.update_money(-8);
        assert_eq!(&2, player.get_money())
    }
}
