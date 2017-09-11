use std::time::Duration;
use states::play_state::entities::components::*;
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            x,
            y,
        }
    }
    pub fn update(&mut self, velocity: &Velocity, delta: Duration) {
        let dt = delta.as_secs() as u32 * 1000 + (delta.subsec_nanos() / 1000000);
        self.x += velocity.dx * dt as f32;
        self.y += velocity.dy * dt as f32;
    }
}
