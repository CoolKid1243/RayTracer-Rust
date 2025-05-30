use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
