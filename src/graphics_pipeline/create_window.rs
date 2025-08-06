use crate::app::application::State;
use crate::ray_tracer::camera::Camera;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
    dpi::LogicalSize,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Ray Tracer")
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();
        
    let mut raytracer = Camera::new(650, 10);
    let mut state = State::new(&window, &raytracer).await;
    let mut surface_configured = false;

    state.run();
    
    let _ = event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
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
                        WindowEvent::RedrawRequested => {
                            if !surface_configured {
                                return;
                            }
                            
                            // Progressive rendering - one sample per frame
                            raytracer.render_progressive();
                            
                            state.update();
                            state.update_image(&raytracer);
                            
                            // Print sample count
                            if raytracer.get_sample_count() % 10 == 0 {
                                println!("Samples: {}", raytracer.get_sample_count());
                            }
                            
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