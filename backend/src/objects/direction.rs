use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[derive(TS)]
#[ts(export)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}