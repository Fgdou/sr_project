use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
#[ts(export)]
pub enum Direction {
    Up, Down, Left, Right
}