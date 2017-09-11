use states::play_state::entities::components::*;
use states::play_state::entities::Invader;
use ggez::graphics;
use std::collections::HashMap;
use uuid::Uuid;
use std::time::Duration;

const WIDTH:f32 = 3.0;
const HEIGHT:f32 = 20.0;

pub struct Bullet {
    pub position: Position,
    velocity: Velocity,
}

impl Bullet {
    pub fn new(x: f32, y: f32, dy: f32) -> Bullet {
        Bullet {
            position: Position::new(x, y),
            velocity: Velocity::new(0.0, dy),
        }
    }
}

impl Bullet {
    fn collides(&self, invader: &mut Invader) -> bool {
        let Position {x, y} = self.position;
        x > invader.position.x && x < invader.position.x + 32.0 && y < invader.position.y + 32.0 && y > invader.position.y
    }
    pub fn update(&mut self, invaders: &mut HashMap<Uuid, Invader>, invaders_to_remove: &mut Vec<Uuid>, bullets_to_remove: &mut Vec<Uuid>, bullet_key: &Uuid, delta: Duration) {
        self.position.update(&self.velocity, delta);
        if self.position.y < 5.0 {
            bullets_to_remove.push(bullet_key.clone());
        } else if self.velocity.dy < 0.0 {
            for (key, invader) in invaders.iter_mut() {
                if self.collides(invader) {
                    invaders_to_remove.push(key.clone());
                    bullets_to_remove.push(bullet_key.clone());
                }
            }
        }
    }
}

impl Drawable for Bullet {
    fn get_center(&self) -> (f32, f32) {
        (self.position.x + (WIDTH / 2.0), self.position.y + (HEIGHT / 2.0))
    }
    fn get_rect(&self) -> graphics::Rect {
        let (x, y) = self.get_center();
        graphics::Rect::new(x, y, WIDTH, HEIGHT)
    }
}
