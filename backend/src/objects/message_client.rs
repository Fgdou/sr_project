use serde::{Deserialize, Serialize};

use crate::Direction;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageClient {
    Connection(String),
    ChangeDirection(Direction)
}