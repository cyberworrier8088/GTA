use glam::Vec3;
use gta_physics::fast_inv_sqrt;

#[derive(Clone, Debug)]
pub struct Vehicle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub speed: f32,
    pub steering: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub reverse_speed: f32,
}

impl Vehicle {
    pub fn new(pos: Vec3, yaw: f32) -> Self {
        Self {
            position: pos,
            velocity: Vec3::ZERO,
            yaw,
            speed: 0.0,
            steering: 0.0,
            max_speed: 28.0,      // ~100 km/h arcade GTA 3 speed
            acceleration: 15.0,
            reverse_speed: 10.0,
        }
    }

    pub fn update(&mut self, throttle: f32, brake: f32, steer: f32, handbrake: bool, dt: f32) {
        // Arcade steering response
        let steer_target = steer * 0.7; // max steering angle
        self.steering += (steer_target - self.steering) * 10.0 * dt;

        // Throttle & Braking
        if throttle > 0.0 {
            self.speed += self.acceleration * throttle * dt;
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        } else if brake > 0.0 {
            if self.speed > 0.5 {
                // Braking while going forward
                self.speed -= 25.0 * brake * dt;
                if self.speed < 0.0 {
                    self.speed = 0.0;
                }
            } else {
                // Reverse
                self.speed -= self.reverse_speed * brake * dt;
                if self.speed < -self.reverse_speed {
                    self.speed = -self.reverse_speed;
                }
            }
        } else {
            // Natural rolling friction
            let friction = if handbrake { 40.0 } else { 4.5 };
            if self.speed > 0.0 {
                self.speed = (self.speed - friction * dt).max(0.0);
            } else if self.speed < 0.0 {
                self.speed = (self.speed + friction * dt).min(0.0);
            }
        }

        // Steer while moving (GTA 3 arcade handling)
        if self.speed.abs() > 0.1 {
            let steer_mult = if self.speed >= 0.0 { 1.0 } else { -1.0 };
            self.yaw += self.steering * (self.speed / self.max_speed).abs().clamp(0.2, 1.0) * 2.2 * steer_mult * dt;
        }

        // Forward vector
        let forward = Vec3::new(self.yaw.sin(), 0.0, self.yaw.cos());
        // Fast normalization with assembly
        let inv_len = fast_inv_sqrt(forward.length_squared().max(0.0001));
        let norm_forward = forward * inv_len;

        self.velocity = norm_forward * self.speed;
        self.position += self.velocity * dt;
    }
}
