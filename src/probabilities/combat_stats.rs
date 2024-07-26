use std::collections::HashMap;

use crate::probabilities::dice::DiceRoll;

#[derive(Clone)]
pub struct CombatStatus {
    pub n_attacks: u32,
    pub n_hits: u32,
    pub n_wounds: u32,
    pub n_mortal_wounds: u32,
    pub n_damages: u32,
}

impl CombatStatus {
    pub fn new() -> CombatStatus {
        CombatStatus {
            n_attacks: 0,
            n_hits: 0,
            n_wounds: 0,
            n_mortal_wounds: 0,
            n_damages: 0,
        }
    }

    pub fn from_status_attributes(attributes: HashMap<CombatStatusAttribute, u32>) -> CombatStatus{
        let mut status = CombatStatus::new();
        for (attr, count) in attributes.iter() {
            attr.update_status(&status, count)
        }
    }

    pub fn with_attacks(&self, n_attacks: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.n_attacks = n_attacks;
        new_status
    }

    pub fn with_hits(&self, n_hits: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.n_hits = n_hits;
        new_status
    }

    pub fn with_wounds(&self, n_wounds: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.n_wounds = n_wounds;
        new_status
    }

    pub fn with_mortal_wounds(&self, n_mortal_wounds: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.n_mortal_wounds = n_mortal_wounds;
        new_status
    }

    pub fn with_damages(&self, n_damages: u32) -> CombatStatus {
        let mut new_status = self.clone();
        new_status.n_damages = n_damages;
        new_status
    }
}

pub enum CombatStatusAttribute {
    Attacks,
    Hits,
    Wounds,
    MortalWounds,
    Damages
}

impl CombatStatusAttribute {
    pub fn update_status(&self, status: &CombatStatus, value: u32) -> CombatStatus {
        match *self {
            CombatStatusAttribute::Attacks => status.with_hits(value),
            CombatStatusAttribute::Hits => status.with_hits(value),
            CombatStatusAttribute::Wounds => status.with_wounds(value),
            CombatStatusAttribute::MortalWounds => status.with_mortal_wounds(value),
            CombatStatusAttribute::Damages => status.with_damages(value),
        }
    }
}



pub enum Quantity {
    Value(u32),
    DiceRoll(DiceRoll),
}

impl Quantity {
    pub fn values_and_probas(&self) -> Vec<(u32, f64)> {
        match self {
            StatType::Value(value) => vec![(*value, 1.0)],
            StatType::DiceRoll(dice) => dice.values_and_probas(),
        }
    }
}

#[derive(Clone)]
pub enum StatType {
    Threshold(u32),
    Quantity(Quantity),
}

#[derive(Clone)]
pub struct AttackStats {
    pub attacks: StatType,
    pub to_hit: StatType,
    pub to_wound: StatType,
    pub damages: StatType,
}

impl AttackStats {
    pub fn new(
        attacks: StatType,
        to_hit: StatType,
        to_wound: StatType,
        damages: StatType,
    ) -> AttackStats {
        AttackStats {
            attacks,
            to_hit,
            to_wound,
            damages,
        }
    }
}

#[derive(Clone)]
pub struct AttackStatsModifier {
    pub attacks: i32,
    pub to_hit: i32,
    pub to_wound: i32,
    pub damages: i32,
}

#[derive(Clone)]
pub struct DefenseStats {
    pub to_save: u32,
    pub ward: Option<u32>,
}

impl DefenseStats {
    pub fn new(to_save: u32, ward: Option<u32>) -> DefenseStats {
        DefenseStats { to_save, ward }
    }
}

#[derive(Clone)]
pub struct DefenseStatsModifier {
    pub to_save: i32,
    pub ward: i32,
}

impl DefenseStatsModifier {
    pub fn new(to_save: i32, ward: i32) -> DefenseStatsModifier {
        DefenseStatsModifier { to_save, ward }
    }
}
