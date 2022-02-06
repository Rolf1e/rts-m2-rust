use crate::components::unit_factory::UnitFactory;
use crate::entity::player::Player;
use crate::entity::unit::{Unit, UnitType};
use crate::exceptions::RtsException;

pub struct Building {
    unit_factory: UnitFactory,
}

impl Building {
    pub fn new() -> Self {
        Building {
            unit_factory: UnitFactory::new(),
        }
    }

    pub fn buy_unit(&self, unit_type: UnitType, player: &mut Player) -> Result<Unit, RtsException> {
        if self.retrieve_money(&unit_type, player) {
            Ok(self.unit_factory.build_unit(unit_type))
        } else {
            Err(RtsException::BuyUnitException(
                unit_type,
                format!("Player {} does not have enough money !", player.get_name()),
            ))
        }
    }

    fn retrieve_money(&self, unit_type: &UnitType, player: &mut Player) -> bool {
        player.update_money(-unit_type.get_cost() as i32).is_some()
    }
}

#[cfg(test)]
mod test_building {
    use super::Building;
    use crate::entity::player::Player;
    use crate::entity::unit::UnitType;

    #[test]
    pub fn should_buy_unit() {
        let mut player = Player::new(String::from("Tigran"));
        player.update_money(100);
        let barrack = Building::new();

        if let Ok(unit) = barrack.buy_unit(UnitType::Classic, &mut player) {
            assert_eq!(&20, unit.get_health());
            assert_eq!(&80, player.get_money());
        } else {
            assert!(false);
        }
    }
}
