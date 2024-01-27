use serde::{Deserialize, Serialize};

use crate::{Player, Vector2};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infos {
    pub players: Vec<Player>,
    pub apples: Vec<Vector2>,
    pub size: Vector2
}