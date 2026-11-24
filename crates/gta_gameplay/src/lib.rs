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
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.2, // slightly tilted down
            health: 100.0,
            armor: 100.0,
            state: PlayerState::OnFoot,
            is_grounded: true,
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32) {
        // Update camera look angles
        let mouse_sens = 0.003;
        self.yaw -= input.mouse_dx * mouse_sens;
        self.pitch = (self.pitch - input.mouse_dy * mouse_sens).clamp(-0.8, 1.1);

        if self.state == PlayerState::OnFoot {
            let speed = if input.sprint { 11.0 } else { 5.5 };
            let mut move_dir = Vec3::ZERO;

            // Camera relative movement
            let forward = Vec3::new(self.yaw.sin(), 0.0, self.yaw.cos()).normalize_or_zero();
            let right = Vec3::new(-self.yaw.cos(), 0.0, self.yaw.sin()).normalize_or_zero();

            if input.forward {
                move_dir += forward;
            }
            if input.backward {
                move_dir -= forward;
            }
            if input.right {
                move_dir += right;
            }
            if input.left {
                move_dir -= right;
            }

            if move_dir.length_squared() > 0.0 {
                let norm = move_dir.normalize();
                self.position += norm * speed * dt;
            }

            // Simple gravity and ground collision
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
    }

    pub fn get_camera_vectors(&self) -> (Vec3, Vec3) {
        // GTA 3 third person follow camera
        let cam_dist = 4.8;
        let cam_height = 1.6;

        let target = self.position + Vec3::new(0.0, cam_height, 0.0);
        let cam_offset = Vec3::new(
            -self.yaw.sin() * self.pitch.cos() * cam_dist,
            self.pitch.sin() * cam_dist + 0.4,
            -self.yaw.cos() * self.pitch.cos() * cam_dist,
        );

        let eye = target + cam_offset;
        (eye, target)
    }
}
