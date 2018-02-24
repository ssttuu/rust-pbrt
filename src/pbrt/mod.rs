pub mod geometry;
pub mod medium;
pub mod sampler;
pub mod spectrum;

use num_traits::{Float, One};
use typenum::{U3, U60};


type Spectrum = spectrum::RGBSpectrum;

pub const INV4PI:f64 = 0.07957747154594766788;

pub fn lerp<T: Float>(t: T, v1: T, v2: T) -> T {
    return (T::one() - t) * v1 + t * v2;
}
