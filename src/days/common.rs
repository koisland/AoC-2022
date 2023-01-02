use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
pub struct GridString {
    pub grid: String,
    pub rows: usize,
    pub cols: usize,
}

impl GridString {
    pub fn new(grid: &str) -> Result<GridString, &'static str> {
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
    pub fn search(&self, elem: char) -> Option<(usize, usize)> {
        if let Some(idx) = self.grid.find(|char| char == elem) {
            Some((idx / self.cols, idx % self.cols))
        } else {
            None
        }
    }
    pub fn get_one(&self, row: usize, col: usize) -> Option<char> {
        self.grid.chars().nth(row * self.cols + col)
    }
    pub fn get(&self, row: Option<usize>, col: Option<usize>) -> Option<Vec<char>> {
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

pub fn alphabet() -> HashMap<char, usize> {
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
