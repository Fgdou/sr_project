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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector2::new(0, 0);
        let v2 = Vector2::new(0, 0);
        let expected = Vector2::new(0, 0);

        assert_eq!(expected, v1+v2);

        let v1 = Vector2::new(1, 2);
        let v2 = Vector2::new(3, 4);
        let expected = Vector2::new(4, 6);

        assert_eq!(expected, v1+v2);
    }

    #[test]
    fn test_eq() {
        assert!(Vector2::new(28, 45) == Vector2::new(28, 44) + Vector2::new(0, 1));
        assert!(Vector2::new(28, 45) != Vector2::new(28, 44) + Vector2::new(0, 2));
    }
}