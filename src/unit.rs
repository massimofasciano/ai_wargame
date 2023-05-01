use crate::{UnitType, Health, MAX_HEALTH, HeuristicScore};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Unit {
    pub (crate) unit_type : UnitType,
    pub (crate) health : Health,
}

impl Default for Unit {
    fn default() -> Self {
        Self::new(UnitType::default())
    }
}

impl Unit {
    pub fn new(unit_type : UnitType) -> Self {
        let health = unit_type.initial_health();
        assert!(health <= MAX_HEALTH);
        Self { unit_type, health }
    }
    pub fn can_repair(&self, target: &Self) -> bool {
        assert!(target.health <= MAX_HEALTH);
        let repair = self.unit_type.repair_amount(&target.unit_type);
        let max_health = target.unit_type.initial_health();
        // repair != 0 && target.health != MAX_HEALTH
        repair != 0 && target.health != max_health
    }
    pub fn apply_repair(&mut self, target: &mut Self) -> u8 {
        assert!(target.health <= MAX_HEALTH);
        let repair = self.unit_type.repair_amount(&target.unit_type);
        let max_health = target.unit_type.initial_health();
        // if repair + target.health < MAX_HEALTH {
        if repair + target.health < max_health {
            target.health += repair;
        } else {
            target.health = max_health;
        }
        repair
    }
    pub fn can_damage(&self, target: &Self) -> bool {
        assert!(target.health <= MAX_HEALTH);
        let damage = self.unit_type.damage_amount(&target.unit_type);
        damage != 0
    }
    pub fn apply_damage(&mut self, target: &mut Self) -> Health {
        assert!(target.health <= MAX_HEALTH);
        let damage = self.unit_type.damage_amount(&target.unit_type);
        if damage < target.health {
            target.health -= damage;
        } else {
            target.health = 0;
        }
        damage
    }
    pub fn clone_apply_damage(&self, target: &Self, is_critical: bool) -> (Self, Health) {
        assert!(target.health <= MAX_HEALTH);
        let mut damage = self.unit_type.damage_amount(&target.unit_type);
        if is_critical {
            damage *= 2;
        }
        let mut target = target.clone();
        if damage < target.health {
            target.health -= damage;
        } else {
            target.health = 0;
        }
        (target, damage)
    }
    pub fn score(&self) -> HeuristicScore {
        self.unit_type.score()
    }
}

