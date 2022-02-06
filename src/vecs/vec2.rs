use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign, Mul, Div}};
use num_traits::Float;

/// implementation of a 2D vector
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2<T: Float> {
    x: T,
    y: T
}

impl<T: Float + Copy> Vec2<T> {
    /// returns a new Vec2 with the specified coordinates
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec2;
    /// 
    /// // creates a new Vec2 called v1
    /// let v1 = Vec2::new(1., 2.);
    /// 
    /// // creates a new Vec2 call v2
    /// let v2 = Vec2::new(10., 20.);
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
    /// let v1 = Vec2::new(1., 2.);
    /// let v2 = Vec2::new(1., 2.);
    /// 
    /// // stores their dot product
    /// let d = v1.dot(v2);
    /// 
    /// assert_eq!(5, d);
    /// ```
    pub fn dot(&self, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normal(&self) -> Vec2<T> {
        Vec2::new(-self.y, self.x)
    }

    pub fn normalize(&self) -> Vec2<T> {
        let length = self.length();

        Vec2::new(self.x / length, self.y / length)
    }
}

impl<T: Float> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T: Float> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T: Float + AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Float + SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Float + Mul + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    
    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2::new(self.x * rhs, self.y * rhs)
    }

}

impl<T: Float> Mul<Vec2<T>> for f32 where f32: Mul<T, Output = T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl<T: Float + Mul + Copy> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    
    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }

}

impl<T: Float + Div + Copy> Div<T> for Vec2<T> {
    type Output = Vec2<T>;
    
    fn div(self, rhs: T) -> Vec2<T> {
        Vec2::new(self.x / rhs, self.y / rhs)
    }

}

impl<T: Float> Div<Vec2<T>> for f32 where f32: Div<T, Output = T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self / rhs.x, self / rhs.y)
    }
}

impl<T: Float + Div + Copy> Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    
    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }

}

impl<T: Float + fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}