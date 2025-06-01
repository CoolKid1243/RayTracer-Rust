use crate::ray_tracer::hittable::Hittable;
use crate::ray_tracer::ray::Ray;
use crate::ray_tracer::hit_record::HitRecord;
use crate::ray_tracer::interval::Interval;
use crate::ray_tracer::vec3::{Vec3, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.x() * r.direction().x() + oc.y() * r.direction().y() + oc.z() * r.direction().z();
        let c = oc.length().powi(2) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_range.min || root > t_range.max {
            root = (-half_b + sqrtd) / a;
            if root < t_range.min || root > t_range.max {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        true
    }
}
