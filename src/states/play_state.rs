extern crate ggez;
use ggez::event::Keycode;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use main_state::MainState;
use std::collections::HashMap;
use states::GameState;

pub struct PlayState {
    text: graphics::Text,
}

impl PlayState {
    pub fn new(font: &graphics::Font, ctx: &mut Context) -> Self {
        PlayState {
            text: graphics::Text::new(ctx, "Testing", &font).unwrap(),
        }
    }
}

impl GameState for PlayState{
    fn update(&mut self, keys: &mut HashMap<Keycode, bool>, ctx: &mut Context, _: Duration) -> GameResult<(usize)> {
        for (key, value) in keys.drain() {
            if value {
                return Ok(0);
            }
        }
        Ok(1)
    }
    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult<()> {
        let dest_point = graphics::Point::new(200.0, 50.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        Ok(())
    }
}
