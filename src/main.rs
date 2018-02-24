mod pbrt;

extern crate image;
extern crate num_traits;
extern crate generic_array;
extern crate typenum;

use std::f64;
use std::fs::File;

use pbrt::geometry::vector::Vector3;
use pbrt::geometry::sphere::Sphere;


pub struct CameraSettings {
    pub width: i64,
    pub height: i64,
    pub inverse_width: f64,
    pub inverse_height: f64,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub angle: f64,
}

impl CameraSettings {
    pub fn new(width: i64, height: i64) -> CameraSettings {
        let fov = 30.0;
        return CameraSettings {
            width,
            height,
            inverse_width: 1.0 / (width as f64),
            inverse_height: 1.0 / (height as f64),
            fov,
            aspect_ratio: (width as f64) / (height as f64),
            angle: (f64::consts::PI * 0.5 * fov / 180.0).tan(),
        };
    }
}

fn trace(origin: Vector3<f64>, direction: Vector3<f64>, spheres: &Vec<Sphere>, depth: i64) -> image::Luma<u8> {
	let mut t_near = f64::MAX;

	//var sphere geometry.Sphere
	let mut found_sphere = false;

	// Find sphere intersection
	let mut t0: f64;
    let mut t1: f64;
	let mut intersects: bool;

	for (i, _) in (0..spheres.len()).enumerate() {
		let resp = spheres[i].intersects(&origin, &direction);

        t0 = resp.0;
        t1 = resp.1;
        intersects = resp.2;

		if intersects {
			if t0 < 0.0 {
				t0 = t1;
			}
			if t0 < t_near {
				t_near = t0;
				found_sphere = true;
			}
		}
	}

	if !found_sphere {
		return image::Luma([0u8])
	}

	return image::Luma([255u8])
}


fn main() {

    let mut spheres: Vec<Sphere> = Vec::new();
//    spheres.push(Sphere::new(Vector3::new()));

    let n = 9;

	for (i, _) in (0..n).enumerate() {
		for (j, _) in (0..n).enumerate()  {
			for (k, _) in (0..n).enumerate()  {
				let x = (i as f64 / n as f64 * 200.0) - 100.0;
				let y = (j as f64 / n as f64 * 200.0) - 100.0;
				let z = (k as f64 / n as f64 * 200.0) - 100.0;
				let mut radius = 1.0;
				if x > 0.0 {
					radius = 2.0;
				}
				spheres.push(Sphere::new(Vector3::new(x, y, z), radius))
			}
		}
	}

    let camera = CameraSettings::new(192, 108);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let imgbuf = image::ImageBuffer::from_fn(camera.width as u32, camera.height as u32, |x, y| {
        let xx = (2.0 * ((x as f64 + 0.5) * camera.inverse_width) - 1.0) * camera.angle * camera.aspect_ratio;
		let yy = (1.0 - 2.0 * ((y as f64 + 0.5) * camera.inverse_height)) * camera.angle;

        let mut ray_dir = Vector3::new(xx, yy, -1.0);
//        ray_dir.normalize();
        let nor2 = ray_dir.length_squared();
        if nor2 > 0.0 {
            let inv_nor = 1.0 / nor2.sqrt();
            ray_dir.x *= inv_nor;
            ray_dir.y *= inv_nor;
            ray_dir.z *= inv_nor;
        }

        return trace(origin, ray_dir, &spheres, 0);
    });

    let ref mut fout = File::create("test.png").unwrap();

    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();


}