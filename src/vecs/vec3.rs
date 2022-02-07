use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign, Div, Mul, Neg}};
use num_traits::Float;

/// implementation of a 3D vector
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3<T: Float> {
    x: T,
    y: T,
    z: T
}

impl<T: Float + Copy> Vec3<T> {
    /// returns a new Vec3 with the specified coordinates
    ///
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates a new Vec3 called v1
    /// let v1 = Vec3::new(1., 2., 3.);
    /// 
    /// // creates a new Vec3 call v2
    /// let v2 = Vec3::new(10., 20., 30.);
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
    /// let v1 = Vec3::new(1., 2., 3.);
    /// let v2 = Vec3::new(1., 2., 3.);
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
    /// let v1 = Vec3::new(3., 2., 1.);
    /// let v2 = Vec3::new(1., 2., 3.);
    /// 
    /// // stores their cross product
    /// let v3 = v1.cross(v2);
    /// 
    /// assert_eq!(Vec3::new(4., -8., 4.), v3);
    /// ```
    pub fn cross(&self, other: Vec3<T>) -> Vec3<T> {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.x * other.z) - (self.z * other.x);
        let z = (self.x * other.y) - (self.y * other.x);

        Vec3::new(x, -y, z)
    }

    /// returns the length of the Vec3
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates a new Vec3
    /// let v = Vec3::new(10., 10., 10.);
    /// 
    /// // gets its length
    /// let len = v.length();
    /// 
    /// assert_eq!(f64::sqrt(300.), len);
    /// ```
    pub fn length(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// returns the normalized the Vec3
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates a new Vec3
    /// let v = Vec3::new(100., 0., 0.);
    /// 
    /// // stores the normalized Vec3
    /// let n = v.normalize();
    /// 
    /// assert_eq!(Vec3::new(1., 0., 0.), n);
    /// ```
    pub fn normalize(&self) -> Vec3<T> {
        let length = self.length();

        *self / length
    }
    
    /// returns the absolute version of the Vec3
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates a new Vec3
    /// let v = Vec3::new(-12., 15., -9.);
    /// 
    /// // stores it's absolute variant
    /// let a = v.abs();
    /// 
    /// assert_eq!(Vec3::new(12., 15., 9.), a);
    /// ```
    pub fn abs(&self) -> Vec3<T> {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// sets the x, y, and z values of the Vec3
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vecs::Vec3;
    /// 
    /// // creates a new Vec3
    /// let mut v = Vec3::new(9., 7., 1.);
    /// 
    /// // gives v new values
    /// v.set(5., 0., 8.);
    /// 
    /// assert_eq!(Vec3::new(5., 0., 8.), v);
    /// ```
    pub fn set(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T: Float + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Float + SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Float + Mul + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    
    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }

}

impl<T: Float> Mul<Vec3<T>> for f32 where f32: Mul<T, Output = T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl<T: Float + Mul + Copy> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

}

impl<T: Float + Div + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    
    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }

}

impl<T: Float> Div<Vec3<T>> for f32 where f32: Div<T, Output = T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl<T: Float + Div + Copy> Div<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    
    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }

}

impl<T: Float> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Float + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}