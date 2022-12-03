use crate::Choice::{Paper, Rock, Scissors};
use crate::Outcome::{Draw, Lose, Win};
use std::str::FromStr;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

impl Choice {
    fn get_move_score(&self) -> u32 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn get_outcome_score(&self, other: &Self) -> u32 {
        match *self {
            Rock => match other {
                Rock => 3,
                Paper => 0,
                Scissors => 6,
            },
            Paper => match other {
                Rock => 6,
                Paper => 3,
                Scissors => 0,
            },
            Scissors => match other {
                Rock => 0,
                Paper => 6,
                Scissors => 3,
            },
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

fn compute_match_score(opponent_move: &Choice, response_move: &Choice) -> u32 {
    response_move.get_outcome_score(opponent_move) + response_move.get_move_score()
}

fn compute_strategy_score_from_moves(strategy_string: &str) -> u32 {
    let mut score = 0;
    let strategy_lines = strategy_string.lines();

    for line in strategy_lines {
        let mut moves = line.split_whitespace();
        let opponent_move = Choice::from_str(moves.next().unwrap()).unwrap();
        let response_move = Choice::from_str(moves.next().unwrap()).unwrap();

        score += compute_match_score(&opponent_move, &response_move);
    }

    score
}

fn compute_strategy_score_from_move_and_outcome(strategy_string: &str) -> u32 {
    let mut score = 0;
    let strategy_lines = strategy_string.lines();

    for line in strategy_lines {
        let mut moves = line.split_whitespace();
        let opponent_move = Choice::from_str(moves.next().unwrap()).unwrap();
        let expected_outcome = Outcome::from_str(moves.next().unwrap()).unwrap();

        let response_move = match opponent_move {
            Rock => match expected_outcome {
                Lose => Scissors,
                Draw => Rock,
                Win => Paper,
            },
            Paper => match expected_outcome {
                Lose => Rock,
                Draw => Paper,
                Win => Scissors,
            },
            Scissors => match expected_outcome {
                Lose => Paper,
                Draw => Scissors,
                Win => Rock,
            },
        };

        score += compute_match_score(&opponent_move, &response_move);
    }

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(compute_strategy_score_from_moves(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(compute_strategy_score_from_move_and_outcome(input))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
