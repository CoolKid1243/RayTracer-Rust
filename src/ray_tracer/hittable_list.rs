use std::sync::Arc;
use crate::ray_tracer::hittable::Hittable;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn hit(&self, r: &crate::ray_tracer::ray::Ray, t_range: crate::ray_tracer::interval::Interval, rec: &mut crate::ray_tracer::hit_record::HitRecord) -> bool {
        let mut temp_rec = crate::ray_tracer::hit_record::HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_range.max;
        for object in &self.objects {
            if object.hit(r, crate::ray_tracer::interval::Interval::new(t_range.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
