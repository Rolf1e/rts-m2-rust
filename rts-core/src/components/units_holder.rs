use crate::entity::unit::Unit;
use crate::exceptions::RtsException;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Coordinate = (usize, usize);
type UnitsStore = HashMap<String, Vec<InnerUnits>>;

type InnerUnits = (Coordinate, Unit);

/// Holds units positions by players
pub struct UnitsHolder {
    units_coordinate_by_players: UnitsStore,
}

impl Default for UnitsHolder {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitsHolder {
    pub fn new() -> Self {
        UnitsHolder {
            units_coordinate_by_players: HashMap::new(),
        }
    }

    pub fn store(
        &mut self,
        player_name: String,
        coordinate: Coordinate,
        unit: Unit,
    ) -> Result<(), RtsException> {
        let inner = (coordinate, unit);

        let to_be_inserted =
            if let Some(olds) = self.units_coordinate_by_players.remove(&player_name) {
                let mut news = Vec::with_capacity(olds.len() + 1);
                news.push(inner);
                dbg!(&news);
                news
            } else {
                vec![inner]
            };

        if let None = self
            .units_coordinate_by_players
            .insert(player_name, to_be_inserted)
        {
            Ok(())
        } else {
            Err(RtsException::StoreUnitCoordinatesException(String::from(
                "Failed to insert new unit",
            )))
        }
    }

    pub fn get_coordinates(&self, player_name: String) -> Option<&Vec<InnerUnits>> {
        self.units_coordinate_by_players.get(&player_name)
    }
}

#[cfg(test)]
pub mod tests_units_holder {

    use super::UnitsHolder;
    use crate::components::unit_factory::UnitFactory;
    use crate::entity::unit::UnitType;

    #[test]
    pub fn should_store_and_then_get_units_coordinates_from_player() {
        let unit_factory = UnitFactory::default();
        let mut units_holder = UnitsHolder::default();

        if let Err(e) = units_holder.store(
            String::from("tigran"),
            (0, 0),
            unit_factory.build_unit(UnitType::Classic),
        ) {
            println!("{}", e);
            assert!(false);
        }
        if let Some(units) = units_holder.get_coordinates(String::from("tigran")) {
            if units.len() != 0 {
                assert!(false);
            }

            let (coordinate, unit) = &units[0];
            assert_eq!(&(0, 0), coordinate);
            assert_eq!(&20, unit.get_health());
        } else {
            assert!(false);
        }
    }
}
