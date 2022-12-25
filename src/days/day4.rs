use std::{fs, error::Error, ops::RangeInclusive, collections::HashSet};

use itertools::Itertools;

trait ToRange {
    fn to_range(&self) -> Result<RangeInclusive<usize>, &'static str>;
}

impl ToRange for Vec<usize> {
    /// Convert vector of `usize` to inclusive range of values.
    fn to_range(&self) -> Result<RangeInclusive<usize>, &'static str> {
        let sorted_idxs = self.into_iter().sorted().collect_vec();
        if let (Some(start), Some(stop)) = (sorted_idxs.get(0), sorted_idxs.get(1)) {
            Ok(**start..=**stop)
        } else {
            Err("Provided slice doesn't contain a start and stop.")
        }
    }
}

/// Convert string of format `#-#` to a full set of `#`'s.
fn range_to_hashset(rng_str: &str) -> Result<HashSet<usize>, Box<dyn Error>> {
    Ok(
        rng_str
        .trim()
        .split("-")
        .map(|val| val.parse::<usize>().unwrap())
        .collect_vec()
        .to_range()?
        .collect()
    )
}

pub fn camp_cleanup_duplicates(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let pairs = contents
        .trim()
        .split("\n")
        .map(|assignments| assignments.split(",").collect_vec())
        .collect_vec();
    // Number of pairs where one elf's assignments are a subset of another.
    let mut n_full_cont_pairs = 0;

    for pair in pairs.iter() {
        if let (Some(a_1), Some(a_2)) = (pair.get(0), pair.get(1)) {
            // Convert ranges to hashset.
            let a_rng_1: HashSet<usize> = range_to_hashset(&a_1)?;
            let a_rng_2: HashSet<usize> = range_to_hashset(&a_2)?;

            // Check if one belongs in the other.
            if a_rng_1.is_subset(&a_rng_2) || a_rng_2.is_subset(&a_rng_1) {
                n_full_cont_pairs += 1
            }
        }
    }
    Ok(n_full_cont_pairs)
}

pub fn camp_cleanup_overlap(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let pairs = contents
        .trim()
        .split("\n")
        .map(|assignments| assignments.split(",").collect_vec())
        .collect_vec();
    // Number of pairs where one elf's assignments are a subset of another.
    let mut n_full_cont_pairs = 0;

    for pair in pairs.iter() {
        if let (Some(a_1), Some(a_2)) = (pair.get(0), pair.get(1)) {
            // Convert ranges to hashset.
            let a_rng_1: HashSet<usize> = range_to_hashset(&a_1)?;
            let a_rng_2: HashSet<usize> = range_to_hashset(&a_2)?;

            // Check if one belongs in the other.
            let overlap_1_2 = a_rng_1.intersection(&a_rng_2).collect_vec();
            if !overlap_1_2.is_empty() {
                n_full_cont_pairs += 1
            }
        }
    }
    Ok(n_full_cont_pairs)
}