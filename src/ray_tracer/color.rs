use crate::ray_tracer::vec3::Color;

pub fn write_color(c: &Color) {
    let ir = (255.999 * c.x().clamp(0.0, 0.999)) as i32;
    let ig = (255.999 * c.y().clamp(0.0, 0.999)) as i32;
    let ib = (255.999 * c.z().clamp(0.0, 0.999)) as i32;
    println!("{} {} {}", ir, ig, ib);
}
