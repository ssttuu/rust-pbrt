use std::clone;
use std::cmp;
use std::ops;

//type CoefficientSpectrum = [f64];
#[derive(Debug, Clone)]
pub struct RGBSpectrum {
    pub samples: [f64; 3],
}

impl RGBSpectrum {
    pub fn new(default: f64) -> RGBSpectrum {
        RGBSpectrum { samples: [default, default, default] }
    }

    pub fn is_black(&self) -> bool {
        for i in 0..self.samples.len() {
            if self.samples[i] == 0.0 {
                return false;
            }
        }
        return true;
    }

}

impl ops::Add for RGBSpectrum {
    type Output = RGBSpectrum;

    fn add(self, rhs: RGBSpectrum) -> RGBSpectrum {
        let mut ret = self;
        for i in 0..ret.samples.len() {
            ret.samples[i] += rhs.samples[i];
        }
        return ret;
    }
}

impl ops::AddAssign for RGBSpectrum {
    fn add_assign(&mut self, rhs: RGBSpectrum) {
        for i in 0..self.samples.len() {
            self.samples[i] += rhs.samples[i];
        }
    }
}

impl ops::Sub for RGBSpectrum {
    type Output = RGBSpectrum;

    fn sub(self, rhs: RGBSpectrum) -> RGBSpectrum {
        let mut ret = self;
        for i in 0..ret.samples.len() {
            ret.samples[i] -= rhs.samples[i];
        }
        return ret;
    }
}

impl ops::SubAssign for RGBSpectrum {
    fn sub_assign(&mut self, rhs: RGBSpectrum) {
        for i in 0..self.samples.len() {
            self.samples[i] -= rhs.samples[i];
        }
    }
}

impl ops::Mul for RGBSpectrum {
    type Output = RGBSpectrum;

    fn mul(self, rhs: RGBSpectrum) -> RGBSpectrum {
        let mut ret = self;
        for i in 0..ret.samples.len() {
            ret.samples[i] *= rhs.samples[i];
        }
        return ret;
    }
}

impl ops::MulAssign for RGBSpectrum {
    fn mul_assign(&mut self, rhs: RGBSpectrum) {
        for i in 0..self.samples.len() {
            self.samples[i] *= rhs.samples[i];
        }
    }
}

impl ops::Div for RGBSpectrum {
    type Output = RGBSpectrum;

    fn div(self, rhs: RGBSpectrum) -> RGBSpectrum {
        let mut ret = self;
        for i in 0..ret.samples.len() {
            ret.samples[i] /= rhs.samples[i];
        }
        return ret;
    }
}

impl ops::DivAssign for RGBSpectrum {
    fn div_assign(&mut self, rhs: RGBSpectrum) {
        for i in 0..self.samples.len() {
            self.samples[i] /= rhs.samples[i];
        }
    }
}

impl cmp::PartialEq for RGBSpectrum {
    fn eq(&self, other: &RGBSpectrum) -> bool {
        for i in 0..self.samples.len() {
            if self.samples[i] != other.samples[i] {
                return false;
            }
        }
        return true;
    }
}

impl cmp::Eq for RGBSpectrum {}
