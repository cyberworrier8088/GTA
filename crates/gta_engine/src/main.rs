use std::sync::Arc;
use std::time::Instant;
use glam::{Mat4, Vec3};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, WindowBuilder},
};

use gta_assets::CityGenerator;
use gta_gameplay::{InputState, Player};
use gta_render::{GpuMesh, Renderer};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("GTA 3: Liberty City - Rust Engine")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)
            .unwrap(),
    );

    // Try grabbing cursor for 3D mouse look
    let _ = window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
    window.set_cursor_visible(false);

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    // Generate Liberty City meshes
    let city_mesh_data = CityGenerator::build_city_block();
    let player_mesh_data = CityGenerator::build_player_mesh();
    let car_mesh_data = CityGenerator::build_car_mesh();

    let city_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &city_mesh_data);
    let player_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &player_mesh_data);
    let car_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &car_mesh_data);

    let (_city_buf, city_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);
    let (car_buf, car_bg) = renderer.create_model_bind_group(
        Mat4::from_translation(Vec3::new(8.0, 0.0, 5.0)) * Mat4::from_rotation_y(0.4),
    );
    let (player_buf, player_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);

    let mut player = Player::new();
    let mut input = InputState::default();
    let mut last_time = Instant::now();

    event_loop.set_control_flow(ControlFlow::Poll);
    let _ = event_loop.run(move |event, elwt| match event {
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta: (dx, dy) },
            ..
        } => {
            input.mouse_dx += dx as f32;
            input.mouse_dy += dy as f32;
        }

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => elwt.exit(),

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(key),
                        ..
                    },
                ..
            } => {
                let pressed = *state == ElementState::Pressed;
                match key {
                    KeyCode::KeyW | KeyCode::ArrowUp => input.forward = pressed,
                    KeyCode::KeyS | KeyCode::ArrowDown => input.backward = pressed,
                    KeyCode::KeyA | KeyCode::ArrowLeft => input.left = pressed,
                    KeyCode::KeyD | KeyCode::ArrowRight => input.right = pressed,
                    KeyCode::ShiftLeft | KeyCode::ShiftRight => input.sprint = pressed,
                    KeyCode::Space => input.jump = pressed,
                    KeyCode::KeyF | KeyCode::Enter => input.enter_vehicle = pressed,
                    _ => {}
                }
            }

            WindowEvent::Resized(physical_size) => {
                renderer.resize(*physical_size);
            }

            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - last_time).as_secs_f32().min(0.1);
                last_time = now;

                // Update gameplay
                player.update(&input, dt);
                input.mouse_dx = 0.0;
                input.mouse_dy = 0.0;

                // Sync 3rd person camera
                let (eye, target) = player.get_camera_vectors();
                renderer.camera.eye = eye;
                renderer.camera.target = target;
                renderer.update_camera();

                // Update Claude player transform
                let player_transform = Mat4::from_translation(player.position)
                    * Mat4::from_rotation_y(player.yaw);
                let uniform = gta_render::renderer::ModelUniform {
                    model: player_transform.to_cols_array_2d(),
                };
                renderer.queue.write_buffer(
                    &player_buf,
                    0,
                    bytemuck::cast_slice(&[uniform]),
                );

                // Render scene with 3D Depth
                let render_items = [
                    (&city_gpu_mesh, &city_bg),
                    (&car_gpu_mesh, &car_bg),
                    (&player_gpu_mesh, &player_bg),
                ];

                match renderer.render_scene(&render_items) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            }
            _ => {}
        },
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => {}
    });
}
