use std::{
    error::Error,
    fs, collections::HashMap,
};


use pathfinding::prelude::astar;
use itertools::Itertools;

use crate::days::{common::alphabet, common::GridString, error::ParserError};

const STARTING_POS: char = 'S';
const ENDING_POS: char = 'E';

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    row: usize,
    col: usize,
}


// https://medium.com/@nicholas.w.swift/easy-a-star-pathfinding-7e6689c7f7b2
impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.row.abs_diff(other.row) + self.col.abs_diff(other.col)) as u32
    }

    fn map_coord_to_height(row: usize, col: usize, grid: &GridString, alpha_map: &HashMap<char, usize>) -> usize {
        let curr_height_char = grid
            .get_one(row, col)
            .expect("No character at coord.");
        *alpha_map.get(&curr_height_char).expect("Character not in alphabet.")
    }
    
    fn successors(&self, grid: &GridString, alpha_map: &HashMap<char, usize>) -> Vec<(Pos, u32)> {
        let curr_height = Pos::map_coord_to_height(self.row, self.col, grid, alpha_map);
        println!("({},{})", self.row, self.col);
        // Check bounds of adjaceny using Grid.
        let adjs = [
            (
                self.row,
                (self.col + 1).clamp(0, grid.cols.saturating_sub(1))
            ),
            (
                self.row,
                self.col.saturating_sub(1)
            ),
            (
                (self.row + 1).clamp(0, grid.rows.saturating_sub(1)), 
                self.col
            ),
            (
                self.row.saturating_sub(1),
                self.col
            ),
        ];
        
        // Go through adjacencies and calculate the height of the position.
        // Add as successor only if the absolute difference between the adj pos and curr pos heights is less than or equal to 1.
        adjs
        .iter()
        .filter_map(|(row, col)| {
            let adj_height = Pos::map_coord_to_height(*row, *col, grid, alpha_map);
            (curr_height <= adj_height + 1).then(|| (Pos {row: *row, col: *col}, 1))
        })
        .collect()
    }
}

pub fn hill_climb(fname: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    
    let mut grid = GridString::new(&contents)?;
    let alphabet = alphabet();

    let (start_pos, stop_pos) = (
        grid.search(STARTING_POS).ok_or(ParserError {
            reason: format!("No starting position {STARTING_POS}"),
        })?,
        grid.search(ENDING_POS).ok_or(ParserError {
            reason: format!("No ending position {ENDING_POS}"),
        })?,
    );

    // Set starting and ending position elevation.
    grid.grid = grid.grid.replace(ENDING_POS, "z").replace(STARTING_POS, "a");

    println!("{:?}\nStart: {:?}, End: {:?}", grid, start_pos, stop_pos);
    let start_node = Pos { row: start_pos.0, col: start_pos.1 };
    let stop_node = Pos { row: stop_pos.0, col: stop_pos.1 };

    let result = astar(
        &start_node,
        |p| p.successors(&grid, &alphabet),
        |p| p.distance(&stop_node),
        |p| *p == stop_node
    );
    println!("{:?}", result);
    Ok(vec![])
}
