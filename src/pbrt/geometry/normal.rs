use std::cmp;
use std::ops;

use num_traits::{Float, One, Zero};

#[derive(Clone, Copy, Debug)]
pub struct Normal3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Normal3<T> {
        Normal3 { x, y, z }
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

    pub fn dot(&self, other: &Normal3<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl<T: Float> ops::Add<Normal3<T>> for Normal3<T> {
    type Output = Normal3<T>;

    fn add(self, rhs: Normal3<T>) -> Normal3<T> {
        return Normal3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        );
    }
}

impl<T: Float> ops::AddAssign<Normal3<T>> for Normal3<T> {
    fn add_assign(&mut self, other: Normal3<T>) {
        *self = Normal3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> ops::Sub<Normal3<T>> for Normal3<T> {
    type Output = Normal3<T>;

    fn sub(self, rhs: Normal3<T>) -> Normal3<T> {
        return Normal3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        );
    }
}

impl<T: Float> ops::SubAssign<Normal3<T>> for Normal3<T> {
    fn sub_assign(&mut self, rhs: Normal3<T>) {
        *self = Normal3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        );
    }
}

impl<T: Float> ops::Mul<T> for Normal3<T> {
    type Output = Normal3<T>;

    fn mul(self, rhs: T) -> Normal3<T> {
        return Normal3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        );
    }
}

impl<T: Float> ops::MulAssign<T> for Normal3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Normal3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        );
    }
}

impl<T: Float> ops::Div<T> for Normal3<T> {
    type Output = Normal3<T>;

    fn div(self, rhs: T) -> Normal3<T> {
        if rhs == Zero::zero() {
            // TODO: raise error?
        };

        return Normal3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        );
    }
}

impl<T: Float> ops::DivAssign<T> for Normal3<T> {
    fn div_assign(&mut self, rhs: T) {
        if rhs == Zero::zero() {
            // TODO: raise error?
            return;
        };

        *self = Normal3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        );
    }
}

impl<T: Float> ops::Index<u8> for Normal3<T> {
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

impl<T: Float> cmp::PartialEq<Normal3<T>> for Normal3<T> {
    fn eq(&self, other: &Normal3<T>) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl<T: Float> cmp::Eq for Normal3<T> {}
