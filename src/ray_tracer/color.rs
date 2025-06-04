use crate::ray_tracer::vec3::Color;
use crate::ray_tracer::interval::Interval;

pub fn write_color(c: &Color) {
    let intensity = Interval::new(0.0, 0.999);
    let r = intensity.clamp(c.x().sqrt());
    let g = intensity.clamp(c.y().sqrt());
    let b = intensity.clamp(c.z().sqrt());

    let rbyte = (256.0 * r) as u8;
    let gbyte = (256.0 * g) as u8;
    let bbyte = (256.0 * b) as u8;

    //println!("{} {} {}", rbyte, gbyte, bbyte);
}
