use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Direction, Player, PlayerState, Vector2};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(TS)]
#[ts(export)]
pub enum Event {
    RemoveApple(Vector2),
    AddApple(Vector2),
    AddPlayer(Player),
    IncreasePlayer(i32),
    RemovePlayer(i32),
    MovePlayer{dir: Direction, id: i32},
    ChangeStatePlayer{state: PlayerState, id: i32},
    SetUsername{id: i32, name: String}
}