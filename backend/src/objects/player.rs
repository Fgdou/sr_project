use serde::{Deserialize, Serialize};

use crate::{Direction, Vector2};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub positions: Vec<Vector2>,
    pub direction: Direction,
}