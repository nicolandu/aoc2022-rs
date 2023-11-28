use crate::{Choice::*, Outcome::*};
use std::{error::Error, io};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Choice {
    fn value(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Choice {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn beaten_by(&self) -> Choice {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

impl Outcome {
    /// Determine outcome from a game.
    fn from_game(us: Choice, opponent: Choice) -> Outcome {
        if opponent == us.beats() {
            return Win;
        }

        if opponent == us {
            return Draw;
        }

        Loss
    }

    /// Determine response which will give this outcome.
    fn to_response(self, opponent: Choice) -> Choice {
        match self {
            Win => opponent.beaten_by(),
            Draw => opponent,
            Loss => opponent.beats(),
        }
    }
}

/// Computes points from a strategy guide line (part 1).
/// # Arguments
/// * `data` - Strategy guide line.
/// Format: \[ABC\]<space>\[XYZ\]
///
/// A, X : Rock
/// B, Y : Paper
/// C, Z : Scissors
///
/// A, B, C : Opponent
/// X, Y, Z : Our response
/// # Examples
/// ```
/// assert_eq!("A Y", Some(8))
fn points_part1(data: &str) -> Option<u32> {
    if data.len() < 3 {
        return None;
    }

    let opponent = match data.chars().next().unwrap() {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => panic!("First character must be 'A', 'B' or 'C'"),
    };

    let us = match data.chars().nth(2).unwrap() {
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => panic!("Second character must be 'X', 'Y' or 'Z'"),
    };

    Some(match Outcome::from_game(us, opponent) {
        Win => 6 + us.value(),
        Draw => 3 + us.value(),
        Loss => us.value(),
    })
}

/// Computes points from a strategy guide line (part 2).
/// # Arguments
/// * `data` - Strategy guide line.
/// Format: \[ABC\]<space>\[XYZ\]
///
/// A : Rock
/// B : Paper
/// C : Scissors
///
/// X : Loss
/// Y : Draw
/// Z : Win
///
/// A, B, C : Opponent
/// X, Y, Z : Our response
/// # Examples
/// ```
/// assert_eq!("A Y", Some(8))
fn points_part2(data: &str) -> Option<u32> {
    if data.len() < 3 {
        return None;
    }

    let opponent = match data.chars().next().unwrap() {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => panic!("First character must be 'A', 'B' or 'C'"),
    };

    let expected = match data.chars().nth(2).unwrap() {
        'X' => Loss,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!("Second character must be 'X', 'Y' or 'Z'"),
    };

    Some(
        match expected {
            Win => 6,
            Draw => 3,
            Loss => 0,
        } + expected.to_response(opponent).value(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = io::read_to_string(io::stdin())?;

    let sum1: u32 = content
        .lines()
        .map(|data| points_part1(data).unwrap_or(0))
        .sum();
    let sum2: u32 = content
        .lines()
        .map(|data| points_part2(data).unwrap_or(0))
        .sum();

    println!("Score: {sum1} (part 1), {sum2} (part 2)");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_from_game() {
        assert_eq!(Outcome::from_game(Rock, Scissors), Win);
        assert_eq!(Outcome::from_game(Rock, Rock), Draw);
        assert_eq!(Outcome::from_game(Rock, Paper), Loss);

        assert_eq!(Outcome::from_game(Paper, Rock), Win);
        assert_eq!(Outcome::from_game(Paper, Paper), Draw);
        assert_eq!(Outcome::from_game(Paper, Scissors), Loss);

        assert_eq!(Outcome::from_game(Scissors, Paper), Win);
        assert_eq!(Outcome::from_game(Scissors, Scissors), Draw);
        assert_eq!(Outcome::from_game(Scissors, Rock), Loss);
    }

    #[test]
    fn outcome_to_response() {
        assert_eq!(Win.to_response(Rock), Paper);
        assert_eq!(Win.to_response(Paper), Scissors);
        assert_eq!(Win.to_response(Scissors), Rock);

        assert_eq!(Draw.to_response(Rock), Rock);
        assert_eq!(Draw.to_response(Paper), Paper);
        assert_eq!(Draw.to_response(Scissors), Scissors);

        assert_eq!(Loss.to_response(Rock), Scissors);
        assert_eq!(Loss.to_response(Paper), Rock);
        assert_eq!(Loss.to_response(Scissors), Paper);
    }

    #[test]
    fn wins_part1() {
        assert_eq!(points_part1("A Y"), Some(8));
        assert_eq!(points_part1("B Z"), Some(9));
        assert_eq!(points_part1("C X"), Some(7));
    }

    #[test]
    fn ties_part1() {
        assert_eq!(points_part1("A X"), Some(4));
        assert_eq!(points_part1("B Y"), Some(5));
        assert_eq!(points_part1("C Z"), Some(6));
    }

    #[test]
    fn losses_part1() {
        assert_eq!(points_part1("A Z"), Some(3));
        assert_eq!(points_part1("B X"), Some(1));
        assert_eq!(points_part1("C Y"), Some(2));
    }

    #[test]
    fn wins_part2() {
        assert_eq!(points_part2("A Z"), Some(8));
        assert_eq!(points_part2("B Z"), Some(9));
        assert_eq!(points_part2("C Z"), Some(7));
    }

    #[test]
    fn ties_part2() {
        assert_eq!(points_part2("A Y"), Some(4));
        assert_eq!(points_part2("B Y"), Some(5));
        assert_eq!(points_part2("C Y"), Some(6));
    }

    #[test]
    fn losses_part2() {
        assert_eq!(points_part2("A X"), Some(3));
        assert_eq!(points_part2("B X"), Some(1));
        assert_eq!(points_part2("C X"), Some(2));
    }
}
