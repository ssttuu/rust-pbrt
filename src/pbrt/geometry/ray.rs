use pbrt::geometry::Point3f;
use pbrt::geometry::Vector3f;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,
    pub t_max: f64,
    pub time: f64,
    //    pub medium: &Medium,
}

impl Ray {
    pub fn new(origin: Point3f, direction: Vector3f, t_max: f64, time: f64) -> Ray {
        Ray { origin, direction, t_max, time }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RayDifferential {
    pub origin: Point3f,
    pub direction: Vector3f,
    pub t_max: f64,
    pub time: f64,

    pub has_differentials: bool,
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}

impl RayDifferential {
    pub fn scale_differentials(&mut self, s: f64) {
        self.rx_origin = self.origin + (self.rx_origin - self.origin) * s;
        self.ry_origin = self.origin + (self.ry_origin - self.origin) * s;
        self.rx_direction = self.direction + (self.rx_direction - self.direction) * s;
        self.ry_direction = self.direction + (self.ry_direction - self.direction) * s;

    }
}