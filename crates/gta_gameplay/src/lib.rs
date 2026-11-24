pub mod vehicle;

pub use vehicle::Vehicle;

use glam::Vec3;

#[derive(Default)]
pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub sprint: bool,
    pub jump: bool,
    pub enter_vehicle: bool,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    OnFoot,
    InVehicle,
}

pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub health: f32,
    pub armor: f32,
    pub state: PlayerState,
    pub is_grounded: bool,
    pub current_vehicle: Option<Vehicle>,
    pub enter_cooldown: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.2,
            health: 100.0,
            armor: 100.0,
            state: PlayerState::OnFoot,
            is_grounded: true,
            current_vehicle: None,
            enter_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, vehicle: &mut Vehicle, input: &InputState, dt: f32) {
        if self.enter_cooldown > 0.0 {
            self.enter_cooldown -= dt;
        }

        // Handle Enter/Exit Vehicle (GTA 3 'F' key)
        if input.enter_vehicle && self.enter_cooldown <= 0.0 {
            self.enter_cooldown = 0.5;
            match self.state {
                PlayerState::OnFoot => {
                    let dist = (self.position - vehicle.position).length();
                    if dist < 4.0 {
                        self.state = PlayerState::InVehicle;
                    }
                }
                PlayerState::InVehicle => {
                    self.state = PlayerState::OnFoot;
                    // Exit to driver's side (left of car)
                    let left = Vec3::new(-vehicle.yaw.cos(), 0.0, vehicle.yaw.sin());
                    self.position = vehicle.position + left * 2.2;
                    self.position.y = 0.0;
                }
            }
        }

        match self.state {
            PlayerState::OnFoot => {
                let mouse_sens = 0.003;
                self.yaw -= input.mouse_dx * mouse_sens;
                self.pitch = (self.pitch - input.mouse_dy * mouse_sens).clamp(-0.8, 1.1);

                let speed = if input.sprint { 11.0 } else { 5.5 };
                let mut move_dir = Vec3::ZERO;

                let forward = Vec3::new(self.yaw.sin(), 0.0, self.yaw.cos()).normalize_or_zero();
                let right = Vec3::new(-self.yaw.cos(), 0.0, self.yaw.sin()).normalize_or_zero();

                if input.forward { move_dir += forward; }
                if input.backward { move_dir -= forward; }
                if input.right { move_dir += right; }
                if input.left { move_dir -= right; }

                if move_dir.length_squared() > 0.0 {
                    let norm = move_dir.normalize();
                    self.position += norm * speed * dt;
                }

                if !self.is_grounded {
                    self.velocity.y -= 20.0 * dt;
                    self.position.y += self.velocity.y * dt;
                    if self.position.y <= 0.0 {
                        self.position.y = 0.0;
                        self.velocity.y = 0.0;
                        self.is_grounded = true;
                    }
                } else if input.jump {
                    self.velocity.y = 7.5;
                    self.is_grounded = false;
                }
            }

            PlayerState::InVehicle => {
                let throttle = if input.forward { 1.0 } else { 0.0 };
                let brake = if input.backward { 1.0 } else { 0.0 };
                let steer = if input.left { -1.0 } else if input.right { 1.0 } else { 0.0 };
                let handbrake = input.jump; // Space = Handbrake in car

                vehicle.update(throttle, brake, steer, handbrake, dt);
                self.position = vehicle.position;
                self.yaw = vehicle.yaw;
            }
        }
    }

    pub fn get_camera_vectors(&self, vehicle: &Vehicle) -> (Vec3, Vec3) {
        match self.state {
            PlayerState::OnFoot => {
                let cam_dist = 4.8;
                let cam_height = 1.6;
                let target = self.position + Vec3::new(0.0, cam_height, 0.0);
                let cam_offset = Vec3::new(
                    -self.yaw.sin() * self.pitch.cos() * cam_dist,
                    self.pitch.sin() * cam_dist + 0.4,
                    -self.yaw.cos() * self.pitch.cos() * cam_dist,
                );
                (target + cam_offset, target)
            }
            PlayerState::InVehicle => {
                // GTA 3 cinematic vehicle chase camera
                let cam_dist = 7.5;
                let cam_height = 2.4;
                let target = vehicle.position + Vec3::new(0.0, 1.2, 0.0);
                let cam_offset = Vec3::new(
                    -vehicle.yaw.sin() * cam_dist,
                    cam_height,
                    -vehicle.yaw.cos() * cam_dist,
                );
                (target + cam_offset, target)
            }
        }
    }
}

