use std::cmp;
use std::ops;

use num_traits::{Float, One, Zero};

use pbrt;
use pbrt::geometry::point::Point2;
use pbrt::geometry::point::Point3;
use pbrt::geometry::vector::Vector2;
use pbrt::geometry::vector::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Bounds2<T: Float> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T: Float> Bounds2<T> {
    pub fn diagonal(&self) -> Vector2<T> {
        return self.max - self.min;
    }

    pub fn area(&self) -> T {
        let d = self.diagonal();
        return d.x * d.y;
    }

    pub fn maximum_extent(&self) -> u8 {
        let d = self.diagonal();
        if d.x > d.y {
            return 0;
        }
        return 1;
    }

    pub fn lerp(&self, other: Point2<T>) -> Point2<T> {
        return Point2::new(
            pbrt::lerp(other.x, self.min.x, self.max.x),
            pbrt::lerp(other.y, self.min.y, self.max.y),
        );
    }

    pub fn offset(&self, other: Point2<T>) -> Vector2<T> {
        let mut o = other - self.min;
        if self.max.x > self.min.x {
            o.x = o.x / (self.max.x - self.min.x);
        }
        if self.max.y > self.min.y {
            o.y = o.y / (self.max.y - self.min.y);
        }

        return o;
    }
}

impl<T: Float> ops::Index<u8> for Bounds2<T> {
    type Output = Point2<T>;

    fn index(&self, index: u8) -> &Point2<T> {
        if index == 0 {
            return &self.min;
        }
        return &self.max;
    }
}

impl<T: Float> cmp::PartialEq for Bounds2<T> {
    fn eq(&self, other: &Bounds2<T>) -> bool {
        return self.min == other.min && self.max == other.max;
    }
}

impl<T: Float> cmp::Eq for Bounds2<T> {}


#[derive(Clone, Copy, Debug)]
pub struct Bounds3<T: Float> {
    pub min: Point3<T>,
    pub max: Point3<T>,
}

impl<T: Float> Bounds3<T> {
    pub fn corner(&self, corner: u8) -> Point3<T> {
        return Point3::new(
            self[(corner & 1)].x,
            self[if corner & 2 > 0 { 1 } else { 0 }].y,
            self[if corner & 4 > 0 { 1 } else { 0 }].z,
        );
    }

    pub fn diagonal(&self) -> Vector3<T> {
        return self.max - self.min;
    }

    pub fn surface_area(&self) -> T {
        let d = self.diagonal();
        return (d.x * d.y + d.x * d.z + d.y * d.z) + (d.x * d.y + d.x * d.z + d.y * d.z);
    }

    pub fn volume(&self) -> T {
        let d = self.diagonal();
        return d.x * d.y * d.z;
    }

    pub fn maximum_extent(&self) -> u8 {
        let d = self.diagonal();
        if d.x > d.y {
            return 0;
        }
        return 1;
    }

    pub fn lerp(&self, other: Point3<T>) -> Point3<T> {
        return Point3::new(
            pbrt::lerp(other.x, self.min.x, self.max.x),
            pbrt::lerp(other.y, self.min.y, self.max.y),
            pbrt::lerp(other.z, self.min.z, self.max.z),
        );
    }

    pub fn offset(&self, other: Point3<T>) -> Vector3<T> {
        let mut o = other - self.min;
        if self.max.x > self.min.x {
            o.x = o.x / (self.max.x - self.min.x);
        }
        if self.max.y > self.min.y {
            o.y = o.y / (self.max.y - self.min.y);
        }

        return o;
    }
}

impl<T: Float> ops::Index<u8> for Bounds3<T> {
    type Output = Point3<T>;

    fn index(&self, index: u8) -> &Point3<T> {
        if index == 0 {
            return &self.min;
        }
        return &self.max;
    }
}

impl<T: Float> cmp::PartialEq for Bounds3<T> {
    fn eq(&self, other: &Bounds3<T>) -> bool {
        return self.min == other.min && self.max == other.max;
    }
}

impl<T: Float> cmp::Eq for Bounds3<T> {}
