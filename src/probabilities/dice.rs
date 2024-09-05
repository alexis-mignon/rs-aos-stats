use std::collections::HashMap;
use regex::Regex;
use std::fmt;

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
    pub fn from_str(dice_str: String) -> Result<DiceRoll, DiceRollParseError> {
        let re = Regex::new(r"(?<n>\d+)?D(?<faces>[36])(\+(?<bonus>\d+))?").map_err(|_| DiceRollParseError::InvalidRegex)?;
        
        if let Some(captures) = re.captures(&dice_str) {
            let n = captures.name("n").map_or(1, |m| m.as_str().parse().unwrap());
            let faces = captures.name("faces").map(|m| m.as_str().parse().unwrap()).unwrap();
            let bonus = captures.name("bonus").map_or(0, |m| m.as_str().parse().unwrap());

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
    InvalidFormat
}

impl fmt::Display for DiceRollParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}