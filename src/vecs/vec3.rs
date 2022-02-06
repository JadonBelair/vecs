use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign}};
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

impl<T: Float> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
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

impl<T: Float + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}