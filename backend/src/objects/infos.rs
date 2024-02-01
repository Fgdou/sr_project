use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Player, Vector2};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub struct Infos {
    pub players: Vec<Player>,
    pub apples: Vec<Vector2>,
    pub size: Vector2,
    pub message_count: i32,
}