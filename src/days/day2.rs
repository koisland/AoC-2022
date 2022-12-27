use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRPSError;

impl Error for ParseRPSError {}

impl std::fmt::Display for ParseRPSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse string to RPS move.")
    }
}
impl FromStr for Move {
    type Err = ParseRPSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err(ParseRPSError),
        }
    }
}

impl FromStr for Outcome {
    type Err = ParseRPSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseRPSError),
        }
    }
}

impl std::cmp::PartialOrd for Move {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Move::Rock => match other {
                Move::Rock => Some(Equal),
                Move::Paper => Some(Less),
                Move::Scissor => Some(Greater),
            },
            Move::Paper => match other {
                Move::Rock => Some(Greater),
                Move::Paper => Some(Equal),
                Move::Scissor => Some(Less),
            },
            Move::Scissor => match other {
                Move::Rock => Some(Less),
                Move::Paper => Some(Greater),
                Move::Scissor => Some(Equal),
            },
        }
    }
}

impl Move {
    pub fn get_outcome(&self, outcome: &Outcome) -> Option<Move> {
        let mut outcome_mv = None;
        for mv in [Move::Paper, Move::Rock, Move::Scissor] {
            let res = if self > &mv {
                Outcome::Win
            } else if self == &mv {
                Outcome::Draw
            } else {
                Outcome::Loss
            };
            if outcome == &res {
                outcome_mv = Some(mv);
                break;
            }
        }
        outcome_mv
    }
}

pub fn rps(fname: &str) -> Result<usize, Box<dyn Error>> {
    let prompt = fs::read_to_string(fname)?;

    // Init score.
    let mut your_score: usize = 0;

    for res in prompt.trim().split("\n") {
        let game_instr = res.split(" ").collect_vec();
        // R - 1, P - 2, S - 3
        // W - 6, Draw - 3, L - 0
        if let (Some(exp), Some(resp)) = (game_instr.get(0), game_instr.get(1)) {
            let opp_move = Move::from_str(exp)?;
            let your_move = Move::from_str(resp)?;

            if your_move > opp_move {
                your_score += 6
            } else if your_move == opp_move {
                your_score += 3
            }
            // Add score of move
            your_score += your_move as usize;
        }
    }
    Ok(your_score)
}

pub fn rps_2(fname: &str) -> Result<usize, Box<dyn Error>> {
    let prompt = fs::read_to_string(fname)?;

    // Init score.
    let mut your_score: usize = 0;

    for res in prompt.trim().split("\n") {
        let game_instr = res.split(" ").collect_vec();
        // A - 1, B - 2, C - 3
        // X - 6, Y - 3, Z - 0
        if let (Some(exp), Some(resp)) = (game_instr.get(0), game_instr.get(1)) {
            let opp_move = Move::from_str(exp)?;
            let opp_outcome = Outcome::from_str(resp)?;
            let your_outcome = match &opp_outcome {
                Outcome::Win => Outcome::Loss,
                Outcome::Draw => Outcome::Draw,
                Outcome::Loss => Outcome::Win,
            };
            let des_move = opp_move
                .get_outcome(&your_outcome)
                .expect("No move to give desired outcome.");
            let score_move = des_move as usize;
            let score_outcome = opp_outcome as usize;
            println!("{exp} - {resp}");
            println!(
                "{:?}, {:?}, {:?}, {:?}",
                opp_move, des_move, score_move, score_outcome
            );

            // Add score of move and outcome
            your_score += score_move + score_outcome;
        }
    }
    Ok(your_score)
}
