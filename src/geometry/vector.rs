use std::ops;

use num_traits::Float;


#[derive(Clone, Copy)]
pub struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    pub fn new(x: i64, y: i64) -> Vector2 {
        Vector2 { x, y }
    }


    pub fn x(&self) -> i64 {
        return self.x;
    }

    pub fn y(&self) -> i64 {
        return self.y;
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();

        return Vector2::new(x, y);
    }
}


#[derive(Clone, Copy)]
pub struct Vector3<T: Float> {
    x: T,
    y: T,
    z: T,
}

impl<T: Float> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> T {
        return self.x;
    }

    pub fn y(&self) -> T {
        return self.y;
    }

    pub fn z(&self) -> T {
        return self.z;
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y
    }

    pub fn set_z(&mut self, z: T) {
        self.z = z
    }

    pub fn length_squared(&self) -> T {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> T {
        return self.length_squared().sqrt();
    }

//    pub fn normalize(&mut self) {
//        let nor2 = self.length_squared();
//        if nor2 > 0 as T {
//            let inv_nor = (1.0 as T) / nor2.sqrt();
//            self.x *= inv_nor;
//            self.y *= inv_nor;
//            self.z *= inv_nor;
//        }
//    }

    pub fn dot(&self, other: &Vector3<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl<T: Float> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        return Vector3::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        );
    }
}

impl<T: Float> ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        return Vector3::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        );
    }
}
