use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Direction, Vector2};
#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub positions: Vec<Vector2>,
    pub direction: Direction,
    pub state: PlayerState,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    Waiting,
    Dead,
    Running
}

impl Player {
    pub fn new(id: i32) -> Self {
        Self {
            direction: Direction::Up,
            id,
            positions: Vec::new(),
            username: String::new(),
            state: PlayerState::Waiting
        }
    }
    pub fn update(&mut self, size: &Vector2) {
        if self.state != PlayerState::Running {
            return
        }
        let dir = match self.direction {
            Direction::Up => Vector2::new(0, -1),
            Direction::Down => Vector2::new(0, 1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        };
        let new_pos = self.positions.last().unwrap().clone() + dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= size.x || new_pos.y >= size.y {
            self.state = PlayerState::Dead;
        } else {
            self.positions.push(new_pos);
            self.positions.remove(0);
        }
    }
    pub fn increase(&mut self) {
        let pos = self.positions.iter().last().unwrap().clone();
        self.positions.insert(0, pos);
    }
    pub fn intersect(&self, apple: &Vector2) -> bool {
        self.positions.iter().any(|p| p == apple)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}