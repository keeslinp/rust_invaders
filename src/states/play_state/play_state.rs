extern crate ggez;
extern crate uuid;
use ggez::event::Keycode;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use main_state::MainState;
use std::collections::HashMap;
use uuid::Uuid;
use states::GameState;
use states::play_state::entities::Player;
use states::play_state::entities::Drawable;
use states::play_state::entities::Bullet;
use states::play_state::Invaders;

struct World {
    player: Player,
    invaders: Invaders,
    bullets: HashMap<Uuid, Bullet>,
}

pub struct PlayState {
    font: graphics::Font,
    world: World,
}

impl PlayState {
    pub fn new(ctx: &mut Context) -> Self {
        let font = graphics::Font::new(ctx, "/OpenSans-Regular.ttf", 20).unwrap();
        let invaders = Invaders::new();
        let player = Player::new();
        PlayState {
            font,
            world: World {
                invaders,
                player,
                bullets: HashMap::new(),
            },
        }
    }
}

impl PlayState {
    fn draw_world(&self, ctx: &mut Context) -> GameResult<()> {
        self.world.player.draw(ctx)?;
        self.world.invaders.draw(ctx)?;
        for (_, bullet) in self.world.bullets.iter() {
            bullet.draw(ctx)?;
        }
        Ok(())
    }
    fn draw_info(&self, ctx: &mut Context) -> GameResult<()> {
        let dest_point = graphics::Point::new(700.0, 25.0);
        let lives_string = format!("lives: {}", &self.world.player.lives.to_string());
        let lives_text = graphics::Text::new(ctx, &lives_string, &self.font)?;
        graphics::draw(ctx, &lives_text, dest_point, 0.0)?;
        Ok(())
    }
}

impl PlayState {
    fn update_bullets(&mut self, delta: Duration) {
        let mut bullets_to_remove = vec![];
        let mut enemies_to_remove = vec![];
        for (key, bullet) in (&mut self.world.bullets).iter_mut() {
            bullet.update(&mut self.world.invaders.invaders, &mut enemies_to_remove, &mut bullets_to_remove, key, delta);
        }
        for key in bullets_to_remove {
            self.world.bullets.remove(&key);
        }

        for key in enemies_to_remove {
            self.world.invaders.invaders.remove(&key);
        }
    }
}

impl GameState for PlayState{
    fn update(&mut self, keys: &mut HashMap<Keycode, bool>, _: &mut Context, delta: Duration) -> GameResult<(usize)> {
        for (key, value) in keys.drain() {
            match (key, value) {
                (Keycode::Q, true) => {
                    return Ok(0);
                },
                (Keycode::Right, val) => {
                    let dx = self.world.player.velocity.dx;
                    self.world.player.velocity.dx = if val { 1.0 } else { 
                        if dx == 1.0 {
                            0.0
                        } else {
                            dx
                        }
                    };
                },
                (Keycode::Left, val) => {
                    let dx = self.world.player.velocity.dx;
                    self.world.player.velocity.dx = if val { -1.0 } else { 
                        if dx == -1.0 {
                            0.0
                        } else {
                            dx
                        }
                    };
                },
                (Keycode::Space, true) => {
                    self.world.player.fire(&mut self.world.bullets);
                },
                _ => {
                }
            }
        }
        self.world.player.update(delta);
        self.world.invaders.update(delta, &mut self.world.bullets);
        self.update_bullets(delta);
        Ok(1)
    }

    fn draw(&self, _: &MainState, ctx: &mut Context) -> GameResult<()> {
        self.draw_world(ctx)?;
        self.draw_info(ctx)?;
        Ok(())
    }
}
