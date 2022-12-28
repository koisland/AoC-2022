use std::error::Error;
use std::fs;

use itertools::Itertools;

pub fn template(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;

    Ok(0)
}