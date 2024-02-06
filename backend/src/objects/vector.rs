use std::ops::Add;

use rand::Rng;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
    pub fn rand(max: &Vector2) -> Vector2 {
        let mut rng = rand::thread_rng();
        Vector2{
            x: rng.gen_range(0..max.x),
            y: rng.gen_range(0..max.y),
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}