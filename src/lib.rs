//! Useful vector math implementations
//! 
//! # Examples
//! 
//! ```
//! use vecs::Vec2;
//! 
//! fn main() {
//!     // creates 2 Vec2 objects
//!     let v1 = Vec2::new(12, 6);
//!     let v2 = Vec2::new(17, 9);
//! 
//!     // adds the vectors together
//!     let v3 = v1 + v2;
//! 
//!     // prints (29, 15) to the console
//!     println!("{}", v3);
//! }
//! ```
//! 
//! ```
//! use vecs::Vec3;
//! 
//! fn main() {
//!     // creates 2 Vec3 objects
//!     let v1 = Vec3::new(2, 6, 7);
//!     let v2 = Vec3::new(5, 3, 8);
//! 
//!     // gets the 2 vectors cross product
//!     let v3 = v1.cross(v2);
//! 
//!     // prints (27, 19, -24) to the console
//!     println!("{}", v3);
//! }
//! ```

use std::{fmt, ops::{Add, Sub, Mul, Neg}};

#[allow(unused)]

/// implementation of a 2D vector
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2<T: Add + Sub + Mul> {
    x: T,
    y: T
}

impl<T: Add<Output = T> + Sub + Mul<Output = T> + Copy> Vec2<T> {
    /// returns a new Vec2 with the specified coordinates
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec2;
    /// 
    /// // creates a new Vec2 called v1
    /// let v1 = Vec2::new(1, 2);
    /// 
    /// // creates a new Vec2 call v2
    /// let v2 = Vec2::new(10, 20);
    /// ```
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    /// returns the dot product of 2 2D vectors
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec2;
    /// 
    /// // creates 2 new Vec2 objects
    /// let v1 = Vec2::new(1, 2);
    /// let v2 = Vec2::new(1, 2);
    /// 
    /// // stores their dot product
    /// let d = v1.dot(v2);
    /// 
    /// assert_eq!(5, d);
    /// ```
    pub fn dot(&self, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Sub + Mul + Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Sub<Output = T> + Add + Mul> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T: Add + Sub + Mul + std::fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// implementation of a 3D vector
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3<T: Add + Sub + Mul + Neg> {
    x: T,
    y: T,
    z: T
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Neg<Output = T>> Vec3<T> {
    /// returns a new Vec3 with the specified coordinates
    ///
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates a new Vec3 called v1
    /// let v1 = Vec3::new(1, 2, 3);
    /// 
    /// // creates a new Vec3 call v2
    /// let v2 = Vec3::new(10, 20, 30);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    /// returns the dot product of 2 3D vectors
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates 2 new Vec3 objects
    /// let v1 = Vec3::new(1, 2, 3);
    /// let v2 = Vec3::new(1, 2, 3);
    /// 
    /// // stores their dot product
    /// let d = v1.dot(v2);
    /// 
    /// assert_eq!(14, d);
    /// ```
    pub fn dot(&self, other: Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// returns the cross product of 2 3D vectors
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates 2 new Vec3 objects
    /// let v1 = Vec3::new(3, 2, 1);
    /// let v2 = Vec3::new(1, 2, 3);
    /// 
    /// // stores their cross product
    /// let v3 = v1.cross(v2);
    /// 
    /// assert_eq!(Vec3::new(4, -8, 4), v3);
    /// ```
    pub fn cross(&self, other: Vec3<T>) -> Vec3<T> {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.x * other.z) - (self.z * other.x);
        let z = (self.x * other.y) - (self.y * other.x);

        Vec3::new(x, -y, z)
    }
}

impl<T: Add<Output = T> + Sub + Mul + Neg> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<T: Add + Sub<Output = T> + Mul + Neg> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<T: Add + Sub + Mul + Neg + std::fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


#[cfg(test)]
mod tests {

    use super::{Vec2, Vec3};

    #[test]
    fn vec2_equal() {
        assert_eq!(Vec2::new(5, -5), Vec2::new(5, -5));
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
        assert_eq!(Vec2::new(-5, 20), Vec2::new(0, 30) - Vec2::new(5, 10));
    }

    #[test]
    fn vec2_dot() {
        assert_eq!(200, Vec2::new(10, 10).dot(Vec2::new(10, 10)));
    }

    #[test]
    fn vec3_equal() {
        assert_eq!(Vec3::new(5, 5, 5), Vec3::new(5, 5, 5));
    }

    #[test]
    fn vec3_not_equal() {
        assert_ne!(Vec3::new(5, 6, 7), Vec3::new(6, 5, 9));
    }

    #[test]
    fn vec3_add() {
        assert_eq!(Vec3::new(5, 5, 5), Vec3::new(2, 7, 1) + Vec3::new(3, -2, 4));
    }

    #[test]
    fn vec3_sub() {
        assert_eq!(Vec3::new(5, 20, 7), Vec3::new(10, 30, 10) - Vec3::new(5, 10, 3));
    }

    #[test]
    fn vec3_dot() {
        assert_eq!(300, Vec3::new(10, 10, 10).dot(Vec3::new(10, 10, 10)));
    }

    #[test]
    fn vec3_cross() {
        assert_eq!(Vec3::new(10, 51, -42), Vec3::new(3, 6, 8).cross(Vec3::new(9, 4, 7)));
    }

}
