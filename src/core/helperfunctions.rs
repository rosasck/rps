/**
 * @file helperfunctions.rs
 * @brief Helper Functions for RPS Game
 * @author Katie Stoltz
 *
 */

// Imports

use crate::core::types::{ RPSMove, RPSResult, RPSJudge, RPSMoveError };
use std::fmt;
use std::str;
use rand::Rng;
use rand::distributions::{ Distribution, Standard };

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

impl Compare<RPSMove, RPSResult> for RPSMove {
    fn compare(&self, b: &RPSMove) -> RPSResult {
        match (self, b) {
            (RPSMove::Rock, RPSMove::Rock) => RPSResult::Tie(self.to_string()),
            (RPSMove::Rock, RPSMove::Paper) => RPSResult::Loss(RPSJudge::PaperBeatsRock),
            (RPSMove::Rock, RPSMove::Scissors) => RPSResult::Win(RPSJudge::RockBeatsScissors),

            (RPSMove::Paper, RPSMove::Rock) => RPSResult::Win(RPSJudge::PaperBeatsRock),
            (RPSMove::Paper, RPSMove::Paper) => RPSResult::Tie(self.to_string()),
            (RPSMove::Paper, RPSMove::Scissors) => RPSResult::Loss(RPSJudge::ScissorsBeatsPaper),

            (RPSMove::Scissors, RPSMove::Rock) => RPSResult::Loss(RPSJudge::RockBeatsScissors),
            (RPSMove::Scissors, RPSMove::Paper) => RPSResult::Win(RPSJudge::ScissorsBeatsPaper),
            (RPSMove::Scissors, RPSMove::Scissors) => RPSResult::Tie(self.to_string()),
        }
    }
}

impl fmt::Display for RPSResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPSResult::Win(result) | RPSResult::Loss(result) => {
                match result {
                    RPSJudge::RockBeatsScissors => write!(f, "Rock beats scissors"),
                    RPSJudge::PaperBeatsRock => write!(f, "Paper beats rock"),
                    RPSJudge::ScissorsBeatsPaper => write!(f, "Scissors beats paper"),
                }
            }
            RPSResult::Tie(result) => write!(f, "{}", result),
        }
    }
}

impl str::FromStr for RPSMove {
    type Err = RPSMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r" | "rock" => Ok(RPSMove::Rock),
            "p" | "paper" => Ok(RPSMove::Paper),
            "s" | "scissors" => Ok(RPSMove::Scissors),
            _ => Err(RPSMoveError::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for RPSMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPSMove::Rock => write!(f, "Rock"),
            RPSMove::Paper => write!(f, "Paper"),
            RPSMove::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Distribution<RPSMove> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RPSMove {
        match rng.gen_range(0..3) {
            0 => RPSMove::Rock,
            1 => RPSMove::Paper,
            2 => RPSMove::Scissors,
            _ => unreachable!(),
        }
    }
}
