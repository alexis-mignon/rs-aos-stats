use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum DiceRoll {
    D6,
    D3,
    ND6(u32),
    ND3(u32),
    D6Plus(u32),
    D3Plus(u32),
    ND3Plus(u32, u32),
    ND6Plus(u32, u32),
}

impl DiceRoll {
    pub fn values_and_probas(&self) -> Vec<(u32, f64)> {
        match self {
            DiceRoll::D6 => (1..=6).map(|x| (x, 1.0 / 6.0)).collect(),
            DiceRoll::D3 => (1..=3).map(|x| (x, 1.0 / 3.0)).collect(),
            DiceRoll::D6Plus(n) => (1..=6).map(|x| (x + n, 1.0 / 6.0)).collect(),
            DiceRoll::D3Plus(n) => (1..=3).map(|x| (x + n, 1.0 / 3.0)).collect(),
            DiceRoll::ND6(n) => _generate_dice_rolls(*n as usize, 6),
            DiceRoll::ND3(n) => _generate_dice_rolls(*n as usize, 3),
            DiceRoll::ND3Plus(n, m) => _generate_dice_rolls(*n as usize, 3)
                .iter()
                .map(|(x, proba)| (*x + m, *proba))
                .collect(),
            DiceRoll::ND6Plus(n, m) => _generate_dice_rolls(*n as usize, 6)
                .iter()
                .map(|(x, proba)| (*x + m, *proba))
                .collect(),
        }
    }
}

fn _generate_dice_rolls(n_dices: usize, n_faces: u32) -> Vec<(u32, f64)> {
    let mut rolls = Vec::new();
    _generate_dice_rolls_recursive(n_dices, n_faces, 0, 0, &mut rolls);

    let roll_counts: HashMap<u32, u32> = rolls.iter().fold(HashMap::new(), |mut acc, roll| {
        *acc.entry(*roll).or_insert(0) += 1;
        acc
    });

    let proba_n_dice_rolls = 1.0 / (n_faces.pow(n_dices as u32) as f64);
    roll_counts
        .iter()
        .map(|(roll, count)| (*roll, *count as f64 * proba_n_dice_rolls))
        .collect()
}

fn _generate_dice_rolls_recursive(
    n_dices: usize,
    n_faces: u32,
    current_dice_index: usize,
    current_roll: u32,
    rolls: &mut Vec<u32>,
) {
    if current_dice_index == n_dices {
        rolls.push(current_roll);
        return;
    }

    for i in 1..=n_faces {
        _generate_dice_rolls_recursive(
            n_dices,
            n_faces,
            current_dice_index + 1,
            current_roll + i,
            rolls,
        );
    }
}

impl DiceRoll {
    /// Parses a dice notation string. This is a convenience wrapper around the FromStr trait.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(dice_str: &str) -> Result<DiceRoll, DiceRollParseError> {
        dice_str.parse()
    }
}

impl FromStr for DiceRoll {
    type Err = DiceRollParseError;

    fn from_str(dice_str: &str) -> Result<DiceRoll, DiceRollParseError> {
        let re = Regex::new(r"(?<n>\d+)?D(?<faces>[36])(\+(?<bonus>\d+))?")
            .map_err(|_| DiceRollParseError::InvalidRegex)?;

        if let Some(captures) = re.captures(dice_str) {
            let n = captures
                .name("n")
                .map_or(1, |m| m.as_str().parse().unwrap());
            let faces = captures
                .name("faces")
                .map(|m| m.as_str().parse().unwrap())
                .unwrap();
            let bonus = captures
                .name("bonus")
                .map_or(0, |m| m.as_str().parse().unwrap());

            if n == 1 {
                match faces {
                    3 => match bonus {
                        0 => Ok(DiceRoll::D3),
                        _ => Ok(DiceRoll::D3Plus(bonus)),
                    },
                    6 => match bonus {
                        0 => Ok(DiceRoll::D6),
                        _ => Ok(DiceRoll::D6Plus(bonus)),
                    },
                    _ => Err(DiceRollParseError::InvalidFaceNumber),
                }
            } else {
                match faces {
                    3 => match bonus {
                        0 => Ok(DiceRoll::ND3(n)),
                        _ => Ok(DiceRoll::ND3Plus(n, bonus)),
                    },
                    6 => match bonus {
                        0 => Ok(DiceRoll::ND6(n)),
                        _ => Ok(DiceRoll::ND6Plus(n, bonus)),
                    },
                    _ => Err(DiceRollParseError::InvalidFaceNumber),
                }
            }
        } else {
            Err(DiceRollParseError::InvalidFormat)
        }
    }
}

#[derive(Debug, Clone)]
pub enum DiceRollParseError {
    InvalidRegex,
    InvalidFaceNumber,
    InvalidFormat,
}

impl fmt::Display for DiceRollParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parsing "D6" should produce a single D6 variant.
    #[test]
    fn parse_d6() {
        let roll = DiceRoll::from_str("D6").unwrap();
        assert!(matches!(roll, DiceRoll::D6));
    }

    /// Parsing "D3" should produce a single D3 variant.
    #[test]
    fn parse_d3() {
        let roll = DiceRoll::from_str("D3").unwrap();
        assert!(matches!(roll, DiceRoll::D3));
    }

    /// Parsing "2D6" should produce ND6(2) — two six-sided dice.
    #[test]
    fn parse_2d6() {
        let roll = DiceRoll::from_str("2D6").unwrap();
        assert!(matches!(roll, DiceRoll::ND6(2)));
    }

    /// Parsing "D6+1" should produce D6Plus(1) — one D6 with a +1 bonus.
    #[test]
    fn parse_d6_plus_1() {
        let roll = DiceRoll::from_str("D6+1").unwrap();
        assert!(matches!(roll, DiceRoll::D6Plus(1)));
    }

    /// Parsing "2D3+1" should produce ND3Plus(2, 1) — two D3 with a +1 bonus.
    #[test]
    fn parse_2d3_plus_1() {
        let roll = DiceRoll::from_str("2D3+1").unwrap();
        assert!(matches!(roll, DiceRoll::ND3Plus(2, 1)));
    }

    /// An unrecognized string like "invalid" should return an error.
    #[test]
    fn parse_invalid() {
        let result = DiceRoll::from_str("invalid");
        assert!(result.is_err());
    }

    /// A D6 should yield exactly 6 outcomes, each with probability 1/6,
    /// and the total probability should sum to 1.
    #[test]
    fn d6_probabilities() {
        let roll = DiceRoll::D6;
        let vp = roll.values_and_probas();
        assert_eq!(vp.len(), 6);
        for (_, p) in &vp {
            assert!((*p - 1.0 / 6.0).abs() < 1e-10);
        }
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// A D3 should yield exactly 3 outcomes whose probabilities sum to 1.
    #[test]
    fn d3_probabilities() {
        let roll = DiceRoll::D3;
        let vp = roll.values_and_probas();
        assert_eq!(vp.len(), 3);
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// 2D6 should only produce sums in the range [2, 12],
    /// and the total probability should sum to 1.
    #[test]
    fn nd6_sum_range() {
        let roll = DiceRoll::ND6(2);
        let vp = roll.values_and_probas();
        for (v, _) in &vp {
            assert!(*v >= 2 && *v <= 12);
        }
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// D3Plus should add a bonus to each D3 outcome,
    /// producing values [1+bonus, 3+bonus].
    #[test]
    fn d3_plus_probabilities() {
        let roll = DiceRoll::D3Plus(2);
        let vp = roll.values_and_probas();
        assert_eq!(vp.len(), 3);
        // Should have values 3, 4, 5 (1+2, 2+2, 3+2)
        assert!(vp.iter().any(|(v, _)| *v == 3));
        assert!(vp.iter().any(|(v, _)| *v == 4));
        assert!(vp.iter().any(|(v, _)| *v == 5));
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// D6Plus should add a bonus to each D6 outcome,
    /// producing values [1+bonus, 6+bonus].
    #[test]
    fn d6_plus_probabilities() {
        let roll = DiceRoll::D6Plus(1);
        let vp = roll.values_and_probas();
        assert_eq!(vp.len(), 6);
        // Should have values 2-7
        assert!(vp.iter().all(|(v, _)| *v >= 2 && *v <= 7));
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// ND3 should produce sums in the range [n, 3*n].
    #[test]
    fn nd3_probabilities() {
        let roll = DiceRoll::ND3(2);
        let vp = roll.values_and_probas();
        // Range should be [2, 6]
        assert!(vp.iter().all(|(v, _)| *v >= 2 && *v <= 6));
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// ND3Plus should produce sums in the range [n+bonus, 3*n+bonus].
    #[test]
    fn nd3_plus_probabilities() {
        let roll = DiceRoll::ND3Plus(2, 1);
        let vp = roll.values_and_probas();
        // Range should be [3, 7] (2d3 gives 2-6, +1 = 3-7)
        assert!(vp.iter().all(|(v, _)| *v >= 3 && *v <= 7));
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// ND6Plus should produce sums in the range [n+bonus, 6*n+bonus].
    #[test]
    fn nd6_plus_probabilities() {
        let roll = DiceRoll::ND6Plus(2, 1);
        let vp = roll.values_and_probas();
        // Range should be [3, 13] (2d6 gives 2-12, +1 = 3-13)
        assert!(vp.iter().all(|(v, _)| *v >= 3 && *v <= 13));
        let total: f64 = vp.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    /// Parsing "2D6+3" should produce ND6Plus(2, 3).
    #[test]
    fn parse_2d6_plus_3() {
        let roll = DiceRoll::from_str("2D6+3").unwrap();
        assert!(matches!(roll, DiceRoll::ND6Plus(2, 3)));
    }

    /// Parsing "D3+2" should produce D3Plus(2).
    #[test]
    fn parse_d3_plus_2() {
        let roll = DiceRoll::from_str("D3+2").unwrap();
        assert!(matches!(roll, DiceRoll::D3Plus(2)));
    }

    /// The DiceRollParseError Display trait should produce
    /// a readable error message.
    #[test]
    fn dice_roll_parse_error_display() {
        let err = DiceRollParseError::InvalidFormat;
        let display = format!("{}", err);
        assert!(display.contains("InvalidFormat"));

        let err = DiceRollParseError::InvalidFaceNumber;
        let display = format!("{}", err);
        assert!(display.contains("InvalidFaceNumber"));
    }
}
