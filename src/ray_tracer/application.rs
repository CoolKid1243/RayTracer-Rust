use std::io::{self, Write};
use std::sync::Arc;

use crate::ray_tracer::vec3::{Vec3, Point3};
use crate::ray_tracer::ray::Ray;
use crate::ray_tracer::hittable_list::HittableList;
use crate::ray_tracer::sphere::Sphere;
use crate::ray_tracer::hit_record::HitRecord;
use crate::ray_tracer::hittable::Hittable;
use crate::ray_tracer::color::{Color, write_color};
use crate::ray_tracer::interval::Interval;

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

pub struct Run {
    image_width: i32,
    image_height: i32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    camera_center: Point3,
    world: HittableList,
}

impl Run {
    pub fn new() -> Self {
        println!("Run");

        // World
        let mut world = HittableList::new();
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        // Image
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 { image_height = 1; }
        println!("P3\n{} {}\n255", image_width, image_height); // Print the image width and height

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

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            camera_center,
            world,
        }
    }

    pub fn update(&self) {
        // Render
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color = ray_color(&r, &self.world);
                write_color(&pixel_color);
            }
        }
    }
}
