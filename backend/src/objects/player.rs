use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Direction, Vector2};
#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: i32,
    username: String,
    positions: Vec<Vector2>,
    direction: Direction,
    state: PlayerState,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    Waiting(i32),
    Connecting,
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
            state: PlayerState::Connecting
        }
    }
    pub fn update(&mut self, size: &Vector2) {
        match self.state {
            PlayerState::Waiting(n) => {
                self.state = if n > 1 {
                    PlayerState::Waiting(n-1)
                } else {
                    PlayerState::Running
                }
            },
            PlayerState::Running => {
                let dir = match self.direction {
                    Direction::Up => Vector2::new(0, -1),
                    Direction::Down => Vector2::new(0, 1),
                    Direction::Left => Vector2::new(-1, 0),
                    Direction::Right => Vector2::new(1, 0),
                };
                let new_pos = self.positions.last().unwrap().clone() + dir;
                if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= size.x || new_pos.y >= size.y {
                    self.kill();
                } else {
                    self.positions.push(new_pos);
                    self.positions.remove(0);
                };
            },
            _ => ()
        }
    }
    pub fn increase(&mut self) {
        let pos = self.positions.iter().last().unwrap().clone();
        self.positions.insert(0, pos);
    }
    pub fn intersect_apple(&self, apple: &Vector2) -> bool {
        self.positions.iter().any(|p| p == apple)
    }
    pub fn intersect_player(&self, other: &Player) -> bool {
        if other == self {
            other.positions[0..other.positions.len()-1].contains(self.positions.last().unwrap())
        } else {
            other.positions.contains(self.positions.last().unwrap())
        }
    }
    pub fn kill(&mut self) {
        if self.state == PlayerState::Running {
            self.state = PlayerState::Dead
        }
    }
    pub fn set_username(&mut self, username: String) {
        if let PlayerState::Connecting = self.state {
            self.state = PlayerState::Waiting(12);
        }
        self.username = username;
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_positions(&self) -> &Vec<Vector2> {
        &self.positions
    }
    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.reverse() != direction {
            self.direction = direction
        }
    }
    pub fn add_position(&mut self, position: Vector2) {
        self.positions.push(position)
    }
    pub fn get_state(&self) -> &PlayerState {
        &self.state
    }
    pub fn get_username(&self) -> &String {
        &self.username
    }


}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}