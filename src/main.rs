mod ray_tracer;

use std::io::{self, Write};
use std::sync::Arc;

use ray_tracer::vec3::{Vec3, Point3};
use ray_tracer::ray::Ray;
use ray_tracer::hit_record::HitRecord;
use ray_tracer::hittable::Hittable;
use ray_tracer::hittable_list::HittableList;
use ray_tracer::sphere::Sphere;
use ray_tracer::color::{Color, write_color};
use ray_tracer::interval::Interval;

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate image height and make sure it's atleast 1
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 { image_height = 1; }

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_center = pixel00_loc
                + (i as f64 * pixel_delta_u)
                + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&pixel_color);
        }
    }

    eprintln!("\rDone.                 ");
}
