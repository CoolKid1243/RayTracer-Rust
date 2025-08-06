use crate::ray_tracer::vec3::{Vec3, Color};

#[derive(Clone, Copy, Debug)]
pub struct PixelData {
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub albedo: Color,
    pub sample_count: u32,
}

impl PixelData {
    pub fn new() -> Self {
        Self {
            color: Color::new(0.0, 0.0, 0.0),
            depth: f32::INFINITY,
            normal: Vec3::new(0.0, 0.0, 0.0),
            albedo: Color::new(0.0, 0.0, 0.0),
            sample_count: 0,
        }
    }
}