use regex::Regex;
use std::fmt;
use std::str::FromStr;

use crate::probabilities::convolution::convolve_n;

pub(crate) const MAX_DICE_COUNT: u32 = 50;

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
            DiceRoll::D6 => single_die_distribution(6),
            DiceRoll::D3 => single_die_distribution(3),
            DiceRoll::D6Plus(n) => shift_distribution(&single_die_distribution(6), *n),
            DiceRoll::D3Plus(n) => shift_distribution(&single_die_distribution(3), *n),
            DiceRoll::ND6(n) => generate_dice_sum_distribution(*n, 6),
            DiceRoll::ND3(n) => generate_dice_sum_distribution(*n, 3),
            DiceRoll::ND3Plus(n, m) => {
                shift_distribution(&generate_dice_sum_distribution(*n, 3), *m)
            }
            DiceRoll::ND6Plus(n, m) => {
                shift_distribution(&generate_dice_sum_distribution(*n, 6), *m)
            }
        }
    }
}

fn single_die_distribution(n_faces: u32) -> Vec<(u32, f64)> {
    match n_faces {
        3 => (1..=3).map(|value| (value, 1.0 / 3.0)).collect(),
        6 => (1..=6).map(|value| (value, 1.0 / 6.0)).collect(),
        _ => unreachable!("unsupported face count: {n_faces}"),
    }
}

fn generate_dice_sum_distribution(n_dices: u32, n_faces: u32) -> Vec<(u32, f64)> {
    assert!(
        n_dices <= MAX_DICE_COUNT,
        "dice count {} exceeds maximum of {}",
        n_dices,
        MAX_DICE_COUNT
    );
    let single_die = single_die_distribution(n_faces);
    convolve_n(&single_die, n_dices)
}

fn shift_distribution(dist: &[(u32, f64)], offset: u32) -> Vec<(u32, f64)> {
    dist.iter()
        .map(|(value, probability)| (*value + offset, *probability))
        .collect()
}

impl DiceRoll {
    /// Parses a dice notation string. This is a convenience wrapper around the FromStr trait.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(dice_str: &str) -> Result<DiceRoll, DiceRollParseError> {
        dice_str.parse()
    }
}

pub(crate) fn validate_dice_count(n_dices: u32) -> Result<(), DiceRollParseError> {
    if n_dices > MAX_DICE_COUNT {
        Err(DiceRollParseError::TooManyDice {
            count: n_dices,
            max: MAX_DICE_COUNT,
        })
    } else {
        Ok(())
    }
}

impl FromStr for DiceRoll {
    type Err = DiceRollParseError;

    fn from_str(dice_str: &str) -> Result<DiceRoll, DiceRollParseError> {
        let re = Regex::new(r"^(?<n>[1-9]\d*)?D(?<faces>[36])(?:\+(?<bonus>\d+))?$")
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

            validate_dice_count(n)?;

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
    TooManyDice { count: u32, max: u32 },
}

impl fmt::Display for DiceRollParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiceRollParseError::InvalidRegex => write!(f, "InvalidRegex"),
            DiceRollParseError::InvalidFaceNumber => write!(f, "InvalidFaceNumber"),
            DiceRollParseError::InvalidFormat => write!(f, "InvalidFormat"),
            DiceRollParseError::TooManyDice { count, max } => {
                write!(f, "dice count {count} exceeds maximum of {max}")
            }
        }
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

    /// Strings must match the dice notation exactly; extra characters or
    /// zero-count dice are invalid.
    #[test]
    fn parse_rejects_malformed_strings() {
        for invalid in ["xD6", "D6junk", "2D6foo", "D3+2x", "0D6"] {
            assert!(DiceRoll::from_str(invalid).is_err());
        }
    }

    /// The dice parser rejects more than 50 dice in a single notation.
    #[test]
    fn parse_rejects_too_many_dice() {
        for invalid in ["51D6", "51D3+1"] {
            assert!(matches!(
                DiceRoll::from_str(invalid),
                Err(DiceRollParseError::TooManyDice { .. })
            ));
        }
    }

    /// The upper bound still allows exactly 50 dice.
    #[test]
    fn parse_accepts_fifty_dice() {
        let roll = DiceRoll::from_str("50D6").unwrap();
        assert!(matches!(roll, DiceRoll::ND6(50)));
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

    /// Larger dice pools should be computed via convolution rather than
    /// recursive expansion, while preserving the exact range and mass.
    #[test]
    fn nd6_large_pool_probabilities() {
        let roll = DiceRoll::ND6(10);
        let vp = roll.values_and_probas();

        assert_eq!(vp.first().unwrap().0, 10);
        assert_eq!(vp.last().unwrap().0, 60);
        assert_eq!(vp.len(), 51);

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
