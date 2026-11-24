use std::sync::Arc;
use std::time::Instant;
use glam::{Mat4, Vec3};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, WindowBuilder},
};

use gta_assets::{CityGenerator, MeshData, ObjLoader};
use gta_gameplay::{InputState, Player, PlayerState, Vehicle};
use gta_render::{GpuMesh, HudRenderer, Renderer, Texture};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("GTA 3: Liberty City - Textured 3D Production Engine")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)
            .unwrap(),
    );

    let _ = window.set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Locked));
    window.set_cursor_visible(false);

    let mut renderer = pollster::block_on(Renderer::new(window.clone()));

    // 1. Load Real 256x256 GTA 3 Textures
    let brick_texture = Texture::from_file(&renderer.device, &renderer.queue, "assets/textures/brick_wall.png")
        .expect("Load brick_wall.png");
    let brick_tex_bg = renderer.create_texture_bind_group(&brick_texture);

    let road_texture = Texture::from_file(&renderer.device, &renderer.queue, "assets/textures/asphalt_road.png")
        .expect("Load asphalt_road.png");
    let road_tex_bg = renderer.create_texture_bind_group(&road_texture);

    let taxi_texture = Texture::from_file(&renderer.device, &renderer.queue, "assets/textures/taxi_livery.png")
        .expect("Load taxi_livery.png");
    let taxi_tex_bg = renderer.create_texture_bind_group(&taxi_texture);

    let mafia_texture = Texture::from_file(&renderer.device, &renderer.queue, "assets/textures/mafia_suit.png")
        .expect("Load mafia_suit.png");
    let mafia_tex_bg = renderer.create_texture_bind_group(&mafia_texture);

    // 2. City ground & highrises
    let city_mesh_data = CityGenerator::build_city_block();
    let city_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &city_mesh_data);
    let (_city_buf, city_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);

    // 3. Load Realistic Low-Poly Taxi (OBJ Mesh + Real Texture)
    let taxi_mesh_data = ObjLoader::load("assets/liberty_taxi_1999.obj")
        .unwrap_or_else(|_| CityGenerator::build_car_mesh());
    let taxi_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &taxi_mesh_data);
    let (taxi_buf, taxi_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);

    // 4. Load Realistic Mafia Enforcer Character (OBJ Mesh + Real Pinstripe Suit Texture)
    let mafia_mesh_data = ObjLoader::load("assets/mafia_enforcer.obj")
        .unwrap_or_else(|_| CityGenerator::build_player_mesh());
    let mafia_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &mafia_mesh_data);
    let (_mafia_buf, mafia_bg) = renderer.create_model_bind_group(
        Mat4::from_translation(Vec3::new(2.5, 0.0, 4.0)) * Mat4::from_rotation_y(-1.2),
    );

    // 5. Claude player mesh
    let player_mesh_data = CityGenerator::build_player_mesh();
    let player_gpu_mesh = GpuMesh::from_mesh_data(&renderer.device, &player_mesh_data);
    let (player_buf, player_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);

    let (_hud_buf, hud_bg) = renderer.create_model_bind_group(Mat4::IDENTITY);

    let mut player = Player::new();
    let mut taxi = Vehicle::new(Vec3::new(8.0, 0.0, 5.0), 0.4);
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

                // Update gameplay & vehicle kinematics
                player.update(&mut taxi, &input, dt);
                input.mouse_dx = 0.0;
                input.mouse_dy = 0.0;

                // Sync 3rd-person follow camera
                let (eye, target) = player.get_camera_vectors(&taxi);
                renderer.camera.eye = eye;
                renderer.camera.target = target;
                renderer.update_camera();

                // Update Taxi GPU matrix
                let taxi_transform = Mat4::from_translation(taxi.position)
                    * Mat4::from_rotation_y(taxi.yaw);
                let taxi_uniform = gta_render::renderer::ModelUniform {
                    model: taxi_transform.to_cols_array_2d(),
                };
                renderer.queue.write_buffer(
                    &taxi_buf,
                    0,
                    bytemuck::cast_slice(&[taxi_uniform]),
                );

                // Update Claude player GPU matrix
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

                // Build GTA 3 HUD overlay
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

                // Render textured 3D world with atmospheric lighting & depth
                let mut render_items = vec![
                    (&city_gpu_mesh, &city_bg, Some(&brick_tex_bg)),
                    (&mafia_gpu_mesh, &mafia_bg, Some(&mafia_tex_bg)),
                    (&taxi_gpu_mesh, &taxi_bg, Some(&taxi_tex_bg)),
                ];
                if player.state == PlayerState::OnFoot {
                    render_items.push((&player_gpu_mesh, &player_bg, None));
                }
                render_items.push((&hud_gpu_mesh, &hud_bg, None));

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
