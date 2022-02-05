//! Useful vector math implementations

use num::Num;
use std::{fmt, ops::{Add, Sub}};

#[allow(unused)]

/// implementation of a 2d vector
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2<T: Num> {
    x: T,
    y: T
}

impl<T: Num + Copy> Vec2<T> {
    /// returns a new Vec2 with the specified coordinates
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    /// returns the dot product of 2 2d vectors
    pub fn dot(&self, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Num> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Num> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T: Num + std::fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[cfg(test)]
mod tests {

    use super::Vec2;

    #[test]
    fn vec2_equal() {
        assert_eq!(Vec2::new(5, 5), Vec2::new(5, 5));
    }

    #[test]
    fn vec2_not_equal() {
        assert_ne!(Vec2::new(5, 6), Vec2::new(6, 5));
    }

    #[test]
    fn vec2_add() {
        assert_eq!(Vec2::new(5, 5), Vec2::new(2, 3) + Vec2::new(3, 2));
    }

    #[test]
    fn vec2_sub() {
        assert_eq!(Vec2::new(5, 20), Vec2::new(10, 30) - Vec2::new(5, 10));
    }

    #[test]
    fn vec2_dot() {
        assert_eq!(200, Vec2::new(10, 10).dot(Vec2::new(10, 10)));
    }

}
