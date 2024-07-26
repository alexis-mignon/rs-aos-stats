use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum DiceRoll {
    D6,
    D3,
    ND6(u32),
    ND3(u32),
    D6Plus(u32),
    D3Plus(u32),
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


#[derive(Clone, Hash, PartialEq, Eq)]
pub enum TestRollOutcome {
    Critical=2,
    Success=1,
    Failure=0
}

impl TestRollOutcome {
    pub fn outcome(roll: u32, threshold: u32, modifier: Option<u32>, has_critical: bool) -> TestRollOutcome{
        let modifier = modifier.unwrap_or(0);

        if roll == 1 {TestRollOutcome::Failure}
        else if has_critical && roll == 6 {TestRollOutcome::Critical}
        else if roll + modifier >= threshold {TestRollOutcome::Success}
        else {TestRollOutcome::Failure}
    }

    pub fn output_counts(threshold: u32, modifier: Option<u32>, has_critical: bool) -> HashMap<TestRollOutcome, u32>{
        let mut outcome_counts = HashMap::new();

        (1..=6).for_each(|roll| {
            let outcome = TestRollOutcome::outcome(roll, threshold, modifier, has_critical);
            *outcome_counts.entry(outcome).or_insert(0) += 1;
        });

        outcome_counts      
    }

    pub fn output_probabilities(threshold: u32, modifier: Option<u32>, has_critical: bool) -> HashMap<TestRollOutcome, f64> {
        let mut outcome_probabilities = HashMap::new();

        (1..=6).for_each(|roll| {
            let outcome = TestRollOutcome::outcome(roll, threshold, modifier, has_critical);
            *outcome_probabilities.entry(outcome).or_insert(0.0) += 1.0 / 6.0;
        });

        outcome_probabilities
    }
}
