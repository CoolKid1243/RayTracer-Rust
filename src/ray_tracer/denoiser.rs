use crate::ray_tracer::pixel_data::PixelData;
use crate::ray_tracer::vec3::{Color, Vec3};
use rayon::prelude::*;

pub struct Denoiser {
    width: u32,
    height: u32,
    kernel_radius: i32,
    spatial_kernel: Vec<Vec<f64>>,
    inv_sigma_color: f64,
    inv_sigma_depth: f64,
}

impl Denoiser {
    pub fn new(width: u32, height: u32) -> Self {
        let kernel_radius = 2;
        let spatial_kernel = Self::make_spatial_kernel(kernel_radius);
        Self {
            width,
            height,
            kernel_radius,
            spatial_kernel,
            inv_sigma_color: 1.0 / (2.0 * 0.3 * 0.3),
            inv_sigma_depth: 1.0 / (2.0 * 10.0 * 10.0),
        }
    }

    pub fn denoise(&self, pixel_data: &[PixelData]) -> Vec<Color> {
        (0..self.height)
            .into_par_iter()
            .flat_map_iter(|y| {
                let mut row = Vec::with_capacity(self.width as usize);
                for x in 0..self.width {
                    row.push(self.denoise_pixel(x, y, pixel_data));
                }
                row
            })
            .collect()
    }

    fn denoise_pixel(&self, x: u32, y: u32, pixel_data: &[PixelData]) -> Color {
        let center_idx = self.get_pixel_index(x, y);
        let center = pixel_data[center_idx];

        let mut total_weight = 0.0f64;
        let mut result = Color::new(0.0, 0.0, 0.0);

        for ky in -self.kernel_radius..=self.kernel_radius {
            for kx in -self.kernel_radius..=self.kernel_radius {
                let nx = x as i32 + kx;
                let ny = y as i32 + ky;
                if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                    continue;
                }

                let neighbor_idx = self.get_pixel_index(nx as u32, ny as u32);
                let neighbor = pixel_data[neighbor_idx];

                let spatial_weight = self.spatial_kernel[(ky + self.kernel_radius) as usize][(kx + self.kernel_radius) as usize];

                // Feature weights
                let color_weight = self.calculate_color_similarity(center.color, neighbor.color);
                let depth_weight = self.calculate_depth_similarity(center.depth as f64, neighbor.depth as f64);
                let normal_weight = self.calculate_normal_similarity(center.normal, neighbor.normal);

                let w = spatial_weight * color_weight * depth_weight * normal_weight;
                result = result + neighbor.color * w;
                total_weight += w;
            }
        }

        if total_weight > 0.0 {
            result / total_weight
        } else {
            center.color
        }
    }

    fn make_spatial_kernel(radius: i32) -> Vec<Vec<f64>> {
        let sigma = radius as f64 * 0.5;
        let inv = 1.0 / (2.0 * sigma * sigma);
        (-radius..=radius)
            .map(|dy| {
                (-radius..=radius)
                    .map(|dx| {
                        let dist2 = (dx * dx + dy * dy) as f64;
                        (-dist2 * inv).exp()
                    })
                    .collect()
            })
            .collect()
    }

    #[inline]
    fn get_pixel_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    #[inline]
    fn calculate_color_similarity(&self, c1: Color, c2: Color) -> f64 {
        let diff = (c1 - c2).length();
        (-diff * diff * self.inv_sigma_color).exp()
    }

    #[inline]
    fn calculate_depth_similarity(&self, d1: f64, d2: f64) -> f64 {
        let diff = (d1 - d2).abs();
        (-diff * diff * self.inv_sigma_depth).exp()
    }

    #[inline]
    fn calculate_normal_similarity(&self, n1: Vec3, n2: Vec3) -> f64 {
        if n1.length() == 0.0 || n2.length() == 0.0 {
            return 1.0;
        }
        Vec3::dot(n1, n2).max(0.0)
    }
}