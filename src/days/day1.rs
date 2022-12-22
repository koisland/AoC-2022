use std::fs;
use std::error::Error;

use itertools::Itertools;

pub fn get_calories(fname: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    Ok(
        contents
        .trim()
        .split("\n\n")
        .map(|cals| 
            cals
            .split("\n")
            .map(|num| num.parse::<usize>().expect("NaN"))
            .sum::<usize>())
        .collect_vec()
    )
}

pub fn max_calories(fname: &str, top_n: usize) -> Result<usize, Box<dyn Error>>{
    let mut all_calories = get_calories(fname)?;
 
    all_calories.sort();
    let mut max_cals: usize = 0;
    for (i, cal) in all_calories.iter().rev().enumerate() {
        println!("{}", cal);
        if i == top_n {
            break;
        }
        max_cals += *cal
    }
    Ok(max_cals)
}