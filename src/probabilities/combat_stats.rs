//use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use crate::probabilities::dice::DiceRoll;

#[derive(Clone, Copy, Debug)]
pub enum Characteristic {
    Value(u32),
    DiceRoll(DiceRoll),
}

impl Characteristic {
    pub fn values_and_probas(&self) -> Vec<(u32, f64)> {
        match self {
            Characteristic::Value(value) => vec![(*value, 1.0)],
            Characteristic::DiceRoll(dice) => dice.values_and_probas(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AttackStats {
    pub attacks: Characteristic,
    pub to_hit: u32,
    pub to_wound: u32,
    pub rend: u32,
    pub damages: Characteristic,
}

impl AttackStats {
    pub fn new(
        attacks: Characteristic,
        to_hit: u32,
        to_wound: u32,
        rend: u32,
        damages: Characteristic,
    ) -> AttackStats {
        AttackStats {
            attacks,
            to_hit,
            to_wound,
            rend,
            damages,
        }
    }

    pub fn with_attacks(&self, value: Characteristic) -> AttackStats {
        AttackStats {
            attacks: value,
            to_hit: self.to_hit,
            to_wound: self.to_wound,
            rend: self.rend,
            damages: self.damages
        }
    }

    pub fn with_damages(&self, value: Characteristic) -> AttackStats {
        AttackStats {
            attacks: self.attacks,
            to_hit: self.to_hit,
            to_wound: self.to_wound,
            rend: self.rend,
            damages: value
        }        
    }

    pub fn with_to_hit(&self, value: u32) -> AttackStats {
        AttackStats {
            attacks: self.attacks,
            to_hit: value,
            to_wound: self.to_wound,
            rend: self.rend,
            damages: self.damages
        }
    }

    pub fn with_to_wound(&self, value: u32) -> AttackStats {
        AttackStats {
            attacks: self.attacks,
            to_hit: self.to_hit,
            to_wound: value,
            rend: self.rend,
            damages: self.damages
        }
    }

    pub fn with_rend(&self, value: u32) -> AttackStats {
        AttackStats {
            attacks: self.attacks,
            to_hit: self.to_hit,
            to_wound: self.to_wound,
            rend: value,
            damages: self.damages
        }
    }

}


#[derive(Clone, Copy, Debug)]
pub struct DefenseStats {
    pub to_save: u32,
    pub ward: Option<u32>,
}

impl DefenseStats {
    pub fn new(to_save: u32, ward: Option<u32>) -> DefenseStats {
        DefenseStats { to_save: to_save, ward: ward }
    }

    pub fn with_to_save(&self, value: u32) -> DefenseStats {
        DefenseStats {to_save: value, ward: self.ward}
    }

    pub fn with_ward(&self, value: u32) -> DefenseStats {
        if value > 0 {
            DefenseStats {to_save: self.to_save, ward: Some(value)}
        }
        else {
            DefenseStats {to_save: self.to_save, ward: None}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RollModifier {
    pub to_hit: i32,
    pub to_wound: i32,
    pub to_save: i32,
}

impl RollModifier {
    pub fn new(to_hit: i32, to_wound: i32, to_save: i32) -> RollModifier {
        RollModifier {to_hit, to_wound, to_save}
    }

    pub fn new_null() -> RollModifier {
        RollModifier {to_hit: 0, to_wound: 0, to_save: 0}
    }
    
    fn apply_modifier(value: u32, modifier: i32, limit_low: i32, limit_high: i32) -> u32 {
        let modifier = match modifier {
            v if v < limit_low => limit_low,
            v if v > limit_high => limit_high,
            _ => modifier
        };
        let new_value = value as i32 + modifier;
        (match new_value {
            v if v < 0 => 0,
            _ => new_value
        }) as u32
    }

    pub fn apply_to_hit_modifier(&self, value: u32) -> u32 {
        RollModifier::apply_modifier(value, self.to_hit, -1, 1)
    }

    pub fn apply_to_wound_modifier(&self, value: u32) -> u32 {
        RollModifier::apply_modifier(value, self.to_wound, -1, 1)
    }

    pub fn apply_to_save_modifier(&self, value: u32) -> u32 {
        RollModifier::apply_modifier(value, self.to_wound, i32::MIN, 1)
    }
}

impl Add for RollModifier {
    type Output = RollModifier;
    fn add(self, other: RollModifier) -> RollModifier {
        RollModifier{
            to_hit: self.to_hit + other.to_hit,
            to_wound: self.to_wound + other.to_wound,
            to_save: self.to_save + other.to_save,
        }
    }
}

impl AddAssign for RollModifier {
    fn add_assign(&mut self, other: RollModifier) {
        self.to_hit += other.to_hit;
        self.to_wound += other.to_wound;
        self.to_save += other.to_save
    }
}
