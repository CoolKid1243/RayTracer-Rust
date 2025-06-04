use std::f64::consts::PI;
use rand::Rng;

pub const INFINITY_F64: f64 = f64::INFINITY;
pub const PI_F64: f64 = PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI_F64 / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    // Returns a random real in [0, 1)
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min, max)
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
