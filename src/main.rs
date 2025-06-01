mod texture;
mod ray_tracer;
mod graphics_pipeline;

use graphics_pipeline::create_window::run;

fn main() {
    pollster::block_on(run());  // Application run
}
