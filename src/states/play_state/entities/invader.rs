use states::play_state::entities::components::*;
use states::play_state::entities::Bullet;
use std::collections::HashMap;
use uuid::Uuid;
use std::time::Duration;
use ggez::graphics;

const HEIGHT: f32 = 32.0;
const WIDTH: f32 = 32.0;

pub struct Invader {
    pub position: Position,
}

impl Invader {
    pub fn new(x: f32, y: f32) -> Self {
        Invader {
            position: Position::new(x, y),
        }
    }


    pub fn update(&mut self, velocity: &Velocity, delta: Duration) {
        self.position.update(velocity, delta);
    }

    pub fn move_down(&mut self) {
        self.position.y += HEIGHT * 1.5;
    }

    pub fn fire(&self, bullets: &mut HashMap<Uuid, Bullet>) {
        let (x, _) = self.get_center();
        let y = self.position.y + HEIGHT;
        bullets.insert(Uuid::new_v4(), Bullet::new(x, y, 1.0));
    }
}

impl Drawable for Invader {
    fn get_center(&self) -> (f32, f32) {
        (self.position.x + (WIDTH / 2.0), self.position.y + (HEIGHT / 2.0))
    }
    fn get_rect(&self) -> graphics::Rect {
        let (x, y) = self.get_center();
        graphics::Rect::new(x, y, WIDTH, HEIGHT)
    }
}
