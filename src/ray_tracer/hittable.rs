use crate::ray_tracer::ray::Ray;
use crate::ray_tracer::hit_record::HitRecord;
use crate::ray_tracer::interval::Interval;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool;
}
