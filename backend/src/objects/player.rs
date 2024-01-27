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
}