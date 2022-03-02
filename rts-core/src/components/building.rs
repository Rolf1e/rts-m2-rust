use crate::components::unit_factory::UnitFactory;
use crate::entity::player::Player;
use crate::entity::player::TurnStrategyRequester;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

/// Produce units and take money from players
pub struct Barrack {
    unit_factory: UnitFactory,
}

/// Produce money and give it to players
pub struct Bank;

impl Default for Barrack {
    fn default() -> Self {
        Self::new()
    }
}

const NEW_MONEY_BATCH: i32 = 100;

impl Bank {
    pub fn give_money<T: TurnStrategyRequester>(
        player: &mut Player<T>,
    ) -> Result<(), RtsException> {
        if let Some(_money) = player.update_money(NEW_MONEY_BATCH) {
            Ok(())
        } else {
            Err(RtsException::UpdatePlayerException(
                "Give new money batch".to_string(),
            ))
        }
    }
}

impl Barrack {
    fn new() -> Self {
        Barrack {
            unit_factory: UnitFactory::default(),
        }
    }

    pub fn buy_unit<T: TurnStrategyRequester>(
        &self,
        unit_type: UnitType,
        player: &mut Player<T>,
    ) -> Result<Unit, RtsException> {
        if self.retrieve_money(&unit_type, player) {
            Ok(self.unit_factory.build_unit(unit_type))?
        } else {
            Err(RtsException::BuyUnitException(
                unit_type,
                format!("Player {} does not have enough money !", player.get_name()),
            ))
        }
    }

    fn retrieve_money<T: TurnStrategyRequester>(
        &self,
        unit_type: &UnitType,
        player: &mut Player<T>,
    ) -> bool {
        player.update_money(-unit_type.get_cost() as i32).is_some()
    }
}

#[cfg(test)]
mod test_building {
    use super::Barrack;
    use crate::components::turn_strategy::TurnStrategy;
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;

    #[test]
    pub fn should_buy_unit() {
        let mut player = Player::new(String::from("Tigran"), TurnStrategy::AI, String::new());
        player.update_money(100);
        let barrack = Barrack::default();

        if let Ok(unit) = barrack.buy_unit(UnitType::Classic, &mut player) {
            assert_eq!(&20, unit.get_health());
            assert_eq!(&80, player.get_money());
        } else {
            assert!(false);
        }
    }
}
