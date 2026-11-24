use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    OnFoot,
    EnteringVehicle,
    InVehicle,
    ExitingVehicle,
}

pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub health: f32,
    pub armor: f32,
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 1.0, 0.0),
            velocity: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            health: 100.0,
            armor: 100.0,
            state: PlayerState::OnFoot,
        }
    }
}
