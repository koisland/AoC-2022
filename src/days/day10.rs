use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use crate::days::error::ParserError;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    NoOp,
    Add(isize)
}

impl Operation {
    fn duration(&self) -> usize {
        match self {
            Operation::NoOp => 1,
            Operation::Add(_) => 2,
        }
    }
}

impl FromStr for Operation {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_instructions = s.trim().splitn(2, " ");
        // Try to split into a tuple of str slices.
        match split_instructions.collect_tuple::<(&str, &str)>() {
            Some((op, amt)) => {
                if let Ok(reg_amt) = amt.parse::<isize>() {
                    match op {
                        "addx" => Ok(Operation::Add(reg_amt)),
                        _ => Err(ParserError { reason: format!("Not implemented. {op}")})
                    }
                } else {
                    Err(ParserError { reason: format!("Unable to convert register amount for {op} ({s}).")})
                }
            },
            None => {
                match s {
                    "noop" => Ok(Operation::NoOp),
                    _ => Err(ParserError { reason: format!("Invalid single-word operation. {s}")})
                }
            }
        }
    }
}

struct CRT {
    screen: String,
    screen_width: usize,
    screen_height: usize,
    sprite_width: usize,
    lit_pixel: char,
    empty_pixel: char
}


#[derive(Debug)]
struct Instruction {
    operation: Operation,
    stop_cycle: usize
}

#[derive(Debug)]
struct SimpleCPU {
    register: isize,
    cycles: usize,
    instructions: VecDeque<Instruction>,
    register_history: Vec<(usize, isize)>
}

impl SimpleCPU {
    fn new() -> Self {
        SimpleCPU {
            register: 1,
            cycles: 1,
            instructions: VecDeque::new(),
            register_history: Vec::new()
        }
    }

    fn complete_command(&mut self) {
        let mut finished_instruction: Vec<usize> = vec![];
        for (i, instruction) in self.instructions.iter_mut().enumerate().rev() {
            // Remove instruction if stop cycle reached.
            if instruction.stop_cycle == self.cycles {
                match instruction.operation {
                    Operation::NoOp => {
                        
                    },
                    Operation::Add(amt) => {
                        self.register += amt
                    },
                };
                finished_instruction.push(i)
            }
        }
        // Remove finished instructions.
        for i_instruction in finished_instruction.into_iter() {
            self.instructions.remove(i_instruction);
        }
    }

    fn run_command(&mut self, cmd_str: &str, screen: Option<&mut str>) -> Result<(), ParserError> {
        let parsed_cmd = Operation::from_str(cmd_str);
        if let Ok(parsed_cmd) = parsed_cmd {
            self.instructions.push_back(
                Instruction {
                    operation: parsed_cmd,
                    stop_cycle: self.cycles + parsed_cmd.duration() }
            );
            // Increment cycles, store register value at timepoint, and then run command.
            for _ in 0..parsed_cmd.duration() {
                self.cycles += 1;
                self.complete_command();
                self.register_history.push((self.cycles, self.signal_strength()));
            }

            Ok(())
        } else {
            Err(parsed_cmd.unwrap_err())
        }
    }

    fn run_program(&mut self, fname: &str, screen: &mut String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(fname)?;
        for instruction in contents.lines() {
            self.run_command(instruction, Some(screen))?;
        }

        Ok(())
    }

    fn signal_strength(&self) -> isize {
        self.register * self.cycles as isize
    }
}
 
pub fn cathode_cpu(fname: &str) -> Result<isize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let mut cpu = SimpleCPU::new();

    let mut cycle_checkpoints: VecDeque<usize> = (60..500).step_by(40).collect();
    cycle_checkpoints.push_front(20);

    for instruction in contents.lines() {
        cpu.run_command(instruction, None)?;
    }
    // Print out timepoints in register history.
    let agg_signal: isize = cpu.register_history
        .iter()
        .filter_map(|(cycle, signal)| 
            if cycle_checkpoints.contains(cycle) {
                println!("{},{}", cycle, signal);
                Some(signal)
            } else {
                None
            }
        )
        .sum();

    Ok(agg_signal)
}

pub fn race_the_beam(fname: &str) -> Result<(), Box<dyn Error>> {
    let mut screen = (0..240).map(|_| '.').collect::<String>();
    let mut cpu = SimpleCPU::new();

    cpu.run_program(fname, &mut screen)?;

    Ok(())
}