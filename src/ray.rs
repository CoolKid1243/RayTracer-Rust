use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    // Constructor
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { orig: origin, dir: direction }
    }

    // Get origin
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    // Get direction
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Compute point along the ray at parameter t
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
