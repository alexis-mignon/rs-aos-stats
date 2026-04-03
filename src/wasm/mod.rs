/// WebAssembly-facing façade over the Rust probability engine.
///
/// Functions in this module are called from the JavaScript front-end to
/// compute exact damage distributions for given combat parameters.
use crate::probabilities::combat_stats::{AttackStats, Characteristic, DefenseStats};
use crate::probabilities::compute_engine::CombatConfig;
use crate::probabilities::dice::DiceRoll;
use crate::probabilities::rules_impl::{compute_damages, HitRuleVariant};
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

/// Helper: parse a hit rule type string into a HitRuleVariant.
fn parse_hit_variant(s: &str) -> Result<HitRuleVariant, JsValue> {
    s.parse::<HitRuleVariant>()
        .map_err(|e| JsValue::from_str(&e))
}

/// Helper: parse a characteristic string (fixed integer or dice notation).
fn parse_characteristic(s: &str) -> Result<Characteristic, String> {
    if let Ok(value) = s.parse::<u32>() {
        return Ok(Characteristic::Value(value));
    }
    DiceRoll::from_str(s)
        .map(Characteristic::DiceRoll)
        .map_err(|e| format!("Invalid characteristic '{}': {:?}", s, e))
}

/// Helper: run the pipeline and build a CombatResult.
fn build_result(config: CombatConfig, hit_variant: HitRuleVariant) -> CombatResult {
    let damages = compute_damages(config, hit_variant);

    let mean_damage: f64 = damages.iter().map(|(d, p)| *d as f64 * p).sum();
    let max_damage = damages.iter().map(|(d, _)| *d).max().unwrap_or(0);

    let probabilities: Vec<DamageProbability> = damages
        .iter()
        .map(|(d, p)| DamageProbability {
            damage: *d,
            probability: *p,
        })
        .collect();

    CombatResult {
        probabilities,
        mean_damage,
        max_damage,
    }
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
    let hit_variant = parse_hit_variant(&params.hit_rule_type)?;

    let result = build_result(config, hit_variant);
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
    let hit_variant = parse_hit_variant(&params.hit_rule_type)?;

    let result = build_result(config, hit_variant);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Compute damage probabilities with dice characteristics for both attacks and damage
#[wasm_bindgen]
pub fn compute_combat_damage_with_dice(params: JsValue) -> Result<JsValue, JsValue> {
    let params: CombatParamsWithDiceCharacteristics =
        serde_wasm_bindgen::from_value(params).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let attacks_char =
        parse_characteristic(&params.attacks_characteristic).map_err(|e| JsValue::from_str(&e))?;
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
    let hit_variant = parse_hit_variant(&params.hit_rule_type)?;

    let result = build_result(config, hit_variant);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // WASM initialization code can go here if needed
}
