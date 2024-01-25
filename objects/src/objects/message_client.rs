use crate::Direction;

pub enum MessageClient {
    Connection(String),
    ChangeDirection(Direction)
}