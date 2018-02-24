pub mod bounds;
pub mod normal;
pub mod point;
pub mod ray;
pub mod sphere;
pub mod vector;


pub type Point2f = point::Point2<f64>;
pub type Point2i = point::Point2<i64>;
pub type Point3f = point::Point3<f64>;
pub type Point3i = point::Point3<i64>;

pub type Vector2f = vector::Vector2<f64>;
pub type Vector2i = vector::Vector2<i64>;
pub type Vector3f = vector::Vector3<f64>;
pub type Vector3i = vector::Vector3<i64>;
