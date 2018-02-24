use std::cmp;
use std::num;
use std::ops;

use num_traits::{Float, One, Zero};

use pbrt;
use pbrt::geometry::Point2f;
use pbrt::geometry::Vector3f;
use pbrt::geometry::ray::Ray;
use pbrt::Spectrum;

pub trait PhaseFunction {
    fn p(&self, wo: &Vector3f, wi: &Vector3f) -> f64;
    fn sample_p(&self, wo: &Vector3f, wi: &Vector3f, u: &Point2f) -> f64;
}

pub fn get_medium_scattering_properties(name: &String, sigma_a: &Spectrum, sigma_s: &Spectrum) -> bool {
    // TODO
    return true;
}

pub fn phase_hg(cos_theta: f64, g: f64) -> f64 {
    let denom:f64 = 1.0 + g * g + 2.0 * g * cos_theta;
    return pbrt::INV4PI * (1.0 - g * g) / (denom * denom.sqrt());
}

pub trait Medium {
//    fn tr(ray: &Ray, sampler: &Sampler) -> Spectrum;
}



