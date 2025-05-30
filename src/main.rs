mod vec3;
mod color;
mod ray;
mod hittable;
mod common;

use std::{io::{self, Write}};
use color::{Color, write_color};
use vec3::{Vec3, Point3};
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = Vec3::dot(&r.direction(), &oc);
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h - a*c;
    
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

pub fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

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

            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }

    eprintln!("\rDone.                 ");
}
