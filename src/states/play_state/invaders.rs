use rand::{thread_rng, Rng};
use states::play_state::entities::Invader;
use states::play_state::entities::Drawable;
use states::play_state::entities::Bullet;
use states::play_state::entities::Velocity;
use std::collections::HashMap;
use uuid::Uuid;
use ggez::{GameResult, Context};
use std::time::Duration;
pub struct Invaders {
    pub invaders: HashMap<Uuid, Invader>,
    velocity: Velocity,
    fire_time: Duration,
    pub move_count: u8,
}

fn create_invaders() -> HashMap<Uuid, Invader> {
    let mut invaders = HashMap::new();
    for row in 1..5_u8 {
        for col in 1..10_u8 {
            invaders.insert(Uuid::new_v4(), Invader::new(col as f32 * 50.0, row as f32 * 50.0));
        }
    }
    invaders
}

impl Invaders {
    pub fn new() -> Self {
        Invaders {
            invaders: create_invaders(),
            velocity: Velocity::new(0.1, 0.0),
            fire_time: Duration::new(1, 0),
            move_count: 0,
        }
    }


    fn should_reverse(&self) -> bool {
        self.invaders.iter().any(|(_, invader)| invader.position.x > 750.0 || invader.position.x <= 0.0)
    }

    fn move_all_down(&mut self) {
        self.move_count += 1;
        for (_, invader) in self.invaders.iter_mut() {
            invader.move_down();
        }
    }

    fn fire(&mut self, bullets: &mut HashMap<Uuid, Bullet>) {
        let mut rng = thread_rng();
        let key_index = rng.gen_range(0, self.invaders.len());
        let key = self.invaders.keys().nth(key_index).unwrap();
        self.invaders.get(&key).unwrap().fire(bullets);
        // bullets.insert(Uuid::new_v4(), Bullet::new(invader.position.x, invader.position.y, 1.0));
    }

    pub fn update(&mut self, delta: Duration, bullets: &mut HashMap<Uuid, Bullet>) {
        for (_, invader) in self.invaders.iter_mut() {
            invader.update(&self.velocity, delta);
        }
        if self.should_reverse() {
            self.velocity.dx *= -1.1;
            self.move_all_down();
        }
        match self.fire_time.checked_sub(delta) {
            Some(dur) => {
                self.fire_time = dur;
            },
            None => {
                self.fire(bullets);
                self.fire_time = Duration::new(1, 0);
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for (_, invader) in self.invaders.iter() {
            invader.draw(ctx)?;
        }
        Ok(())
    }

    pub fn has_landed(&self) -> bool {
        self.move_count > 4
    }
}
