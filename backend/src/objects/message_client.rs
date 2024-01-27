use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::Direction;

#[derive(TS)]
#[ts(export)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageClient {
    Connection(String),
    ChangeDirection(Direction)
}