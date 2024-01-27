use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    x: i32,
    y: i32
}