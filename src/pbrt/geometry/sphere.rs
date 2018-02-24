use std::f64;

use pbrt::geometry::Vector3f;


#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vector3f,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3f, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn center(&self) -> Vector3f {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn intersects(&self, origin: &Vector3f, direction: &Vector3f) -> (f64, f64, bool) {
        let l = self.center() - *origin;
        let tca = l.dot(direction);

        if tca < 0.0 {
            return (f64::MAX, f64::MAX, false);
        }

        let d2 = l.length_squared() - tca * tca;
        if d2 > self.radius * self.radius {
            return (f64::MAX, f64::MAX, false);
        }

        let thc = (self.radius * self.radius).sqrt() - d2;

        let t0 = tca - thc;
        let t1 = tca + thc;
        return (t0, t1, true);
    }
}
