use cgmath::Vector2;

use crate::Player;

pub struct Infos {
    pub players: Vec<Player>,
    pub apples: Vec<Vector2<i32>>,
    pub size: Vector2<i32>
}