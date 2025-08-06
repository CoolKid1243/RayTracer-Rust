use crate::ray_tracer::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { orig: origin, dir: direction }
    }
    
    #[inline]
    pub fn origin(&self) -> Vec3 { self.orig }
    
    #[inline]
    pub fn direction(&self) -> Vec3 { self.dir }
    
    #[inline]
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}