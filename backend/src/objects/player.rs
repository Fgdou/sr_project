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
            positions: vec!(Vector2::zero()),
            username: String::new(),
            state: PlayerState::Waiting
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}