use cgmath::Vector2;

use crate::Direction;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub positions: Vec<Vector2<i32>>,
    pub direction: Direction,
}