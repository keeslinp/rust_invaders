use std::collections::HashMap;
use std::time::Duration;
use ggez::{GameResult, Context};
use ggez::event::Keycode;
use main_state::MainState;
pub mod menu_state;
pub mod play_state;
mod menu_items;
mod entities;
mod components;

pub trait GameState {
    fn update(&mut self, &mut HashMap<Keycode, bool>, &mut Context, Duration) -> GameResult<(usize)>;
    fn draw(&self, state: &MainState, ctx: &mut Context) -> GameResult<()>;
}
