use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn build_priorities() -> HashMap<char, usize> {
    let low_alpha = (b'a'..=b'z')
        .filter_map(|c| {
            let c = c as char;
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, chr)| (chr, i + 1))
        .collect::<HashMap<char, usize>>();

    let mut alphabet = (b'A'..=b'Z')
        .filter_map(|c| {
            let c = c as char;
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, chr)| (chr, i + 27))
        .collect::<HashMap<char, usize>>();

    // Join two alphabets
    for (k, v) in low_alpha.into_iter() {
        alphabet.insert(k, v);
    }

    alphabet
}
pub fn rucksack(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let alphabet = build_priorities();
    let mut all_priorities: Vec<usize> = vec![];

    for mut sack in contents.trim().split("\n") {
        sack = sack.trim();
        let n_items = sack.len();
        let first_comp = &sack[0..(n_items / 2)];
        let second_comp = &sack[(n_items / 2)..];
        assert_eq!(first_comp.len(), second_comp.len());

        let first_comp_item_cnts: HashSet<char> = first_comp.chars().collect();
        let second_comp_item_cnts: HashSet<char> = second_comp.chars().collect();
        let shared_items = first_comp_item_cnts
            .intersection(&second_comp_item_cnts)
            .collect_vec();

        if let Some(shared_item) = shared_items.get(0) {
            let priority = alphabet.get(&shared_item).unwrap_or(&0);
            // println!("{:?} - {}", shared_item, priority);
            all_priorities.push(*priority)
        } else {
            println!("No shared items in sack: {sack}")
        }
    }

    Ok(all_priorities.iter().sum())
}

pub fn elf_groups(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let alphabet = build_priorities();
    let mut all_priorities: Vec<usize> = vec![];

    for group in &contents.trim().split("\n").chunks(3) {
        let shared_sack_contents = group
            .into_iter()
            .map(|sack| sack.trim().chars().collect::<HashSet<char>>())
            .reduce(|acc, cnt| acc.intersection(&cnt).cloned().collect());
        if let Some(shared_items) = shared_sack_contents {
            if let Some(first_elem) = shared_items.iter().next() {
                all_priorities.push(*alphabet.get(first_elem).unwrap_or(&0));
            }
        }
    }
    Ok(all_priorities.iter().sum())
}
