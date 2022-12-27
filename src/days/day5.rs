use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

lazy_static! {
    static ref RGX_INSTRUCTIONS: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn parse_stack_text(stack_str: &str) -> Result<Vec<VecDeque<char>>, Box<dyn Error>> {
    let mut lines = stack_str.lines().collect_vec();
    let mut stacks: Vec<VecDeque<char>> = vec![];

    let stack_str_idx = lines
        .last()
        .map(|stack_nums| {
            stack_nums
                .trim_end()
                .chars()
                .enumerate()
                .filter_map(|(i, char)| if char != ' ' { Some(i) } else { None })
                .collect_vec()
        })
        .expect("No stack numbers found.");

    lines.pop();

    // Get max number of stacks and generate a double-ended queue for each stack.
    for _ in 0..stack_str_idx.len() {
        let stack: VecDeque<char> = VecDeque::new();
        stacks.push(stack)
    }

    // Parse from top to bottom so must reverse to ensure order is correct.
    for line in lines.into_iter().rev() {
        for (stack_idx, line_idx) in stack_str_idx.iter().enumerate() {
            if let Some(item) = line.chars().nth(*line_idx).filter(|char| *char != ' ') {
                if let Some(stack) = stacks.get_mut(stack_idx) {
                    stack.push_back(item)
                } else {
                    println!("Stack {stack_idx} not found.")
                }
            }
        }
    }

    Ok(stacks)
}

#[derive(Debug)]
struct StackParseError;

impl std::fmt::Display for StackParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse stack from given prompt.")
    }
}
impl Error for StackParseError {}

pub fn crate_mover_9000(fname: &str) -> Result<Vec<char>, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let stack_instructions = contents.split("\n\r\n").collect_vec();

    // Unpack stack and the moving instructions.
    if let (Some(stack), Some(instructions)) =
        (stack_instructions.get(0), stack_instructions.get(1))
    {
        // Get stacks from text.
        let mut stacks = parse_stack_text(&stack)?;

        // Iterate through instructions.
        for line in instructions.lines() {
            // Match on pattern in instructions.
            if let Some(cap) = RGX_INSTRUCTIONS.captures(line) {
                if let (Some(n_crates_mtch), Some(from_stack_mtch), Some(to_stack_mtch)) =
                    (cap.get(1), cap.get(2), cap.get(3))
                {
                    let n_crates: usize = n_crates_mtch.as_str().trim().parse()?;
                    let from_stack_idx: usize = from_stack_mtch.as_str().parse()?;
                    let to_stack_idx: usize = to_stack_mtch.as_str().parse()?;

                    // Move n crates from a stack
                    for _ in 0..n_crates {
                        let mut item: Option<char> = None;
                        if let Some(stack_from) = stacks.get_mut(from_stack_idx - 1) {
                            item = stack_from.pop_back();
                        };
                        if let (Some(stack_to), Some(moved_item)) =
                            (stacks.get_mut(to_stack_idx - 1), item)
                        {
                            stack_to.push_back(moved_item)
                        }
                    }
                    // println!("Moved {} from {} to {}", n_crates, from_stack_idx, to_stack_idx);
                }
            }
        }

        // println!("{:#?}", stacks);
        let last_items = stacks
            .iter_mut()
            .map(|stack| stack.pop_back().unwrap_or(' '))
            .collect_vec();
        Ok(last_items)
    } else {
        Err(Box::new(StackParseError))
    }
}

pub fn crate_mover_9001(fname: &str) -> Result<Vec<char>, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let stack_instructions = contents.split("\n\r\n").collect_vec();

    // Unpack stack and the moving instructions.
    if let (Some(stack), Some(instructions)) =
        (stack_instructions.get(0), stack_instructions.get(1))
    {
        // Get stacks from text.
        let mut stacks = parse_stack_text(&stack)?;

        // Iterate through instructions.
        for line in instructions.lines() {
            // Match on pattern in instructions.
            if let Some(cap) = RGX_INSTRUCTIONS.captures(line) {
                if let (Some(n_crates_mtch), Some(from_stack_mtch), Some(to_stack_mtch)) =
                    (cap.get(1), cap.get(2), cap.get(3))
                {
                    let n_crates: usize = n_crates_mtch.as_str().trim().parse()?;
                    let from_stack_idx: usize = from_stack_mtch.as_str().parse()?;
                    let to_stack_idx: usize = to_stack_mtch.as_str().parse()?;

                    // Move n crates from a stack
                    // Use intermediate vector to hold moved items.
                    let mut crate_holder: Vec<char> = vec![];
                    for _ in 0..n_crates {
                        if let Some(stack_from) = stacks.get_mut(from_stack_idx - 1) {
                            if let Some(item) = stack_from.pop_back() {
                                crate_holder.push(item);
                            }
                        };
                    }
                    if let Some(stack_to) = stacks.get_mut(to_stack_idx - 1) {
                        // From intermediate crate holder, reverse order to ensure moving multiple crates retain order.
                        for held_item in crate_holder.into_iter().rev() {
                            stack_to.push_back(held_item)
                        }
                    }
                    // println!("Moved {} from {} to {}", n_crates, from_stack_idx, to_stack_idx);
                }
            }
        }

        // println!("{:#?}", stacks);
        let last_items = stacks
            .iter_mut()
            .map(|stack| stack.pop_back().unwrap_or(' '))
            .collect_vec();
        Ok(last_items)
    } else {
        Err(Box::new(StackParseError))
    }
}
