use std::f64::consts::PI;

pub const INFINITY_F64: f64 = f64::INFINITY;
pub const PI_F64: f64 = PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI_F64 / 180.0
}
