use std::cmp;
use std::ops;

use num_traits::{Float, One, Zero};

use pbrt::geometry::vector::Vector2;
use pbrt::geometry::vector::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Point2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x, y }
    }

    pub fn length_squared(&self) -> T {
        return self.x * self.x + self.y * self.y;
    }

    pub fn length(&self) -> T {
        return self.length_squared().sqrt();
    }
}


impl<T: Float> ops::Add<Point2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Point2<T>) -> Point2<T> {
        return Point2::new(
            self.x + rhs.x,
            self.y + rhs.y,
        );
    }
}

impl<T: Float> ops::AddAssign<Point2<T>> for Point2<T> {
    fn add_assign(&mut self, other: Point2<T>) {
        *self = Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Float> ops::Sub<Point2<T>> for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Point2<T>) -> Vector2<T> {
        return Vector2::new(
            self.x - rhs.x,
            self.y - rhs.y,
        );
    }
}

impl<T: Float> ops::SubAssign<Point2<T>> for Point2<T> {
    fn sub_assign(&mut self, rhs: Point2<T>) {
        *self = Point2::new(
            self.x - rhs.x,
            self.y - rhs.y,
        );
    }
}

impl<T: Float> ops::Mul<T> for Point2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: T) -> Point2<T> {
        return Point2::new(
            self.x * rhs,
            self.y * rhs,
        );
    }
}

impl<T: Float> ops::MulAssign<T> for Point2<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Point2::new(
            self.x * rhs,
            self.y * rhs,
        );
    }
}

impl<T: Float> ops::Div<T> for Point2<T> {
    type Output = Point2<T>;

    fn div(self, rhs: T) -> Point2<T> {
        if rhs == Zero::zero() {
            // TODO: raise error?
        };

        return Point2::new(
            self.x / rhs,
            self.y / rhs,
        );
    }
}

impl<T: Float> ops::DivAssign<T> for Point2<T> {
    fn div_assign(&mut self, rhs: T) {
        if rhs == Zero::zero() {
            // TODO: raise error?
            return;
        };

        *self = Point2::new(
            self.x / rhs,
            self.y / rhs,
        );
    }
}

impl<T: Float> ops::Index<u8> for Point2<T> {
    type Output = T;

    fn index(&self, index: u8) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.y, // FIXME: THIS IS REALLY WRONG!
        }
    }
}

impl<T: Float> cmp::PartialEq<Point2<T>> for Point2<T> {
    fn eq(&self, other: &Point2<T>) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl<T: Float> cmp::Eq for Point2<T> {}


#[derive(Clone, Copy, Debug)]
pub struct Point3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z }
    }

    pub fn length_squared(&self) -> T {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> T {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&mut self) {
        let nor2 = self.length_squared();
        if nor2 > Zero::zero() {
            let one: T = One::one();
            let inv_nor = one / nor2.sqrt();
            self.x = self.x * inv_nor;
            self.y = self.y * inv_nor;
            self.z = self.z * inv_nor;
        }
    }

    pub fn dot(&self, other: &Point3<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl<T: Float> ops::Add<Point3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Point3<T>) -> Point3<T> {
        return Point3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        );
    }
}

impl<T: Float> ops::AddAssign<Point3<T>> for Point3<T> {
    fn add_assign(&mut self, other: Point3<T>) {
        *self = Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> ops::Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Vector3<T> {
        return Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        );
    }
}

impl<T: Float> ops::SubAssign<Point3<T>> for Point3<T> {
    fn sub_assign(&mut self, rhs: Point3<T>) {
        *self = Point3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        );
    }
}

impl<T: Float> ops::Mul<T> for Point3<T> {
    type Output = Point3<T>;

    fn mul(self, rhs: T) -> Point3<T> {
        return Point3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        );
    }
}

impl<T: Float> ops::MulAssign<T> for Point3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Point3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        );
    }
}

impl<T: Float> ops::Div<T> for Point3<T> {
    type Output = Point3<T>;

    fn div(self, rhs: T) -> Point3<T> {
        if rhs == Zero::zero() {
            // TODO: raise error?
        };

        return Point3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        );
    }
}

impl<T: Float> ops::DivAssign<T> for Point3<T> {
    fn div_assign(&mut self, rhs: T) {
        if rhs == Zero::zero() {
            // TODO: raise error?
            return;
        };

        *self = Point3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        );
    }
}

impl<T: Float> ops::Index<u8> for Point3<T> {
    type Output = T;

    fn index(&self, index: u8) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.z, // FIXME: THIS IS REALLY WRONG!
        }
    }
}

impl<T: Float> cmp::PartialEq<Point3<T>> for Point3<T> {
    fn eq(&self, other: &Point3<T>) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl<T: Float> cmp::Eq for Point3<T> {}

//
// VECTORS
//
impl<T: Float> ops::Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Point3<T> {
        return Point3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        );
    }
}

impl<T: Float> ops::AddAssign<Vector3<T>> for Point3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        *self = Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> ops::Sub<Vector3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        return Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        );
    }
}

impl<T: Float> ops::SubAssign<Vector3<T>> for Point3<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        *self = Point3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        );
    }
}

impl<T: Float> cmp::PartialEq<Vector3<T>> for Point3<T> {
    fn eq(&self, other: &Vector3<T>) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}
