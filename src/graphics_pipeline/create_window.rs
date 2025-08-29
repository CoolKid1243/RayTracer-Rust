use crate::app::application::State;
use crate::ray_tracer::camera::Camera;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

pub fn print_status(fps: u32, denoising_on: bool, cursor_grabbed: bool) {
    use std::io::{stdout, Write};
    // Move cursor up 3 lines, clear lines, go to start
    print!("\x1B[3F\x1B[0J");
    println!("FPS: {:<3}", fps);
    println!("Denoising: {}", if denoising_on { "ON " } else { "OFF" });
    println!(
        "Mouse: {} (TAB to toggle)",
        if cursor_grabbed { "LOCKED" } else { "FREE  " }
    );
    stdout().flush().unwrap();
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let window: Arc<Window> = Arc::new(
        WindowBuilder::new()
            .with_title("Real-time Ray Tracer")
            .with_inner_size(LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .unwrap(),
    );

    let mut raytracer = Camera::new(650, 10);

    let mut state = State::new(window.as_ref(), &raytracer).await;

    let mut surface_configured = false;

    let main_window_id = window.id();

    // FPS timing variables
    let mut last_time = Instant::now();
    let mut frame_count: u32 = 0;
    let mut last_frame_time = Instant::now();

    // Mouse handling
    let mut cursor_grabbed = true;

    // Initially grab the cursor
    let _ = window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .or_else(|_e| window.set_cursor_grab(winit::window::CursorGrabMode::Locked));
    window.set_cursor_visible(false);

    state.run();

    // Clone Arc into the closure
    let window = Arc::clone(&window);

    let _ = event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == main_window_id => {
                // Always process input first
                state.input(event);

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
                        event:
                            KeyEvent {
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
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::KeyT),
                                ..
                            },
                        ..
                    } => {
                        // Toggle denoising on T key press
                        raytracer.toggle_denoising();
                    }

                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Tab),
                                ..
                            },
                        ..
                    } => {
                        // Toggle cursor on Tab key press
                        cursor_grabbed = !cursor_grabbed;
                        if cursor_grabbed {
                            let _ = window
                                .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                                .or_else(|_e| {
                                    window.set_cursor_grab(winit::window::CursorGrabMode::Locked)
                                });
                            window.set_cursor_visible(false);
                        } else {
                            let _ = window.set_cursor_grab(winit::window::CursorGrabMode::None);
                            window.set_cursor_visible(true);
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        if !surface_configured {
                            return;
                        }

                        // Calculate delta time
                        let now = Instant::now();
                        let dt = now.duration_since(last_frame_time).as_secs_f64();
                        last_frame_time = now;

                        // Process continuous input
                        state.process_continuous_input(&mut raytracer, dt);

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
                            print_status(fps, raytracer.is_denoising_enabled(), cursor_grabbed);
                            frame_count = 0;
                            last_time = Instant::now();
                        }

                        // Request next frame immediately for real-time rendering
                        window.request_redraw();
                    }

                    _ => {}
                }
            }

            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } if cursor_grabbed => {
                    // Process mouse movement for camera rotation
                    raytracer.process_mouse_movement(delta.0, -delta.1); // Negative Y for natural feel
                }
                _ => {}
            },

            Event::AboutToWait => {
                // Continuously request redraws for real-time rendering
                window.request_redraw();
            }

            _ => {}
        }
    });
}