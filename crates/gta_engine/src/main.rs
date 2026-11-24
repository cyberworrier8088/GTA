use std::sync::Arc;
use std::time::Instant;
use glam::{Mat4, Vec3};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, WindowBuilder},
};

use gta_assets::{CityGenerator, MeshData};
use gta_gameplay::{InputState, Player, PlayerState, Vehicle};
use gta_render::{GpuMesh, HudRenderer, Renderer};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("GTA 3: Liberty City - Rust Production Engine")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)
            .unwrap(),
    );

    // Lock cursor for authentic 3D look
    let _ = window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
    window.set_cursor_visible(false);

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    // Generate Liberty City world meshes
    let city_mesh_data = CityGenerator::build_city_block();
    let player_mesh_data = CityGenerator::build_player_mesh();
    let car_mesh_data = CityGenerator::build_car_mesh();

    let city_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &city_mesh_data);
    let player_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &player_mesh_data);
    let car_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &car_mesh_data);

    let (_city_buf, city_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);
    let (car_buf, car_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);
    let (player_buf, player_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);
    let (_hud_buf, hud_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);

    // Initial game entities
    let mut player = Player::new();
    let mut car = Vehicle::new(Vec3::new(8.0, 0.0, 5.0), 0.4);
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

                // 1. Update Game Physics & States
                player.update(&mut car, &input, dt);
                input.mouse_dx = 0.0;
                input.mouse_dy = 0.0;

                // 2. Camera tracking
                let (eye, target) = player.get_camera_vectors(&car);
                renderer.camera.eye = eye;
                renderer.camera.target = target;
                renderer.update_camera();

                // 3. Update Car GPU Transform Buffer
                let car_transform = Mat4::from_translation(car.position)
                    * Mat4::from_rotation_y(car.yaw);
                let car_uniform = gta_render::renderer::ModelUniform {
                    model: car_transform.to_cols_array_2d(),
                };
                renderer.queue.write_buffer(
                    &car_buf,
                    0,
                    bytemuck::cast_slice(&[car_uniform]),
                );

                // 4. Update Player GPU Transform Buffer (hide or position Claude)
                let player_pos = if player.state == PlayerState::InVehicle {
                    Vec3::new(0.0, -100.0, 0.0) // hide inside car
                } else {
                    player.position
                };
                let player_transform = Mat4::from_translation(player_pos)
                    * Mat4::from_rotation_y(player.yaw);
                let player_uniform = gta_render::renderer::ModelUniform {
                    model: player_transform.to_cols_array_2d(),
                };
                renderer.queue.write_buffer(
                    &player_buf,
                    0,
                    bytemuck::cast_slice(&[player_uniform]),
                );

                // 5. Build dynamic GTA 3 HUD overlay
                let (hud_verts, hud_indices) = HudRenderer::build_hud_geometry(
                    renderer.camera.aspect,
                    player.health,
                    player.armor,
                );
                let hud_mesh_data = MeshData {
                    name: "hud".to_string(),
                    vertices: hud_verts,
                    indices: hud_indices,
                };
                let hud_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &hud_mesh_data);

                // Render scene items + HUD
                let mut render_items = vec![
                    (&city_gpu_mesh, &city_bg),
                    (&car_gpu_mesh, &car_bg),
                ];
                if player.state == PlayerState::OnFoot {
                    render_items.push((&player_gpu_mesh, &player_bg));
                }
                render_items.push((&hud_gpu_mesh, &hud_bg));

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
