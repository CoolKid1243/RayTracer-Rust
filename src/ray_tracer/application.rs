use std::sync::Arc;
use std::io::Write;

use crate::ray_tracer::vec3::{Vec3, Point3, Color};
use crate::ray_tracer::ray::Ray;
use crate::ray_tracer::hittable_list::HittableList;
use crate::ray_tracer::sphere::Sphere;
use crate::ray_tracer::hit_record::HitRecord;
use crate::ray_tracer::interval::Interval;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

pub struct RayTracerApp {
    image_width: u32,
    image_height: u32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    camera_center: Point3,
    world: HittableList,
}

impl RayTracerApp {
    pub fn new() -> Self {
        // World
        let mut world = HittableList::new();
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        // Window size
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 800;
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 { image_height = 1; }

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        RayTracerApp {
            image_width,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            camera_center,
            world,
        }
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }
    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    pub fn render_rgba(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; (self.image_width * self.image_height * 4) as usize];
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + self.pixel_delta_u * (i as f64)
                    + self.pixel_delta_v * (j as f64);
                let r = Ray::new(self.camera_center, pixel_center - self.camera_center);
                let pixel_color = ray_color(&r, &self.world);
                let r_u8 = (pixel_color.x().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
                let g_u8 = (pixel_color.y().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
                let b_u8 = (pixel_color.z().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
                let idx = ((j * self.image_width + i) * 4) as usize;
                buffer[idx] = r_u8;
                buffer[idx + 1] = g_u8;
                buffer[idx + 2] = b_u8;
                buffer[idx + 3] = 255;
            }
            
            eprint!("\rScanlines remaining: {}", self.image_height - 1 - j);
            std::io::stdout().flush().unwrap();
        }
        buffer
    }
}
