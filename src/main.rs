mod ray_tracer;
mod graphics_pipeline;
mod app;

use graphics_pipeline::create_window::run;

fn main() {
    pollster::block_on(run());
}