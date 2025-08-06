use crate::ray_tracer::vec3::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord { p: Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0), t: 0.0 }
    }
}