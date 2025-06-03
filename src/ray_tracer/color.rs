use crate::ray_tracer::vec3::Color;

pub fn write_color(c: &Color) {
    let ir = (c.x().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ig = (c.y().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ib = (c.z().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
    let ia = 255;
    println!("r: {:3}, g: {:3}, b: {:3}, a: {}", ir, ig, ib, ia);
}
