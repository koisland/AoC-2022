use std::{
    cell::RefCell,
    error::Error,
    fs,
    rc::{Rc, Weak},
};

use itertools::Itertools;

use crate::days::{common::alphabet, common::GridString, error::ParserError};

const STARTING_POS: char = 'S';
const ENDING_POS: char = 'E';

#[derive(Debug, Clone)]
struct Hill {
    row: usize,
    col: usize,
    parent: Option<Weak<Hill>>,
    distance: usize,
    heuristic: usize,
    cost: usize,
}

impl PartialEq for Hill {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    /// Only compare based on if same row and col.
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

// https://medium.com/@nicholas.w.swift/easy-a-star-pathfinding-7e6689c7f7b2
impl Hill {
    fn new(row: usize, col: usize, parent: Option<Rc<Hill>>) -> Hill {
        let parent_node = if let Some(parent_ref) = parent {
            Some(Rc::downgrade(&parent_ref))
        } else {
            None
        };
        Hill {
            row,
            col,
            parent: parent_node,
            distance: 0,
            heuristic: 0,
            cost: 0,
        }
    }
}

// #[test]
/// Here we'll implement the A* algorithm to calculate the shortest distance to our destination, E.
// fn test_hill_climb() {
pub fn hill_climb(fname: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    // let contents = fs::read_to_string("data/test_day_12_1.txt").unwrap();
    let contents = fs::read_to_string(fname)?;
    
    let mut grid = GridString::new(&contents)?;
    // let mut grid = GridString::new(&contents).unwrap();
    let alphabet = alphabet();

    let (start_pos, stop_pos) = (
        grid.search(STARTING_POS).ok_or(ParserError {
            reason: format!("No starting position {STARTING_POS}"),
        })?,
        grid.search(ENDING_POS).ok_or(ParserError {
            reason: format!("No ending position {ENDING_POS}"),
        })?,
    );
    // let (start_pos, stop_pos) = (
    //     grid.search(STARTING_POS).ok_or(ParserError {
    //         reason: format!("No starting position {STARTING_POS}"),
    //     }).unwrap(),
    //     grid.search(ENDING_POS).ok_or(ParserError {
    //         reason: format!("No ending position {ENDING_POS}"),
    //     }).unwrap(),
    // );

    // Set starting and ending position elevation.
    grid.grid = grid.grid.replace(ENDING_POS, "z").replace(STARTING_POS, "a");

    println!("{:?}\nStart: {:?}, End: {:?}", grid, start_pos, stop_pos);
    let start_node = Hill::new(start_pos.0, start_pos.1, None);
    let stop_node = Hill::new(stop_pos.0, stop_pos.1, None);

    let open_nodes: RefCell<Vec<Rc<Hill>>> = RefCell::new(vec![Rc::new(start_node)]);
    let mut closed_nodes: Vec<Rc<Hill>> = vec![];

    while !open_nodes.borrow().is_empty() {
        let mut curr_idx: usize = 0;
        let mut curr_node = open_nodes.borrow().first().expect("No first node.").clone();
        println!("Current Node: {:?}", (curr_node.row, curr_node.col));
        for (i, node) in open_nodes.borrow().iter().enumerate() {
            // If open node has lower cost that current node, use that node as the main node.
            if node.cost < curr_node.cost {
                curr_node = node.clone();
                curr_idx = i;
            }
        }
        
        // Remove open node.
        // Add current node to closed nodes.
        open_nodes.borrow_mut().remove(curr_idx);
        closed_nodes.push(curr_node.clone());

        // Reached goal.
        if *curr_node == stop_node {
            let mut path_found: Vec<(usize, usize)> = vec![];
            while let Some(parent) = &curr_node.parent {
                let parent = parent.upgrade().unwrap();
                path_found.push((parent.row, parent.col));
                curr_node = parent
            }
            // Reverse the path found from current path to last path so ordered.
            path_found.reverse();
            return Ok(path_found);
        }

        // Create child nodes
        let mut children_nodes: Vec<Hill> = vec![];
        // Set adjacencies and clamp to size of grid.
        let adjacencies = [
            (curr_node.row, (curr_node.col + 1).clamp(0, grid.cols)),
            (curr_node.row, curr_node.col.saturating_sub(1)),
            ((curr_node.row + 1).clamp(0, grid.rows), curr_node.col),
            (curr_node.row.saturating_sub(1), curr_node.col),
        ];

        // Calculate the height of the current hill
        let curr_hill_height = alphabet.get(
            &grid
            .fast_get(curr_node.row, curr_node.col)
            .unwrap()
        ).expect("Not valid letter.");
        
        // Loop through chidl nodes and add node if pass height criteria.
        // Is valid hill if only one unit of height difference.
        for adj in adjacencies {
            // Create a new Hill.
            let new_node = Hill::new(adj.0, adj.1, Some(curr_node.clone()));

            // grid.get returns vector.
            if let Some(hill_char) = grid.fast_get(adj.0, adj.1) {
                let adj_hill = alphabet.get(&hill_char).expect("Not valid letter.");
                // Calculate height difference to determine if adjacent tile is okay to move to.
                let height_diff = adj_hill.saturating_sub(*curr_hill_height);
                // Is valid hill if only one unit of height difference.
                // Or is the ending position character.
                if height_diff <= 1 {
                    children_nodes.push(new_node)
                }
            }
        }

        'child_node_loop: for child_node in children_nodes.iter_mut() {
            // If child node is in checked nodes, ignore it.
            for closed_node in closed_nodes.iter() {
                if *child_node == *closed_node.as_ref() {
                    continue 'child_node_loop;
                }
            }
            // Child node is an adjacent node so increment it's overall distance (irrespective of direction)
            child_node.distance = curr_node.distance + 1;
            // Calculate manhattan distance as heuristic from current child node to end node.
            child_node.heuristic = child_node.row.abs_diff(stop_node.row)
                + child_node.col.abs_diff(stop_node.col);
            // Calculate cost of moving to that node.
            child_node.cost = child_node.distance + child_node.heuristic;

            // Ignore child if on open_list and distance is greater than open_node.
            // * Want to minimize distance.
            for open_node in open_nodes.borrow().iter() {
                if *child_node == *open_node.as_ref() && child_node.distance > open_node.distance {
                    continue 'child_node_loop;
                }
            }
            // Add child node to open_nodes if all conditions pass.
            open_nodes.borrow_mut().push(Rc::new(child_node.clone()))
        }
    }

    Ok(vec![])
}
