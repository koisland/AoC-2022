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
    
    fn successors<F: Fn(usize, usize) -> bool>(&self, grid: &GridString, condition: F, alpha_map: &HashMap<char, usize>)-> Vec<(Pos, u32)> {
        let curr_height = Pos::map_coord_to_height(self.row, self.col, grid, alpha_map);
        // println!("({},{})", self.row, self.col);
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
            (condition(adj_height, curr_height)).then(|| (Pos {row: *row, col: *col}, 1))
        })
        .collect()
    }
}

pub fn hill_climb(fname: &str) -> Result<usize, Box<dyn Error>> {
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

    let (path, n_steps) = astar(
        &start_node,
        |p| p.successors(
            &grid,
            |adj, curr| adj <= curr + 1,
            &alphabet),
        |p| p.distance(&stop_node),
        |p| *p == stop_node
    ).ok_or(ParserError {
        reason: format!("No path found."),
    })?;

    Ok(n_steps as usize)
}

pub fn hill_climb_any_start(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    
    let mut grid = GridString::new(&contents)?;
    let alphabet = alphabet();

    // Set starting and ending position elevation.
    grid.grid = grid.grid.replace(ENDING_POS, "z").replace(STARTING_POS, "a");
    
    let (all_start_pos, stop_pos) = (
        grid.search_all('a').ok_or(ParserError {
            reason: format!("No starting position 'a'"),
        })?,
        grid.search('z').ok_or(ParserError {
            reason: format!("No ending position 'z'"),
        })?,
    );

    let mut all_paths_n_steps = vec![];
    for start_pos in all_start_pos {
        println!("Start: {:?}, End: {:?}", start_pos, stop_pos);
        let start_node = Pos { row: start_pos.0, col: start_pos.1 };
        let stop_node = Pos { row: stop_pos.0, col: stop_pos.1 };
    
        if let Some((_, n_steps)) = astar(
            &start_node,
            |p| p.successors(
                &grid,
                |adj, curr| adj <= curr + 1,
                &alphabet),
            |p| p.distance(&stop_node),
            |p| *p == stop_node
        ) {
            all_paths_n_steps.push(n_steps)
        } else {
            continue;
        }
    };

    println!("{:?}", all_paths_n_steps);
    if let Some(least_n_steps) = all_paths_n_steps.iter().min() {
        Ok(*least_n_steps as usize)
    } else {
        Err(Box::new(ParserError { reason: "No paths found.".to_string()}))
    }
    
}

#[test]
fn test_day12_1() {
    let input = "data/test_day_12_1.txt";
    let pathfinder = hill_climb(input);
    if let Ok(n_steps) = pathfinder {
        assert_eq!(n_steps, 31)
    } else {
        panic!("{}", pathfinder.unwrap_err())
    }
}

#[test]
fn test_day12_2() {
    let input = "data/test_day_12_1.txt";
    let pathfinder = hill_climb_any_start(input);
    if let Ok(n_steps) = pathfinder {
        assert_eq!(n_steps, 29)
    } else {
        panic!("{}", pathfinder.unwrap_err())
    }
}