use std::{
    collections::{VecDeque, HashSet},
    error::Error,
    fs,
    ops::{Add, AddAssign, Sub, Range},
    str::FromStr,
    vec, cmp,
};
use crate::days::error::ParserError;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn is_adj(&self, other: &Position) -> bool {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        if dx <= 1 && dy <= 1 {
            true
        } else {
            false
        }
    }

    fn is_on_axis(&self, other: &Position) -> bool {
        if self.x == other.x || self.y == other.y {
            true
        } else {
            false
        }
    }

    // https://www.reddit.com/r/adventofcode/comments/zgnice/2022_day_9_solutions/izugdyl/
    fn compute_distance(self, to: Position) -> u32 {
        let delta_x: u32 = (to.x - self.x).abs() as u32;
        let delta_y: u32 = (to.y - self.y).abs() as u32;
        return cmp::max(delta_x, delta_y);
    }
    
    fn compute_tail_move(self, to: Position) -> Position {
        let mut delta_x = to.x - self.x;
        let mut delta_y= to.y - self.y;
        if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
            delta_x = delta_x.clamp(-1, 1);
            delta_y = delta_y.clamp(-1, 1);
        } else if delta_x.abs() == 2 && delta_y == 0 {
            delta_x = delta_x.clamp(-1, 1);
        } else if delta_x == 0 && delta_y.abs() == 2 {
            delta_y = delta_y.clamp(-1, 1);
        }
        Position{ x: delta_x, y: delta_y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        match value {
            Direction::U => Position { x: 0, y: 1 },
            Direction::D => Position { x: 0, y: -1 },
            Direction::L => Position { x: -1, y: 0 },
            Direction::R => Position { x: 1, y: 0 },
        }
    }
}



impl FromStr for Direction {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            _ => Err(ParserError {
                reason: "Not a valid direction.".to_string(),
            }),
        }
    }
}

#[derive(Debug)]
struct Move {
    pos: Position,
    pos_change: Position,
    segment: usize,
}

#[derive(Debug)]
struct Rope {
    segments: VecDeque<Position>,
    moves: Vec<Move>,
}

impl Rope {
    fn new(segments: usize) -> Self {
        Rope {
            segments: VecDeque::from_iter(vec![Position { x: 0, y: 0 }; segments]),
            moves: vec![],
        }
    }

    /// Move a `Rope` some `Direction` step-wise.
    fn move_rope(&mut self, direction: Direction) {
        let mut pos_changes = vec![];
        let pos_change = Position::from(direction);

        for (i, segment) in self.segments.iter().enumerate() {
            // Head of rope.
            if i == 0 {
                pos_changes.push(pos_change);
            } else {
                if let (Some(prev_segment), Some(prev_segment_change)) =
                    (self.segments.get(i - 1), pos_changes.get(i - 1))
                {
                    if segment.compute_distance(*prev_segment) > 1 {
                        let adj_pos_change = segment.compute_tail_move(*prev_segment);
                        pos_changes.push(adj_pos_change);
                    } else {
                        pos_changes.push(Position { x: 0, y: 0 })
                    }

                //     let new_prev_segment_pos = *prev_segment + *prev_segment_change;

                //     // If directly adjacent to previous segment.
                //     if segment.is_adj(&new_prev_segment_pos) {
                //         continue;
                //         // pos_changes.push(Position { x: 0, y: 0 })
                //     } else if !segment.is_adj(&new_prev_segment_pos)
                //         && !segment.is_on_axis(&new_prev_segment_pos)
                //     {
                        
                //     } else {
                //         pos_changes.push(pos_change)
                //     }
                // }
                }
            }
        }
        for (i, (curr_pos, pos_change)) in self.segments.iter_mut().zip(pos_changes).enumerate() {
            
            *curr_pos += pos_change;
            println!("{i} Moved {:?} to {:?}", pos_change, curr_pos);
            self.moves.push(Move {
                pos: *curr_pos,
                pos_change,
                segment: i,
            })
        }
    }

    /// Produce a grid showing the positions that a specific rope segment(s) visited.
    /// 
    /// **Warning**: Very memory expensive as will produce sparse grid.
    /// ```
    /// ..##..
    /// ...##.
    /// .####.
    /// ....#.
    /// _###..
    /// ```
    fn visited_positions(&self, rope_segments: Range<usize>) -> String {
        let mut grid: Vec<Vec<char>> = vec![];
        let mut cols: usize = 0;
        let mut rows: usize = 0;

        // Get bounds.
        for mv in self.moves.iter() {
            let (x, y) = (mv.pos.x as usize, mv.pos.y as usize);
            if x > cols {
                cols = x
            }
            if y > rows {
                rows = y
            }
        }
        // Fill out rows
        for _ in 0..=rows {
            grid.push(vec![])
        }
        // Fill empty spaces in grid. Fill in rope elements.
        for mv in self.moves.iter() {
            let (_, row) = (mv.pos.x as usize, mv.pos.y as usize);
            
            let spec_row = grid.get_mut(row).unwrap();
            while spec_row.len() <= cols {
                spec_row.push('.')
            }
            if !rope_segments.contains(&mv.segment) {
                continue;
            }
            // Should never panic.
            grid[mv.pos.y as usize][mv.pos.x as usize] = '#'
        }
        grid.iter()
            .rev()
            .map(|cols| cols.iter().join(""))
            .join("\n")
    }
}

pub fn rope_movement(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;

    let segment_len: usize = 10;
    let mut rope = Rope::new(segment_len);

    for instruction in contents.lines() {
        if let Some((direction, steps)) = instruction.splitn(2, " ").collect_tuple() {
            let movement_direction = Direction::from_str(direction)?;

            // Move step wise from instructions.
            for _ in 0..steps.parse::<usize>()? {
                rope.move_rope(movement_direction)
            }
        }
    }
    println!(
        "{:?}",
        rope.moves.iter().filter_map(|mv|  if mv.segment == segment_len - 1 {Some(mv.pos)} else {None})
        .collect_vec()
    );
    // Get the second element in rope (ie. the tail).
    let uniq_tail_pos: HashSet<Position> = rope.moves
        .iter()
        .filter_map(|mv|  if mv.segment == segment_len - 1 {Some(mv.pos)} else {None})
        .collect();

    // // // View grid.
    // let grid_pos = rope.visited_positions(Range { start: 1, end: 2 });
    // let grid_char_cnts = grid_pos.chars().counts();
    // let visited_tail_pos = grid_char_cnts.get(&'#').unwrap_or(&0);
    // Ok(*visited_tail_pos)

    Ok(uniq_tail_pos.len())
}
