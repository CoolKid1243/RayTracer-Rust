use crate::app::application::State;
use crate::ray_tracer::camera::Camera;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
    dpi::LogicalSize,
};
use std::time::{Instant, Duration};

pub fn print_status(fps: u32, denoising_on: bool) {
    use std::io::{stdout, Write};
    // Move cursor up 2 lines, clear both lines, go to start
    print!("\x1B[2F\x1B[0J"); 
    println!("FPS: {:<3}", fps);
    println!("Denoising: {}", if denoising_on { "ON " } else { "OFF" });
    stdout().flush().unwrap();
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Real-time Ray Tracer")
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();
        
    let mut raytracer = Camera::new(650, 10);
    let mut state = State::new(&window, &raytracer).await;
    let mut surface_configured = false;

    // FPS timing variables
    let mut last_time = Instant::now();
    let mut frame_count: u32 = 0;

    state.run();
    
    let _ = event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                            surface_configured = true;
                        }
                        WindowEvent::KeyboardInput {
                            event: KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::KeyR),
                                ..
                            },
                            ..
                        } => {
                            // Reset accumulation on R key press
                            raytracer.reset_accumulation();
                        }
                        WindowEvent::KeyboardInput {
                            event: KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::KeyT),
                                ..
                            },
                            ..
                        } => {
                            // Toggle denoising on T key press
                            raytracer.toggle_denoising();
                        }
                        WindowEvent::RedrawRequested => {
                            if !surface_configured {
                                return;
                            }
                            
                            // Progressive rendering - one sample per frame
                            raytracer.render_progressive();
                            
                            state.update();
                            state.update_image(&raytracer);
                            
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size)
                                }
                                Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                                    log::error!("OutOfMemory");
                                    control_flow.exit();
                                }
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout");
                                }
                            }

                            // --- FPS calculation ---
                            frame_count += 1;
                            let elapsed = last_time.elapsed();
                            if elapsed >= Duration::from_secs(1) {
                                let fps = frame_count / elapsed.as_secs() as u32;
                                print_status(fps, raytracer.is_denoising_enabled());
                                frame_count = 0;
                                last_time = Instant::now();
                            }
                            
                            // Request next frame immediately for real-time rendering
                            state.window().request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                // Continuously request redraws for real-time rendering
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}