use std::{error::Error, fs};

use itertools::Itertools;

struct GridString {
    grid: String,
    rows: usize,
    cols: usize,
}

impl GridString {
    fn new(grid: &str) -> Result<GridString, &'static str> {
        let rows = grid.lines().collect_vec();
        // Assuming all rows are equal length.
        if let Some(n_cols) = rows.get(0).and_then(|row| Some(row.len())) {
            Ok(GridString {
                grid: rows.join(""),
                rows: rows.len(),
                cols: n_cols,
            })
        } else {
            Err("Empty grid provided.")
        }
    }
    fn get(&self, row: Option<usize>, col: Option<usize>) -> Option<Vec<char>> {
        let query = self
            .grid
            .chars()
            .enumerate()
            .filter_map(|(i, elem)| {
                if let (Some(r), Some(c)) = (row, col) {
                    ((i / self.cols) == r && (i % self.cols) == c).then_some(elem)
                } else if let Some(r) = row {
                    (i / self.cols == r).then_some(elem)
                } else if let Some(c) = col {
                    (i % self.cols == c).then_some(elem)
                } else {
                    None
                }
            })
            .collect_vec();

        // Check if query empty so as to never return empty vec.
        if !query.is_empty() {
            Some(query)
        } else {
            None
        }
    }
}

pub fn tree_top_visibility(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    // TODO: Use 2D list. No matrices.

    let forest = GridString::new(&contents)?;
    /*
          01234
          |||||
        0-30373
        1-25512
        2-65332
        3-33549
        4-35390

    */

    // Get all tree coordinates on edges.
    let mut visible_trees: usize = 0;
    let mut edge_trees: Vec<(usize, usize)> = (0..forest.cols)
        .cartesian_product([0, forest.rows - 1])
        .collect_vec();
    edge_trees.extend(
        [0, forest.rows - 1]
            .into_iter()
            .cartesian_product(0..forest.cols)
            .collect_vec(),
    );

    // Remove duplicated trees in corners from cartesian product.
    visible_trees += edge_trees.len().saturating_sub(4);

    for (i, tree) in forest.grid.chars().enumerate() {
        let coords = (i / forest.cols, i % forest.cols);
        // Ignore tree if on edge.
        if !edge_trees.contains(&coords) {
            if let (Some(tree_height), Some(trees_col), Some(trees_row)) = (
                tree.to_digit(10),
                forest.get(None, Some(coords.1)),
                forest.get(Some(coords.0), None),
            ) {
                // Convert characters of digits to values.
                let mut tree_col_heights = trees_col
                    .into_iter()
                    .map(|tree| tree.to_digit(10))
                    .collect::<Option<Vec<u32>>>()
                    .expect("Can't coerce value in col.");
                let mut tree_row_heights = trees_row
                    .into_iter()
                    .map(|tree| tree.to_digit(10))
                    .collect::<Option<Vec<u32>>>()
                    .expect("Can't coerce value in col.");

                // Remove compared tree.
                tree_col_heights.remove(coords.0);
                tree_row_heights.remove(coords.1);

                // Split tree along axis at compared tree idx.
                let l_r_trees_cols = tree_col_heights.split_at(coords.0);
                let l_r_trees_rows = tree_row_heights.split_at(coords.1);
                
                // Find max tree height within split axis.
                let l_r_trees_cols_max = [l_r_trees_cols.0.iter().max().unwrap(), l_r_trees_cols.1.iter().max().unwrap()];
                let l_r_trees_rows_max = [l_r_trees_rows.0.iter().max().unwrap(), l_r_trees_rows.1.iter().max().unwrap()];
                
                // Find any tree where along split axis, any tree is shorter than it.
                if [l_r_trees_cols_max, l_r_trees_rows_max].concat().iter().any(|tree| **tree < tree_height) {
                    // println!("{tree_height} {:?} - col {:?} row {:?}", coords, l_r_trees_cols_max, l_r_trees_rows_max);
                    visible_trees += 1; 
                }
            }
        }
    }

    Ok(visible_trees)
}

fn tree_view_dst(tree_ht: u32, trees_along_axis: &[u32]) -> usize {
    let mut vis_trees: usize = 0;
    for adj_tree in trees_along_axis.iter() {
        // See a tree.
        vis_trees += 1;
        // Exit loop if a tree larger or equal is seen.
        if *adj_tree >= tree_ht {
            break;
        }
    }
    vis_trees
}

pub fn tree_scenic_scores(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    // TODO: Use 2D list. No matrices.

    let forest = GridString::new(&contents)?;
    let mut scenery_scores: Vec<usize> = vec![];

    // TODO: Lazy and didn't remove edge trees. Would improve time.
    for (i, tree) in forest.grid.chars().enumerate() {
        let coords = (i / forest.cols, i % forest.cols);
        if let (Some(tree_height), Some(trees_col), Some(trees_row)) = (
            tree.to_digit(10),
            forest.get(None, Some(coords.1)),
            forest.get(Some(coords.0), None),
        ) {
            // Convert characters of digits to values.
            let mut tree_col_heights = trees_col
                .into_iter()
                .map(|tree| tree.to_digit(10))
                .collect::<Option<Vec<u32>>>()
                .expect("Can't coerce value in col.");
            let mut tree_row_heights = trees_row
                .into_iter()
                .map(|tree| tree.to_digit(10))
                .collect::<Option<Vec<u32>>>()
                .expect("Can't coerce value in col.");

            // Remove compared tree.
            tree_col_heights.remove(coords.0);
            tree_row_heights.remove(coords.1);

            // Split tree along axis at compared tree idx.
            let (l_tree_cols, r_tree_cols) = tree_col_heights.split_at(coords.0);
            let (l_tree_rows, r_tree_rows) = tree_row_heights.split_at(coords.1);
            
            // TODO: Unnecessary clone. Can do calculate and assign to 4 vars.
            // Reverse direction and order trees come in respective to compared tree.
            let u_trees_cols = l_tree_cols.iter().rev().cloned().collect_vec();
            let l_tree_rows = l_tree_rows.iter().rev().cloned().collect_vec();

            let tree_views = [&u_trees_cols, r_tree_cols, &l_tree_rows, r_tree_rows]
                .iter()
                .map(|adj_trees| 
                    tree_view_dst(tree_height, &adj_trees)
                )
                .collect_vec();

            // println!("{tree_height} {:?} -> {:?}", coords, tree_views);
            scenery_scores.push(tree_views.iter().product())
        }
    }
    let max_scenery_score = scenery_scores.iter().max().unwrap();
    Ok(*max_scenery_score)
}