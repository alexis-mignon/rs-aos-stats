use crate::probabilities::combat_stats::{AttackStats, Characteristic, DefenseStats};
use crate::probabilities::combat_tree::{compute_damages, CombatConfig, Rule};
use crate::probabilities::dice::DiceRoll;
use crate::probabilities::rules::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DamageProbability {
    pub damage: u32,
    pub probability: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CombatResult {
    pub probabilities: Vec<DamageProbability>,
    pub mean_damage: f64,
    pub max_damage: u32,
}

/// Compute damage probabilities for a given combat scenario
#[wasm_bindgen]
pub fn compute_combat_damage(
    attacks: u32,
    to_hit: u32,
    to_wound: u32,
    rend: u32,
    damage: u32,
    save: u32,
    ward: Option<u32>,
    hit_rule_type: &str,
) -> Result<JsValue, JsValue> {
    let attack_stats = AttackStats::new(
        Characteristic::Value(attacks),
        to_hit,
        to_wound,
        rend,
        Characteristic::Value(damage),
    );

    let defense_stats = DefenseStats::new(save, ward);
    let config = CombatConfig::new(attack_stats, defense_stats);

    // Build the rule sequence based on hit_rule_type
    let mut sequence: Vec<Box<dyn Rule>> = vec![Box::new(AttackCharacteristicRule)];

    // Add the appropriate hit rule
    match hit_rule_type {
        "normal" => sequence.push(Box::new(HitRule)),
        "crit_auto_wound" => sequence.push(Box::new(CritAutoWoundRule)),
        "crit_mortal_wound" => sequence.push(Box::new(CritMortalWoundRule)),
        "crit_double_hit" => sequence.push(Box::new(CritDoubleHitRule)),
        _ => return Err(JsValue::from_str("Invalid hit rule type")),
    }

    // Add remaining rules
    sequence.push(Box::new(WoundRule));
    sequence.push(Box::new(SaveRule));
    sequence.push(Box::new(DamagesRule));

    // Add ward rule if ward save exists
    if ward.is_some() {
        sequence.push(Box::new(WardRule));
    }

    // Compute damages
    let damages = compute_damages(config, &sequence);

    // Calculate statistics
    let mean_damage: f64 = damages.iter().map(|(d, p)| *d as f64 * p).sum();
    let max_damage = damages.iter().map(|(d, _)| *d).max().unwrap_or(0);

    let probabilities: Vec<DamageProbability> = damages
        .iter()
        .map(|(d, p)| DamageProbability {
            damage: *d,
            probability: *p,
        })
        .collect();

    let result = CombatResult {
        probabilities,
        mean_damage,
        max_damage,
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Compute damage probabilities with dice rolls for attacks
#[wasm_bindgen]
pub fn compute_combat_damage_with_dice_attacks(
    attacks_dice: &str,
    to_hit: u32,
    to_wound: u32,
    rend: u32,
    damage: u32,
    save: u32,
    ward: Option<u32>,
    hit_rule_type: &str,
) -> Result<JsValue, JsValue> {
    let dice_roll = match attacks_dice {
        "D3" => DiceRoll::D3,
        "D6" => DiceRoll::D6,
        _ => return Err(JsValue::from_str("Invalid dice type for attacks")),
    };

    let attack_stats = AttackStats::new(
        Characteristic::DiceRoll(dice_roll),
        to_hit,
        to_wound,
        rend,
        Characteristic::Value(damage),
    );

    let defense_stats = DefenseStats::new(save, ward);
    let config = CombatConfig::new(attack_stats, defense_stats);

    let mut sequence: Vec<Box<dyn Rule>> = vec![Box::new(AttackCharacteristicRule)];

    match hit_rule_type {
        "normal" => sequence.push(Box::new(HitRule)),
        "crit_auto_wound" => sequence.push(Box::new(CritAutoWoundRule)),
        "crit_mortal_wound" => sequence.push(Box::new(CritMortalWoundRule)),
        "crit_double_hit" => sequence.push(Box::new(CritDoubleHitRule)),
        _ => return Err(JsValue::from_str("Invalid hit rule type")),
    }

    sequence.push(Box::new(WoundRule));
    sequence.push(Box::new(SaveRule));
    sequence.push(Box::new(DamagesRule));

    if ward.is_some() {
        sequence.push(Box::new(WardRule));
    }

    let damages = compute_damages(config, &sequence);

    let mean_damage: f64 = damages.iter().map(|(d, p)| *d as f64 * p).sum();
    let max_damage = damages.iter().map(|(d, _)| *d).max().unwrap_or(0);

    let probabilities: Vec<DamageProbability> = damages
        .iter()
        .map(|(d, p)| DamageProbability {
            damage: *d,
            probability: *p,
        })
        .collect();

    let result = CombatResult {
        probabilities,
        mean_damage,
        max_damage,
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // WASM initialization code can go here if needed
}
