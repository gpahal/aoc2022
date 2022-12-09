use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::utils::must_read_file;

pub fn part1() {
    let data = must_read_file("data/day9/part1.txt");
    let lines = data.trim().split("\n");
    let mut state = State::new(2);
    lines.for_each(|line| {
        let movement_option = Movement::parse(line);
        if let Some(movement) = movement_option {
            state.update(&movement);
        }
    });

    println!(
        "Visited count: {}",
        state.visited.iter().fold(0, |acc, (_, v)| acc + v.len())
    );
}

pub fn part2() {
    let data = must_read_file("data/day9/part2.txt");
    let lines = data.trim().split("\n");
    let mut state = State::new(10);
    lines.for_each(|line| {
        let movement_option = Movement::parse(line);
        if let Some(movement) = movement_option {
            state.update(&movement);
        }
    });

    println!(
        "Visited count: {}",
        state.visited.iter().fold(0, |acc, (_, v)| acc + v.len())
    );
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Movement {
    direction: Direction,
    len: i32,
}

impl Movement {
    fn parse(line: &str) -> Option<Movement> {
        let line = line.trim();
        let parts = line.split(" ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return None;
        }

        let direction = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return None,
        };

        let len = parts[1].parse::<i32>().ok()?;
        Some(Movement { direction, len })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    row: i32,
    col: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

struct State {
    positions: Vec<Position>,
    visited: HashMap<i32, HashSet<i32>>,
}

impl State {
    fn new(len: usize) -> State {
        let mut positions = Vec::with_capacity(len);
        for _ in 0..len {
            positions.push(Position { row: 0, col: 0 });
        }
        let mut state = State {
            positions,
            visited: HashMap::new(),
        };
        state.add_visited(len - 1, 0, 0);
        state
    }

    fn add_visited(&mut self, i: usize, row: i32, col: i32) {
        if i == self.positions.len() - 1 {
            let row_set = self.visited.entry(row).or_insert(HashSet::new());
            row_set.insert(col);
        }
    }

    fn update(&mut self, movement: &Movement) {
        for _ in 0..movement.len {
            self.update_direction(&movement.direction);
        }
    }

    fn update_direction(&mut self, direction: &Direction) {
        let (row_delta, col_delta) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        self.positions[0].row += row_delta;
        self.positions[0].col += col_delta;

        for i in 1..self.positions.len() {
            self.update_direction_single(i);
        }
    }

    fn update_direction_single(&mut self, index: usize) {
        let row_diff = self.positions[index - 1].row - self.positions[index].row;
        let col_diff = self.positions[index - 1].col - self.positions[index].col;
        let row_movement = row_diff > 1 || row_diff < -1;
        let col_movement = col_diff > 1 || col_diff < -1;

        if !row_movement && !col_movement {
            return;
        }

        if row_movement {
            self.positions[index].row += if row_diff > 0 { 1 } else { -1 };
            if col_diff != 0 {
                self.positions[index].col += if col_diff > 0 { 1 } else { -1 };
            }
        } else if col_movement {
            self.positions[index].col += if col_diff > 0 { 1 } else { -1 };
            if row_diff != 0 {
                self.positions[index].row += if row_diff > 0 { 1 } else { -1 };
            }
        }

        self.add_visited(index, self.positions[index].row, self.positions[index].col);
    }
}
