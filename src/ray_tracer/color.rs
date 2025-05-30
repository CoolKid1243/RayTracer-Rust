use crate::ray_tracer::vec3::Vec3;

pub type Color = Vec3;

// Write the rgb values of a 'color' to a PPM image format 
// This asssumes each canel is in [0.0, 1.0] and scales to [0, 255]
pub fn write_color(color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Scale and convert to integer in the range [0, 255]
    let r_byte = (255.999 * r) as u8;
    let g_byte = (255.999 * g) as u8;
    let b_byte = (255.999 * b) as u8;

    // Print the color values in PPM format
    println!("{} {} {}", r_byte, g_byte, b_byte);
}
