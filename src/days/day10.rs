use crate::days::error::ParserError;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    NoOp,
    Add(isize),
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
                        _ => Err(ParserError {
                            reason: format!("Not implemented. {op}"),
                        }),
                    }
                } else {
                    Err(ParserError {
                        reason: format!("Unable to convert register amount for {op} ({s})."),
                    })
                }
            }
            None => match s {
                "noop" => Ok(Operation::NoOp),
                _ => Err(ParserError {
                    reason: format!("Invalid single-word operation. {s}"),
                }),
            },
        }
    }
}

#[derive(Debug)]
struct CRT {
    screen: String,
    screen_width: usize,
    screen_height: usize,
    sprite_pos: Vec<usize>,
    lit_pixel: char,
    empty_pixel: char,
}

impl CRT {
    fn new() -> Self {
        let screen_width: usize = 40;
        let screen_height: usize = 6;
        let empty_pixel: char = '.';

        CRT {
            screen: (0..screen_width * screen_height)
                .map(|_| empty_pixel)
                .collect::<String>(),
            screen_width,
            screen_height,
            sprite_pos: vec![0, 1, 2],
            lit_pixel: '#',
            empty_pixel,
        }
    }

    /// Each cycle, the CRT draws a single pixel.
    fn update_screen(&mut self, cycle: usize) {
        let cycle_pos = cycle % self.screen_width;

        // If register is at the sprite position, light pixel.
        if self.sprite_pos.contains(&cycle_pos) {
            self.screen
                .replace_range(cycle..cycle, &self.lit_pixel.to_string())
        }
    }
    fn update_sprite_pos(&mut self, register: usize) {
        // Update sprite pos based on memory register.
        self.sprite_pos = (register..=register + 2).collect_vec();
    }

    fn display(&self) {
        // Add extra 40 px width for tuple windows and additional step.
        let screen = (0..(self.screen_width * self.screen_height) + self.screen_width)
            .step_by(self.screen_width)
            .tuple_windows::<(usize, usize)>()
            .map(|(start, stop)| self.screen.get(start..stop).unwrap())
            .join("\n");
        println!("{screen}");
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    stop_cycle: usize,
}

#[derive(Debug)]
struct SimpleCPU {
    register: isize,
    cycles: usize,
    instructions: VecDeque<Instruction>,
    register_history: Vec<(usize, isize)>,
}

impl SimpleCPU {
    fn new() -> Self {
        SimpleCPU {
            register: 0,
            cycles: 0,
            instructions: VecDeque::new(),
            register_history: Vec::new(),
        }
    }

    fn complete_command(&mut self) {
        let mut finished_instruction: Vec<usize> = vec![];
        for (i, instruction) in self.instructions.iter_mut().enumerate().rev() {
            // Remove instruction if stop cycle reached.
            if instruction.stop_cycle == self.cycles {
                match instruction.operation {
                    Operation::NoOp => {}
                    Operation::Add(amt) => self.register += amt,
                };
                finished_instruction.push(i)
            }
        }
        // Remove finished instructions.
        for i_instruction in finished_instruction.into_iter() {
            self.instructions.remove(i_instruction);
        }
    }

    fn run_command(
        &mut self,
        cmd_str: &str,
        screen: Option<Rc<RefCell<CRT>>>,
    ) -> Result<(), ParserError> {
        let parsed_cmd = Operation::from_str(cmd_str);
        if let Ok(parsed_cmd) = parsed_cmd {
            // Init command.
            self.instructions.push_back(Instruction {
                operation: parsed_cmd,
                stop_cycle: self.cycles + parsed_cmd.duration(),
            });

            // Increment cycles, store register value at timepoint, and then run command.
            for _ in 0..parsed_cmd.duration() {
                // Draw next pixel and update register position.
                if let Some(screen) = screen.as_ref() {
                    screen.borrow_mut().update_screen(self.cycles);
                    screen.borrow().display();
                }

                self.cycles += 1;

                self.complete_command();

                // Update sprite_pos after completing command.
                if let Some(screen) = screen.as_ref() {
                    screen
                        .borrow_mut()
                        .update_sprite_pos(self.register.clamp(0, 100_000_000) as usize);
                }

                self.register_history
                    .push((self.cycles, self.signal_strength()));
            }

            Ok(())
        } else {
            Err(parsed_cmd.unwrap_err())
        }
    }

    fn run_program(&mut self, fname: &str, screen: Rc<RefCell<CRT>>) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(fname)?;
        for instruction in contents.lines() {
            self.run_command(instruction, Some(screen.clone()))?;
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
    let agg_signal: isize = cpu
        .register_history
        .iter()
        .filter_map(|(cycle, signal)| {
            if cycle_checkpoints.contains(cycle) {
                println!("{},{}", cycle, signal);
                Some(signal)
            } else {
                None
            }
        })
        .sum();

    Ok(agg_signal)
}

pub fn race_the_beam(fname: &str) -> Result<(), Box<dyn Error>> {
    let screen = Rc::new(RefCell::new(CRT::new()));
    let mut cpu = SimpleCPU::new();

    cpu.run_program(fname, screen.clone())?;

    Ok(())
}
