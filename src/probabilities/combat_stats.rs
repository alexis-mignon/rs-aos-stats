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
        DefenseStats { to_save, ward }
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
        RollModifier::apply_modifier(value, self.to_save, i32::MIN, 1)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probabilities::dice::DiceRoll;

    /// A positive to_hit modifier of +1 should increase the roll value by 1.
    #[test]
    fn apply_to_hit_positive_modifier() {
        let m = RollModifier::new(1, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(3), 4);
    }

    /// A negative to_hit modifier of -1 should decrease the roll value by 1.
    #[test]
    fn apply_to_hit_negative_modifier() {
        let m = RollModifier::new(-1, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(3), 2);
    }

    /// Hit/wound modifiers are clamped to [-1, +1] per AoS rules,
    /// so a +3 modifier should behave the same as +1.
    #[test]
    fn apply_to_hit_clamped_modifier() {
        let m = RollModifier::new(3, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(3), 4);
    }

    /// Regression test for the bug where apply_to_save_modifier read
    /// self.to_wound instead of self.to_save. With to_wound=5 and
    /// to_save=-1, the result must reflect the to_save field.
    #[test]
    fn apply_to_save_modifier_uses_save_field() {
        let m = RollModifier::new(0, 5, -1);
        assert_eq!(m.apply_to_save_modifier(4), 3);
    }

    /// The Add trait should combine two RollModifiers field-by-field.
    #[test]
    fn roll_modifier_add() {
        let a = RollModifier::new(1, 2, 3);
        let b = RollModifier::new(-1, -2, -3);
        let c = a + b;
        assert_eq!(c.to_hit, 0);
        assert_eq!(c.to_wound, 0);
        assert_eq!(c.to_save, 0);
    }

    /// The AddAssign trait (+=) should accumulate modifiers in place.
    #[test]
    fn roll_modifier_add_assign() {
        let mut a = RollModifier::new(1, 0, 0);
        a += RollModifier::new(0, 1, 1);
        assert_eq!(a.to_hit, 1);
        assert_eq!(a.to_wound, 1);
        assert_eq!(a.to_save, 1);
    }

    /// A fixed Characteristic should return a single outcome with probability 1.
    #[test]
    fn characteristic_fixed_value() {
        let c = Characteristic::Value(3);
        let vp = c.values_and_probas();
        assert_eq!(vp, vec![(3, 1.0)]);
    }

    /// A dice-roll Characteristic (D6) should return 6 outcomes
    /// whose probabilities sum to 1.
    #[test]
    fn characteristic_dice_roll() {
        let c = Characteristic::DiceRoll(DiceRoll::D6);
        let vp = c.values_and_probas();
        assert_eq!(vp.len(), 6);
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// Test all AttackStats builder methods (with_*) to ensure they
    /// correctly modify individual fields while preserving others.
    #[test]
    fn attack_stats_with_methods() {
        let stats = AttackStats::new(
            Characteristic::Value(5),
            3, 4, 1,
            Characteristic::Value(2)
        );

        // Test with_attacks
        let modified = stats.with_attacks(Characteristic::Value(10));
        assert_eq!(modified.attacks.values_and_probas(), vec![(10, 1.0)]);
        assert_eq!(modified.to_hit, 3);

        // Test with_to_hit
        let modified = stats.with_to_hit(2);
        assert_eq!(modified.to_hit, 2);
        assert_eq!(modified.to_wound, 4);

        // Test with_to_wound
        let modified = stats.with_to_wound(5);
        assert_eq!(modified.to_wound, 5);
        assert_eq!(modified.rend, 1);

        // Test with_rend
        let modified = stats.with_rend(3);
        assert_eq!(modified.rend, 3);

        // Test with_damages
        let modified = stats.with_damages(Characteristic::Value(3));
        assert_eq!(modified.damages.values_and_probas(), vec![(3, 1.0)]);
        assert_eq!(modified.to_hit, 3);
    }

    /// Test DefenseStats builder methods (with_*) including the
    /// special behavior of with_ward that converts 0 to None.
    #[test]
    fn defense_stats_with_methods() {
        let stats = DefenseStats::new(4, None);

        // Test with_to_save
        let modified = stats.with_to_save(3);
        assert_eq!(modified.to_save, 3);
        assert_eq!(modified.ward, None);

        // Test with_ward with positive value
        let modified = stats.with_ward(5);
        assert_eq!(modified.ward, Some(5));

        // Test with_ward with zero (should be None per business logic)
        let modified = stats.with_ward(0);
        assert_eq!(modified.ward, None);

        // Test with_ward preserves to_save
        let stats = DefenseStats::new(4, Some(6));
        let modified = stats.with_ward(5);
        assert_eq!(modified.to_save, 4);
        assert_eq!(modified.ward, Some(5));
    }

    /// Test RollModifier::new_null creates a modifier with all zeros.
    #[test]
    fn roll_modifier_new_null() {
        let m = RollModifier::new_null();
        assert_eq!(m.to_hit, 0);
        assert_eq!(m.to_wound, 0);
        assert_eq!(m.to_save, 0);
    }

    /// Test apply_to_wound_modifier behaves correctly with positive
    /// and negative modifiers, including clamping.
    #[test]
    fn apply_to_wound_modifier() {
        // Positive modifier
        let m = RollModifier::new(0, 1, 0);
        assert_eq!(m.apply_to_wound_modifier(3), 4);

        // Negative modifier
        let m = RollModifier::new(0, -1, 0);
        assert_eq!(m.apply_to_wound_modifier(3), 2);

        // Clamped modifier (should behave as +1)
        let m = RollModifier::new(0, 5, 0);
        assert_eq!(m.apply_to_wound_modifier(3), 4);
    }

    /// Test that modifiers resulting in negative values are clamped to 0.
    #[test]
    fn apply_modifier_negative_result() {
        // Large negative modifier on small value should result in 0
        let m = RollModifier::new(-10, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(1), 0);

        let m = RollModifier::new(0, -10, 0);
        assert_eq!(m.apply_to_wound_modifier(1), 0);
    }

    /// Test that negative modifiers below the limit are clamped to -1
    /// for hit and wound rolls (per AoS rules).
    #[test]
    fn apply_modifier_below_limit() {
        // Modifier of -10 should be clamped to -1 for hit rolls
        let m = RollModifier::new(-10, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(5), 4);

        // Modifier of -10 should be clamped to -1 for wound rolls
        let m = RollModifier::new(0, -10, 0);
        assert_eq!(m.apply_to_wound_modifier(5), 4);
    }

    /// Test that save modifiers can be very negative (rend effect).
    #[test]
    fn apply_to_save_large_negative() {
        // Save modifiers don't have a lower limit, so -10 should work
        let m = RollModifier::new(0, 0, -10);
        // A save of 11 with -10 modifier becomes 1 (11-10=1)
        assert_eq!(m.apply_to_save_modifier(11), 1);
    }

    /// Test boundary case where positive modifier is exactly at the +1 limit.
    #[test]
    fn apply_modifier_exactly_at_positive_limit() {
        // Modifier of exactly +1 should work without clamping
        let m = RollModifier::new(1, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(4), 5);

        let m = RollModifier::new(0, 1, 0);
        assert_eq!(m.apply_to_wound_modifier(4), 5);

        // Save modifier of +1 should also work
        let m = RollModifier::new(0, 0, 1);
        assert_eq!(m.apply_to_save_modifier(4), 5);
    }

    /// Test boundary case where negative modifier is exactly at the -1 limit.
    #[test]
    fn apply_modifier_exactly_at_negative_limit() {
        // Modifier of exactly -1 should work without clamping
        let m = RollModifier::new(-1, 0, 0);
        assert_eq!(m.apply_to_hit_modifier(4), 3);

        let m = RollModifier::new(0, -1, 0);
        assert_eq!(m.apply_to_wound_modifier(4), 3);
    }
}
