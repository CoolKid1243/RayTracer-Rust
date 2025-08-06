use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use rayon::prelude::*;

use crate::ray_tracer::vec3::{Vec3, Point3, Color};
use crate::ray_tracer::ray::Ray;
use crate::ray_tracer::hittable_list::HittableList;
use crate::ray_tracer::sphere::Sphere;
use crate::ray_tracer::hit_record::HitRecord;
use crate::ray_tracer::interval::Interval;
use crate::ray_tracer::pixel_data::PixelData;
use crate::ray_tracer::denoiser::Denoiser;

struct FastRng {
    state: u64,
}

impl FastRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((self.state >> 32) as u32 as f64) / (u32::MAX as f64)
    }
    
    fn random_in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let p = Vec3::new(
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

fn ray_color_iterative_with_data(r: &Ray, world: &HittableList, depth: u32, rng: &mut FastRng) -> (Color, PixelData) {
    let mut current_ray = *r;
    let mut attenuation = Color::new(1.0, 1.0, 1.0);
    let mut pixel_data = PixelData::new();
    let mut first_hit = true;
    
    for _ in 0..depth {
        let mut rec = HitRecord::new();
        if world.hit(&current_ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            
            // Store G-buffer data from first hit only
            if first_hit {
                pixel_data.depth = rec.t as f32;
                pixel_data.normal = rec.normal;
                pixel_data.albedo = Color::new(0.7, 0.7, 0.7); // Default material color
                first_hit = false;
            }
            
            // Lambertian diffuse with optimized random direction
            let target = rec.p + rec.normal + Vec3::unit_vector(&rng.random_in_unit_sphere());
            current_ray = Ray::new(rec.p, target - rec.p);
            attenuation = attenuation * 0.5;
        } else {
            // Background gradient
            let unit_direction = Vec3::unit_vector(&current_ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            let background = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
            let final_color = attenuation * background;
            pixel_data.color = final_color;
            return (final_color, pixel_data);
        }
    }
    
    // Exceeded depth
    let black = Color::new(0.0, 0.0, 0.0);
    pixel_data.color = black;
    (black, pixel_data)
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    max_depth: u32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    camera_center: Point3,
    world: Arc<HittableList>,
    pixel_buffer: Vec<PixelData>,
    sample_count: AtomicU32,
    current_frame: u32,
    denoiser: Denoiser,
    enable_denoising: bool,
}

impl Camera {
    pub fn new(image_width: u32, max_depth: u32) -> Self {
        // World setup
        let mut world = HittableList::new();
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        // Image and aspect ratio
        let aspect_ratio = 16.0 / 9.0;
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }

        // Camera geometry
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

        let buffer_size = (image_width * image_height) as usize;
        
        Self {
            image_width,
            image_height,
            max_depth,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            camera_center,
            world: Arc::new(world),
            pixel_buffer: vec![PixelData::new(); buffer_size],
            sample_count: AtomicU32::new(0),
            current_frame: 0,
            denoiser: Denoiser::new(image_width, image_height),
            enable_denoising: false,
        }
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    pub fn reset_accumulation(&mut self) {
        self.pixel_buffer.fill(PixelData::new());
        self.sample_count.store(0, Ordering::Relaxed);
        self.current_frame = 0;
    }

    pub fn render_progressive(&mut self) {
        let samples_this_frame = 1; // Samples per frame
        self.current_frame += 1;
        
        // Use parallel processing with rayon
        let pixel_results: Vec<(Color, PixelData)> = (0..self.image_height * self.image_width)
            .into_par_iter()
            .map(|pixel_idx| {
                let j = pixel_idx / self.image_width;
                let i = pixel_idx % self.image_width;
                
                let seed = (self.current_frame as u64) * 1000000 + pixel_idx as u64;
                let mut rng = FastRng::new(seed);
                
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                let mut combined_data = PixelData::new();
                
                for _ in 0..samples_this_frame {
                    let u = i as f64 + rng.next();
                    let v = j as f64 + rng.next();

                    let pixel_sample = self.pixel00_loc
                        + self.pixel_delta_u * u
                        + self.pixel_delta_v * v;

                    let ray = Ray::new(self.camera_center, pixel_sample - self.camera_center);
                    let (sample_color, sample_data) = ray_color_iterative_with_data(&ray, &self.world, self.max_depth, &mut rng);
                    
                    pixel_color += sample_color;
                    
                    // Accumulate G-buffer data
                    if samples_this_frame == 1 {
                        combined_data = sample_data;
                    }
                }
                
                combined_data.color = pixel_color;
                (pixel_color, combined_data)
            })
            .collect();

        // Accumulate samples in pixel buffer
        for (i, (sample_color, sample_data)) in pixel_results.iter().enumerate() {
            self.pixel_buffer[i].color += *sample_color;
            self.pixel_buffer[i].depth = sample_data.depth;
            self.pixel_buffer[i].normal = sample_data.normal;
            self.pixel_buffer[i].albedo = sample_data.albedo;
            self.pixel_buffer[i].sample_count += 1;
        }
        
        self.sample_count.fetch_add(samples_this_frame, Ordering::Relaxed);
    }

    pub fn render_rgba(&self) -> Vec<u8> {
        let mut buffer = vec![0u8; (self.image_width * self.image_height * 4) as usize];
        let sample_count = self.sample_count.load(Ordering::Relaxed) as f64;
        
        if sample_count == 0.0 {
            return buffer; // Return black if no samples yet
        }
        
        let scale = 1.0 / sample_count;
        
        // Get colors 
        let colors = if self.enable_denoising && sample_count >= 4.0 {
            let mut scaled_pixel_data = self.pixel_buffer.clone();
            for pixel in &mut scaled_pixel_data {
                pixel.color = pixel.color * scale; 
            }
            
            // Apply denoising to the scaled data
            self.denoiser.denoise(&scaled_pixel_data)
        } else {
            // Use raw accumulated colors
            self.pixel_buffer.iter().map(|p| p.color * scale).collect()
        };
        
        for (i, color) in colors.iter().enumerate() {
            // Gamma correction and clamping
            let r = (color.x().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
            let g = (color.y().sqrt().clamp(0.0, 0.999) * 256.0) as u8;
            let b = (color.z().sqrt().clamp(0.0, 0.999) * 256.0) as u8;

            let idx = i * 4;
            buffer[idx] = r;
            buffer[idx + 1] = g;
            buffer[idx + 2] = b;
            buffer[idx + 3] = 255;
        }

        buffer
    }
    
    pub fn get_sample_count(&self) -> u32 {
        self.sample_count.load(Ordering::Relaxed)
    }
    
    pub fn get_pixel_data(&self) -> &[PixelData] {
        &self.pixel_buffer
    }

    pub fn is_denoising_enabled(&self) -> bool {
        self.enable_denoising
    }

    pub fn toggle_denoising(&mut self) {
        self.enable_denoising = !self.enable_denoising;
    }
}
