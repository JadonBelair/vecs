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

#[allow(unused, unused_imports)]

use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign, Mul, Div}};

mod vecs;

pub use vecs::{vec2::Vec2, vec3::Vec3};

#[cfg(test)]
mod tests {

    use super::{Vec2, Vec3};

    #[test]
    fn vec2_equal() {
        assert_eq!(Vec2::new(5., -5.), Vec2::new(5., -5.));
    }

    #[test]
    fn vec2_not_equal() {
        assert_ne!(Vec2::new(5., 6.), Vec2::new(6., 5.));
    }

    #[test]
    fn vec2_add() {
        assert_eq!(Vec2::new(5., 5.), Vec2::new(2., 3.) + Vec2::new(3., 2.));
    }

    #[test]
    fn vec2_sub() {
        assert_eq!(Vec2::new(-5., 20.), Vec2::new(0., 30.) - Vec2::new(5., 10.));
    }

    #[test]
    fn vec2_dot() {
        assert_eq!(200., Vec2::new(10., 10.).dot(Vec2::new(10., 10.)));
    }

    #[test]
    fn vec3_equal() {
        assert_eq!(Vec3::new(5., 5., 5.), Vec3::new(5., 5., 5.));
    }

    #[test]
    fn vec3_not_equal() {
        assert_ne!(Vec3::new(5., 6., 7.), Vec3::new(6., 5., 9.));
    }

    #[test]
    fn vec3_add() {
        assert_eq!(Vec3::new(5., 5., 5.), Vec3::new(2., 7., 1.) + Vec3::new(3., -2., 4.));
    }

    #[test]
    fn vec3_sub() {
        assert_eq!(Vec3::new(5., 20., 7.), Vec3::new(10., 30., 10.) - Vec3::new(5., 10., 3.));
    }

    #[test]
    fn vec3_dot() {
        assert_eq!(300., Vec3::new(10., 10., 10.).dot(Vec3::new(10., 10., 10.)));
    }

    #[test]
    fn vec3_cross() {
        assert_eq!(Vec3::new(10., 51., -42.), Vec3::new(3., 6., 8.).cross(Vec3::new(9., 4., 7.)));
    }

}
