use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Direction, Event, Vector2};
#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: i32,
    username: String,
    positions: Vec<Vector2>,
    direction: Direction,
    state: PlayerState,

    #[ts(skip)]
    diffs: Vec<Event>
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    Waiting(i32),
    Connecting,
    Dead(i32),
    Running
}

impl Player {
    pub fn new(id: i32) -> Self {
        Self {
            direction: Direction::Up,
            id,
            positions: Vec::new(),
            username: String::new(),
            state: PlayerState::Connecting,
            diffs: Vec::new()
        }
    }
    fn set_state(&mut self, state: PlayerState) {
        self.diffs.push(Event::ChangeStatePlayer { state: state.clone(), id: self.id });
        self.state = state
    }
    pub fn update(&mut self, size: &Vector2) {
        match self.state {
            PlayerState::Waiting(1) => {
                self.set_state(PlayerState::Running)
            }
            PlayerState::Waiting(n) => {
                self.set_state(PlayerState::Waiting(n-1))
            },
            PlayerState::Running => {
                let dir = match self.direction {
                    Direction::Up => Vector2::new(0, -1),
                    Direction::Down => Vector2::new(0, 1),
                    Direction::Left => Vector2::new(-1, 0),
                    Direction::Right => Vector2::new(1, 0),
                };
                let new_pos = self.head() + dir;
                if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= size.x || new_pos.y >= size.y {
                    self.kill();
                } else {
                    self.positions.push(new_pos);
                    self.positions.remove(0);
                };
            },
            PlayerState::Dead(0) => {},
            PlayerState::Dead(n) => {
                self.set_state(PlayerState::Dead(n-1))
            }
            PlayerState::Connecting => {},
        }
    }
    pub fn increase(&mut self) {
        let pos = self.positions.iter().last().unwrap().clone();
        self.positions.insert(0, pos);
        self.diffs.push(Event::IncreasePlayer(self.id))
    }
    pub fn intersect(&self, pos: &Vector2) -> bool {
        self.positions.iter().any(|p| p == pos)
    }
    pub fn intersect_apple(&self, apple: &Vector2) -> bool {
        &self.head() == apple
    }
    fn head(&self) -> Vector2 {
        self.positions.last().unwrap().clone()
    }
    pub fn intersect_player(&self, other: &Player) -> bool {
        if other == self {
            other.positions[0..other.positions.len()-1].contains(self.positions.last().unwrap())
        } else {
            other.intersect(&self.head())
        }
    }
    pub fn kill(&mut self) {
        if self.state == PlayerState::Running {
            self.set_state(PlayerState::Dead(12));
        }
    }
    pub fn set_username(&mut self, username: String) {
        if let PlayerState::Connecting = self.state {
            self.set_state(PlayerState::Waiting(12));
            self.username = username.clone();
            self.diffs.push(Event::SetUsername { id: self.id, name: username })
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.reverse() != direction {
            self.diffs.push(Event::MovePlayer { dir: direction.clone(), id: self.id });
            self.direction = direction
        }
    }
    pub fn add_position(&mut self, position: Vector2) {
        self.positions.insert(0, position)
    }
    pub fn state(&self) -> &PlayerState {
        &self.state
    }
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn diff(&mut self) -> Vec<Event> {
        let list = self.diffs.to_owned();
        self.diffs = Vec::new();
        return list
    }


}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}