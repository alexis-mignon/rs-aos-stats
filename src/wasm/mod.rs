use crate::probabilities::combat_stats::{AttackStats, Characteristic, DefenseStats};
use crate::probabilities::compute_engine::{compute_damages, CombatConfig, Rule};
use crate::probabilities::dice::DiceRoll;
use crate::probabilities::rules_impl::*;
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

#[derive(Serialize, Deserialize)]
pub struct CombatParams {
    pub attacks: u32,
    pub to_hit: u32,
    pub to_wound: u32,
    pub rend: u32,
    pub damage: u32,
    pub save: u32,
    pub ward: Option<u32>,
    pub hit_rule_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CombatParamsWithDice {
    pub attacks_dice: String,
    pub to_hit: u32,
    pub to_wound: u32,
    pub rend: u32,
    pub damage: u32,
    pub save: u32,
    pub ward: Option<u32>,
    pub hit_rule_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CombatParamsWithDiceCharacteristics {
    pub attacks_characteristic: String,
    pub to_hit: u32,
    pub to_wound: u32,
    pub rend: u32,
    pub damage_characteristic: String,
    pub save: u32,
    pub ward: Option<u32>,
    pub hit_rule_type: String,
}

/// Compute damage probabilities for a given combat scenario
#[wasm_bindgen]
pub fn compute_combat_damage(params: JsValue) -> Result<JsValue, JsValue> {
    let params: CombatParams =
        serde_wasm_bindgen::from_value(params).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let attack_stats = AttackStats::new(
        Characteristic::Value(params.attacks),
        params.to_hit,
        params.to_wound,
        params.rend,
        Characteristic::Value(params.damage),
    );

    let defense_stats = DefenseStats::new(params.save, params.ward);
    let config = CombatConfig::new(attack_stats, defense_stats);

    // Build the rule sequence based on hit_rule_type
    let mut sequence: Vec<Box<dyn Rule>> = vec![Box::new(AttackCharacteristicRule)];

    // Add the appropriate hit rule
    match params.hit_rule_type.as_str() {
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
    if params.ward.is_some() {
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
pub fn compute_combat_damage_with_dice_attacks(params: JsValue) -> Result<JsValue, JsValue> {
    let params: CombatParamsWithDice =
        serde_wasm_bindgen::from_value(params).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let dice_roll = match params.attacks_dice.as_str() {
        "D3" => DiceRoll::D3,
        "D6" => DiceRoll::D6,
        _ => return Err(JsValue::from_str("Invalid dice type for attacks")),
    };

    let attack_stats = AttackStats::new(
        Characteristic::DiceRoll(dice_roll),
        params.to_hit,
        params.to_wound,
        params.rend,
        Characteristic::Value(params.damage),
    );

    let defense_stats = DefenseStats::new(params.save, params.ward);
    let config = CombatConfig::new(attack_stats, defense_stats);

    let mut sequence: Vec<Box<dyn Rule>> = vec![Box::new(AttackCharacteristicRule)];

    match params.hit_rule_type.as_str() {
        "normal" => sequence.push(Box::new(HitRule)),
        "crit_auto_wound" => sequence.push(Box::new(CritAutoWoundRule)),
        "crit_mortal_wound" => sequence.push(Box::new(CritMortalWoundRule)),
        "crit_double_hit" => sequence.push(Box::new(CritDoubleHitRule)),
        _ => return Err(JsValue::from_str("Invalid hit rule type")),
    }

    sequence.push(Box::new(WoundRule));
    sequence.push(Box::new(SaveRule));
    sequence.push(Box::new(DamagesRule));

    if params.ward.is_some() {
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

/// Helper function to parse characteristic strings
fn parse_characteristic(s: &str) -> Result<Characteristic, String> {
    // Try parsing as fixed value first
    if let Ok(value) = s.parse::<u32>() {
        return Ok(Characteristic::Value(value));
    }

    // Otherwise parse as dice notation
    DiceRoll::from_str(s)
        .map(Characteristic::DiceRoll)
        .map_err(|e| format!("Invalid characteristic '{}': {:?}", s, e))
}

/// Compute damage probabilities with dice characteristics for both attacks and damage
#[wasm_bindgen]
pub fn compute_combat_damage_with_dice(params: JsValue) -> Result<JsValue, JsValue> {
    let params: CombatParamsWithDiceCharacteristics =
        serde_wasm_bindgen::from_value(params).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Parse attacks characteristic
    let attacks_char =
        parse_characteristic(&params.attacks_characteristic).map_err(|e| JsValue::from_str(&e))?;

    // Parse damage characteristic
    let damage_char =
        parse_characteristic(&params.damage_characteristic).map_err(|e| JsValue::from_str(&e))?;

    let attack_stats = AttackStats::new(
        attacks_char,
        params.to_hit,
        params.to_wound,
        params.rend,
        damage_char,
    );

    let defense_stats = DefenseStats::new(params.save, params.ward);
    let config = CombatConfig::new(attack_stats, defense_stats);

    // Build the rule sequence based on hit_rule_type
    let mut sequence: Vec<Box<dyn Rule>> = vec![Box::new(AttackCharacteristicRule)];

    // Add the appropriate hit rule
    match params.hit_rule_type.as_str() {
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
    if params.ward.is_some() {
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

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // WASM initialization code can go here if needed
}
