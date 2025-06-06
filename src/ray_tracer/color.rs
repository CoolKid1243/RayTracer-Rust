use crate::ray_tracer::vec3::Color;
use crate::ray_tracer::interval::Interval;

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        linear.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(c: &Color) {
    // We’ll clamp each gamma component
    let intensity = Interval::new(0.0, 0.999);

    let r = intensity.clamp(linear_to_gamma(c.x()));
    let g = intensity.clamp(linear_to_gamma(c.y()));
    let b = intensity.clamp(linear_to_gamma(c.z()));

    // Convert [0, 1] to RGBA [0, 255]
    let r_byte = (256.0 * r) as u8;
    let g_byte = (256.0 * g) as u8;
    let b_byte = (256.0 * b) as u8;

    println!("{} {} {}", r_byte, g_byte, b_byte);
}
