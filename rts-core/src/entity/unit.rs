use crate::components::play_ground::{HasIdentifier, Identifier};
use crate::entity::skill::Skill;
use crate::exceptions::RtsException;
use std::fmt::Display;

const CLASSIC_UNIT_COST: i16 = 20;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit {
    identifier: Identifier,
    max_health: i16,
    max_mana: i16,
    max_armor: i16,
    max_range: i16,

    // mutable fields
    health: i16,
    mana: i16,
    armor: i16,
    range: i16,
    skills: Vec<Skill>,
}

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Classic,
}

impl HasIdentifier for Unit {
    fn get_identifier(&self) -> Identifier {
        self.identifier
    }

    fn is(&self, identifier: &Identifier) -> bool {
        self.identifier.eq(identifier)
    }
}

impl UnitType {
    pub fn get_cost(&self) -> i16 {
        match &self {
            UnitType::Classic => CLASSIC_UNIT_COST,
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "h:{} m:{} a:{} r:{}",
            self.health, self.mana, self.armor, self.range
        )
    }
}

impl Unit {
    /// Create unit and initialize her stats at the top
    pub fn from(
        identifier: Identifier,
        max_health: i16,
        max_mana: i16,
        max_armor: i16,
        max_range: i16,
        skills: Vec<Skill>,
    ) -> Self {
        Unit {
            identifier,
            max_health,
            max_mana,
            max_armor,
            max_range,
            health: max_health,
            mana: max_mana,
            armor: max_armor,
            range: max_range,
            skills,
        }
    }

    pub fn update_health(&mut self, new_health: i16) -> Result<(), RtsException> {
        if let Some(res) = Unit::update_attribut_under_max(self.health, self.max_health, new_health)
        {
            self.health = res;
            Ok(())
        } else {
            Err(RtsException::GeneralException(String::from(
                "Failed to update health",
            )))
        }
    }

    pub fn update_mana(&mut self, new_mana: i16) -> Result<(), RtsException> {
        if let Some(res) = Unit::update_attribut_under_max(self.mana, self.max_mana, new_mana) {
            self.mana = res;
            Ok(())
        } else {
            Err(RtsException::GeneralException(String::from(
                "Failed to update mana",
            )))
        }
    }

    pub fn update_armor(&mut self, new_armor: i16) -> Result<(), RtsException> {
        if let Some(res) = Unit::update_attribut_under_max(self.armor, self.max_armor, new_armor) {
            self.armor = res;
            Ok(())
        } else {
            Err(RtsException::GeneralException(String::from(
                "Failed to update armor",
            )))
        }
    }

    pub fn update_range(&mut self, new_range: i16) -> Result<(), RtsException> {
        if let Some(res) = Unit::update_attribut_under_max(self.range, self.max_range, new_range) {
            self.range = res;
            Ok(())
        } else {
            Err(RtsException::GeneralException(String::from(
                "Failed to update range",
            )))
        }
    }

    /// if to_update_with is negative Unit is damaged
    /// or Unit is healed
    fn update_attribut_under_max(
        attribut: i16,
        max_attribut: i16,
        to_update_with: i16,
    ) -> Option<i16> {
        if let Some(res) = attribut.checked_add(to_update_with) {
            let to_return = if res >= max_attribut {
                max_attribut
            } else {
                res
            };
            Some(to_return)
        } else {
            None
        }
    }

    pub fn get_health(&self) -> &i16 {
        &self.health
    }

    pub fn get_mana(&self) -> &i16 {
        &self.mana
    }

    pub fn get_armor(&self) -> &i16 {
        &self.armor
    }

    pub fn get_range(&self) -> &i16 {
        &self.range
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            UnitType::Classic => write!(f, "Classic"),
        }
    }
}

#[cfg(test)]
mod test_unit {

    use super::Unit;

    // Damage cases
    #[test]
    pub fn should_lost_health() {
        let mut unit = Unit::from(1, 10, 0, 0, 0, Vec::new());
        if let Err(e) = unit.update_health(-2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&8, unit.get_health());
        }
    }

    #[test]
    pub fn should_use_mana() {
        let mut unit = Unit::from(1, 0, 10, 0, 0, Vec::new());
        if let Err(e) = unit.update_mana(-2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&8, unit.get_mana());
        }
    }

    #[test]
    pub fn should_update_by_dropping_armor() {
        let mut unit = Unit::from(1, 0, 0, 10, 0, Vec::new());
        if let Err(e) = unit.update_armor(-2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&8, unit.get_armor());
        }
    }

    #[test]
    pub fn should_update_by_dropping_range() {
        let mut unit = Unit::from(1, 0, 0, 0, 10, Vec::new());
        if let Err(e) = unit.update_range(-2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&8, unit.get_range());
        }
    }

    // Healing cases
    #[test]
    pub fn should_heal() {
        let mut unit = Unit::from(1, 10, 0, 0, 0, Vec::new());
        unit.update_health(-3).unwrap();
        if let Err(e) = unit.update_health(2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&9, unit.get_health());
        }
    }

    #[test]
    pub fn should_restore_mana() {
        let mut unit = Unit::from(1, 0, 10, 0, 0, Vec::new());
        unit.update_mana(-3).unwrap();
        if let Err(e) = unit.update_mana(2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&9, unit.get_mana());
        }
    }

    #[test]
    pub fn should_gain_armor() {
        let mut unit = Unit::from(1, 0, 0, 10, 0, Vec::new());
        unit.update_armor(-3).unwrap();
        if let Err(e) = unit.update_armor(2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&9, unit.get_armor());
        }
    }

    #[test]
    pub fn should_gain_range() {
        let mut unit = Unit::from(1, 0, 0, 0, 10, Vec::new());
        unit.update_range(-3).unwrap();
        if let Err(e) = unit.update_range(2) {
            println!("{}", e);
            assert!(false);
        } else {
            assert_eq!(&9, unit.get_range());
        }
    }
}
