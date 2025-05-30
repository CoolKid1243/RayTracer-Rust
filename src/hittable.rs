use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::interval::Interval;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
