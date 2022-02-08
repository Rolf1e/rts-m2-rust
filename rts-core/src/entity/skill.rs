pub struct Skill {
    mana_cost: i8,
    damage: i16,
    cooldown: i32, // ms

    // mutable fields
    actual_cooldown: i32, // ms
    level: i8,
}

impl Skill {
    /// Create a skill object and init his actual fields at the top
    pub fn from(mana_cost: i8, damage: i16, cooldown: i32) -> Self {
        Skill {
            mana_cost,
            damage,
            cooldown,
            actual_cooldown: cooldown,
            level: 0,
        }
    }

    pub fn update_level(&mut self) {
        self.level += 1
    }

    pub fn update_actual_cooldown(&mut self, new_cooldown: i32) {
        self.actual_cooldown = new_cooldown
    }

    pub fn get_actual_cooldown(&self) -> &i32 {
        &self.actual_cooldown
    }

    pub fn get_mana_cost(&self) -> &i8 {
        &self.mana_cost
    }

    pub fn get_damage(&self) -> &i16 {
        &self.damage
    }

    pub fn get_cooldown(&self) -> &i32 {
        &self.cooldown
    }
}
