use std::fs;
use std::error::Error;
use std::cmp::Ordering::{Equal, Less, Greater};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3
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
            _ => Err(ParseRPSError)
        }
    }


}

impl std::cmp::PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
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
            Move::Rock => {
                match other {
                    Move::Rock => Some(Equal),
                    Move::Paper => Some(Less),
                    Move::Scissor => Some(Greater),
                }
            },
            Move::Paper => {
                match other {
                    Move::Rock => Some(Greater),
                    Move::Paper => Some(Equal),
                    Move::Scissor => Some(Less),
                }
            },
            Move::Scissor => {
                match other {
                    Move::Rock => Some(Less),
                    Move::Paper => Some(Greater),
                    Move::Scissor => Some(Equal),
                }
            },
        }
    }
}


pub fn rps(fname: &str) -> Result<usize, Box<dyn Error>> {
    let prompt = fs::read_to_string(fname)?;

    // Init score.
    let mut your_score: usize =  0;

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