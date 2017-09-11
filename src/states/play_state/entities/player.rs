use std::time::Duration;
use ggez::graphics;
use std::collections::HashMap;
use uuid::Uuid;
use states::play_state::entities::Bullet;
use states::play_state::entities::components::*;

const HEIGHT: f32 = 32.0;
const WIDTH: f32 = 64.0;

pub struct Player {
    pub lives: u8,
    position: Position,
    pub velocity: Velocity,
}

impl Player {
    pub fn new() -> Self {
        Player {
            lives: 3,
            position: Position::new(100.0, 500.0),
            velocity: Velocity::new(0.0, 0.0),
        }
    }
}

impl Player {
    pub fn update(&mut self, delta: Duration) {
        self.position.update(&self.velocity, delta);
    }

    pub fn fire(&self, bullets: &mut HashMap<Uuid, Bullet>) {
        let (x, _) = self.get_center();
        let y = self.position.y;
        bullets.insert(Uuid::new_v4(), Bullet::new(x, y, -1.0));
    }
}

impl Drawable for Player {
    fn get_center(&self) -> (f32, f32) {
        (self.position.x + (WIDTH / 2.0), self.position.y + (HEIGHT / 2.0))
    }

    fn get_rect(&self) -> graphics::Rect {
        let (x, y) = self.get_center();
        graphics::Rect::new(x, y, WIDTH, HEIGHT)
    }
}
